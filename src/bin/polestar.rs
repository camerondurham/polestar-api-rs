//! Command-line interface for inspecting account vehicles and telemetry.

use clap::{Parser, Subcommand};
use polestar_api::auth::AuthState;
use polestar_api::models::telemetry::Telemetry;
use polestar_api::models::vehicle::Vehicle;
use polestar_api::{PolestarClient, PolestarError};
use serde_json::Value;
use std::error::Error;
use std::io;

#[derive(Parser)]
#[command(name = "polestar", version, about = "Read Polestar vehicle telemetry")]
struct Cli {
    /// Polestar ID email; prefer the POLESTAR_USERNAME environment variable.
    #[arg(long, env = "POLESTAR_USERNAME", global = true, hide_env_values = true)]
    username: Option<String>,

    /// Use imperial units for telemetry distance fields.
    #[arg(long, global = true)]
    imperial: bool,

    /// Vehicle VIN. If omitted, the only vehicle in the account is selected.
    #[arg(long, env = "POLESTAR_VIN", global = true, hide_env_values = true)]
    vin: Option<String>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Verify the public authentication service and local configuration.
    Doctor,
    /// List vehicles associated with the Polestar account.
    Vehicles {
        /// Emit the complete response as JSON.
        #[arg(long)]
        json: bool,

        /// Fetch richer vehicle details where available.
        #[arg(long)]
        verbose: bool,

        /// Fail if verbose mode probes are unsupported; this also enables verbose probing.
        #[arg(long)]
        strict_verbose: bool,
    },
    /// Fetch current battery, odometer, and health telemetry (default).
    Telemetry {
        /// Emit the complete response as JSON.
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    match cli
        .command
        .as_ref()
        .unwrap_or(&Command::Telemetry { json: false })
    {
        Command::Doctor => doctor(&cli).await?,
        Command::Vehicles {
            json,
            verbose,
            strict_verbose,
        } => {
            let client = client_from_cli(&cli)?;
            let want_verbose = *verbose || *strict_verbose;
            let vehicles = if want_verbose {
                if *strict_verbose {
                    client.get_vehicles_verbose().await?
                } else {
                    match client.get_vehicles_verbose().await {
                        Ok(vehicles) => vehicles,
                        Err(err) if verbose_fields_unsupported(&err) => {
                            eprintln!(
                            "Verbose vehicle fields are not available from this API response. Falling back to basic vehicles output."
                        );
                            client.get_vehicles().await?
                        }
                        Err(err) => return Err(err.into()),
                    }
                }
            } else {
                client.get_vehicles().await?
            };
            print_vehicles(&vehicles, *json)?;
        }
        Command::Telemetry { json } => {
            let client = client_from_cli(&cli)?;
            let vin = resolve_vin(&client, cli.vin.as_deref()).await?;
            let telemetry = client.get_telemetry(&vin).await?;
            print_telemetry(&telemetry, *json, cli.imperial)?;
        }
    }

    Ok(())
}

fn client_from_cli(cli: &Cli) -> Result<PolestarClient, Box<dyn Error>> {
    let username = cli.username.clone().ok_or_else(|| {
        io::Error::other(
            "POLESTAR_USERNAME is missing; copy .env.example to .env and add your Polestar ID email",
        )
    })?;
    let password = std::env::var("POLESTAR_PASSWORD").map_err(|_| {
        io::Error::other(
            "POLESTAR_PASSWORD is missing; copy .env.example to .env and add your Polestar ID password",
        )
    })?;

    Ok(PolestarClient::new(username, password)?)
}

async fn doctor(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let auth = AuthState::new(String::new(), String::new());
    let oidc = auth.get_oidc_config(&http_client).await?;

    println!("Polestar auth service: reachable ({})", oidc.issuer);
    println!("POLESTAR_USERNAME: {}", configured(cli.username.as_deref()));
    println!(
        "POLESTAR_PASSWORD: {}",
        configured(std::env::var("POLESTAR_PASSWORD").ok().as_deref())
    );
    println!("POLESTAR_VIN: {}", configured(cli.vin.as_deref()));
    println!("VIN is optional when the account contains exactly one vehicle.");

    Ok(())
}

fn configured(value: Option<&str>) -> &'static str {
    if value.is_some_and(|value| !value.trim().is_empty()) {
        "configured"
    } else {
        "missing"
    }
}

async fn resolve_vin(
    client: &PolestarClient,
    configured_vin: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    if let Some(vin) = configured_vin.filter(|vin| !vin.trim().is_empty()) {
        return Ok(vin.trim().to_ascii_uppercase());
    }

    let vehicles = client.get_vehicles().await?;
    match vehicles.as_slice() {
        [vehicle] => Ok(vehicle.vin.clone()),
        [] => Err(io::Error::other("No vehicles were returned for this Polestar account").into()),
        _ => {
            eprintln!("Multiple vehicles are associated with this account:");
            for vehicle in &vehicles {
                eprintln!("  {}  {}", masked_vin(&vehicle.vin), display_model(vehicle));
            }
            Err(io::Error::other(
                "Set POLESTAR_VIN to select a vehicle (run `polestar vehicles --json` to see full VINs)",
            )
            .into())
        }
    }
}

