//! Example of fetching vehicle information.
//!
//! Usage:
//! ```bash
//! export POLESTAR_USERNAME="your_email@example.com"
//! export POLESTAR_PASSWORD="your_password"
//! export POLESTAR_VIN="your_vin"
//! cargo run --example vehicle_info
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

    println!("Fetching vehicle information for VIN: {}", vin);

    // Fetch vehicle data
    match client.get_vehicle(&vin).await {
        Ok(vehicle) => {
            println!("\n=== Vehicle Information ===");
            println!("  VIN: {}", vehicle.vin);

            if let Some(reg) = &vehicle.registration_number {
                println!("  Registration: {}", reg);
            }
            if let Some(market) = &vehicle.market {
                println!("  Market: {}", market);
            }

            println!("\n=== Model ===");
            if let Some(name) = &vehicle.content.model.name {
                println!("  Name: {}", name);
            }
            if let Some(code) = &vehicle.content.model.code {
                println!("  Code: {}", code);
            }

            if let Some(spec) = &vehicle.content.specification {
                if let Some(motor) = &spec.motor {
                    println!("\n=== Motor Specifications ===");
                    if let Some(power) = &motor.power {
                        println!("  Power: {}", power);
                    }
                    if let Some(torque) = &motor.torque {
                        println!("  Torque: {}", torque);
                    }
                    if let Some(accel) = &motor.acceleration {
                        println!("  0-100 km/h: {}", accel);
                    }
                }

                if let Some(battery) = &spec.battery {
                    println!("\n=== Battery ===");
                    println!("  {}", battery);
                }

                if let Some(torque) = &spec.torque {
                    println!("\n=== Torque ===");
                    println!("  {}", torque);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching vehicle info: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
