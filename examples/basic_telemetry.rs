//! Basic example of fetching vehicle telemetry.
//!
//! Usage:
//! ```bash
//! export POLESTAR_TOKEN="your_token"
//! export POLESTAR_VIN="your_vin"
//! cargo run --example basic_telemetry
//! ```

use polestar_api_rs::PolestarClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment
    let token = env::var("POLESTAR_TOKEN").expect("POLESTAR_TOKEN must be set");
    let vin = env::var("POLESTAR_VIN").expect("POLESTAR_VIN must be set");

    // Create client
    let client = PolestarClient::new(token)?;

    println!("Fetching telemetry for VIN: {}", vin);

    // Fetch telemetry
    match client.get_telemetry(&vin).await {
        Ok(telemetry) => {
            println!("\n=== Battery Information ===");
            if let Some(charge) = telemetry.battery.charge_level_percentage {
                println!("  Charge Level: {:.1}%", charge);
            }
            if let Some(status) = &telemetry.battery.charge_status {
                println!("  Status: {}", status);
            }
            if let Some(power) = telemetry.battery.charging_power_watts {
                println!("  Charging Power: {:.0} W", power);
            }
            if let Some(time) = telemetry.battery.estimated_charging_time_minutes {
                println!("  Time to Full: {} minutes", time);
            }
            if let Some(range) = telemetry.battery.estimated_distance_to_empty_km {
                println!("  Estimated Range: {:.1} km", range);
            }

            println!("\n=== Odometer ===");
            if let Some(odo) = telemetry.odometer.odometer_meters {
                println!("  Total Distance: {:.1} km", odo as f64 / 1000.0);
            }
            if let Some(speed) = telemetry.odometer.average_speed_kmh {
                println!("  Average Speed: {:.1} km/h", speed);
            }

            println!("\n=== Health ===");
            if let Some(warning) = &telemetry.health.service_warning_status {
                println!("  Service Warning: {}", warning);
            }
        }
        Err(e) => {
            eprintln!("Error fetching telemetry: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