fn print_vehicles(vehicles: &[Vehicle], json: bool) -> Result<(), Box<dyn Error>> {
    if json {
        println!("{}", serde_json::to_string_pretty(vehicles)?);
        return Ok(());
    }

    if vehicles.is_empty() {
        println!("No vehicles returned.");
    }
    for vehicle in vehicles {
        println!(
            "{}  {}{}",
            masked_vin(&vehicle.vin),
            display_model(vehicle),
            vehicle
                .model_year
                .as_deref()
                .map(|year| format!(" ({year})"))
                .unwrap_or_default()
        );
    }

    Ok(())
}

fn print_telemetry(
    telemetry: &Telemetry,
    json: bool,
    imperial: bool,
) -> Result<(), Box<dyn Error>> {
    if json {
        if imperial {
            println!(
                "{}",
                serde_json::to_string_pretty(&telemetry_to_imperial_json(telemetry)?)?
            );
        } else {
            println!("{}", serde_json::to_string_pretty(telemetry)?);
        }
        return Ok(());
    }

    let range_unit = if imperial { " mi" } else { " km" };
    let service_unit = if imperial { " mi" } else { " km" };
    let odometer_unit = if imperial { " mi" } else { " km" };

    println!("Battery");
    if let Some(battery) = &telemetry.battery {
        print_optional("  Charge", battery.charge_level_percentage, "%");
        if let Some(status) = &battery.charge_status {
            println!("  Status: {status}");
        }
        print_optional(
            "  Time to full",
            battery.estimated_charging_time_minutes,
            " min",
        );
        let estimated_range = if imperial {
            battery
                .estimated_distance_to_empty_miles
                .map(|miles| miles as f64)
                .or_else(|| {
                    battery
                        .estimated_distance_to_empty_km
                        .map(|kilometers| kilometers_to_miles(kilometers as f64))
                })
        } else {
            battery
                .estimated_distance_to_empty_km
                .map(|kilometers| kilometers as f64)
        };
        print_optional_float("  Estimated range", estimated_range, range_unit);
    } else {
        println!("  No sample returned");
    }

    println!("Odometer");
    if let Some(meters) = telemetry
        .odometer
        .as_ref()
        .and_then(|odometer| odometer.odometer_meters)
    {
        let distance = if imperial {
            meters_to_miles(meters as f64)
        } else {
            meters as f64 / 1000.0
        };
        println!("  {:.1}{odometer_unit}", distance);
    } else {
        println!("  No sample returned");
    }

    println!("Health");
    if let Some(health) = &telemetry.health {
        if let Some(warning) = &health.service_warning {
            println!("  Service: {warning}");
        }
        print_optional("  Days to service", health.days_to_service, "");
        let service_distance = if imperial {
            health
                .distance_to_service_km
                .map(|kilometers| kilometers_to_miles(kilometers as f64))
        } else {
            health
                .distance_to_service_km
                .map(|kilometers| kilometers as f64)
        };
        print_optional_float("  Distance to service", service_distance, service_unit);
    } else {
        println!("  No sample returned");
    }

    Ok(())
}

fn print_optional_float(label: &str, value: Option<f64>, suffix: &str) {
    if let Some(value) = value {
        println!("{label}: {value:.1}{suffix}");
    }
}

fn print_optional(label: &str, value: Option<i64>, suffix: &str) {
    if let Some(value) = value {
        println!("{label}: {value}{suffix}");
    }
}

fn kilometers_to_miles(km: f64) -> f64 {
    km * 0.621_371_192_f64
}

fn meters_to_miles(meters: f64) -> f64 {
    meters / 1609.344_f64
}

fn value_to_f64(value: &Value) -> Option<f64> {
    value.as_f64()
}

fn telemetry_to_imperial_json(telemetry: &Telemetry) -> Result<Value, serde_json::Error> {
    let mut telemetry = serde_json::to_value(telemetry)?;
    if let Some(battery) = telemetry.get_mut("battery").and_then(Value::as_object_mut) {
        if let Some(miles) = battery
            .get("estimatedDistanceToEmptyMiles")
            .and_then(value_to_f64)
        {
            battery.insert("estimatedDistanceToEmptyMiles".into(), Value::from(miles));
        } else if let Some(km) = battery
            .get("estimatedDistanceToEmptyKm")
            .and_then(value_to_f64)
        {
            battery.insert(
                "estimatedDistanceToEmptyMiles".into(),
                Value::from(kilometers_to_miles(km)),
            );
        }
    }

    if let Some(health) = telemetry.get_mut("health").and_then(Value::as_object_mut) {
        if let Some(km) = health.get("distanceToServiceKm").and_then(value_to_f64) {
            health.insert(
                "distanceToServiceMiles".into(),
                Value::from(kilometers_to_miles(km)),
            );
        }
    }

    if let Some(odometer) = telemetry.get_mut("odometer").and_then(Value::as_object_mut) {
        if let Some(meters) = odometer.get("odometerMeters").and_then(value_to_f64) {
            odometer.insert("odometerMiles".into(), Value::from(meters_to_miles(meters)));
        }
    }

    if let Some(obj) = telemetry.as_object_mut() {
        obj.insert("distanceUnits".into(), Value::from("imperial"));
    }

    Ok(telemetry)
}

fn verbose_fields_unsupported(error: &PolestarError) -> bool {
    error.is_verbose_probe_error()
}

fn display_model(vehicle: &Vehicle) -> &str {
    vehicle
        .model_name
        .as_deref()
        .or(vehicle.content.model.name.as_deref())
        .unwrap_or("unknown model")
}

fn masked_vin(vin: &str) -> String {
    let suffix = vin.get(vin.len().saturating_sub(4)..).unwrap_or(vin);
    format!("*************{suffix}")
}
