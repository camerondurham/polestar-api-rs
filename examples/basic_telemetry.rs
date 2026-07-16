//! Basic example of fetching vehicle telemetry.
//!
//! Usage:
//! ```bash
//! export POLESTAR_USERNAME="your_email@example.com"
//! export POLESTAR_PASSWORD="your_password"
//! export POLESTAR_VIN="your_vin"
//! cargo run --example basic_telemetry
//! ```

use polestar_api::PolestarClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment
    let username = env::var("POLESTAR_USERNAME").expect("POLESTAR_USERNAME must be set");
    let password = env::var("POLESTAR_PASSWORD").expect("POLESTAR_PASSWORD must be set");
    let vin = env::var("POLESTAR_VIN").expect("POLESTAR_VIN must be set");

    // Create client
    let client = PolestarClient::new(username, password)?;

    println!("Fetching telemetry for VIN: {}", vin);

    // Fetch telemetry
    match client.get_telemetry(&vin).await {
        Ok(telemetry) => {
            println!("\n=== Battery Information ===");
            if let Some(battery) = &telemetry.battery {
                if let Some(charge) = battery.charge_level_percentage {
                    println!("  Charge Level: {}%", charge);
                }
                if let Some(status) = &battery.charge_status {
                    println!("  Status: {}", status);
                }
                if let Some(time) = battery.estimated_charging_time_minutes {
                    println!("  Time to Full: {} minutes", time);
                }
                if let Some(range) = battery.estimated_distance_to_empty_km {
                    println!("  Estimated Range: {} km", range);
                }
            } else {
                println!("  No battery sample returned");
            }

            println!("\n=== Odometer ===");
            if let Some(ref odo) = telemetry.odometer {
                if let Some(meters) = odo.odometer_meters {
                    println!("  Total Distance: {:.1} km", meters as f64 / 1000.0);
                }
            }

            println!("\n=== Health ===");
            if let Some(health) = &telemetry.health {
                if let Some(warning) = &health.service_warning {
                    println!("  Service Warning: {}", warning);
                }
            } else {
                println!("  No health sample returned");
            }
        }
        Err(e) => {
            eprintln!("Error fetching telemetry: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
