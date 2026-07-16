//! Compatibility example for the former verbose vehicle-information API.
//!
//! Polestar no longer exposes a stable verbose vehicle contract. This example
//! remains so existing commands keep working, but it prints the supported
//! account vehicle summary instead.

use polestar_api::PolestarClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let username = env::var("POLESTAR_USERNAME").expect("POLESTAR_USERNAME must be set");
    let password = env::var("POLESTAR_PASSWORD").expect("POLESTAR_PASSWORD must be set");
    let vin = env::var("POLESTAR_VIN").expect("POLESTAR_VIN must be set");

    let client = PolestarClient::new(username, password)?;
    let vehicle = client.get_vehicle_verbose(&vin).await?;

    println!("VIN: {}", vehicle.vin);
    println!(
        "Model: {}",
        vehicle
            .model_name
            .as_deref()
            .or(vehicle.content.model.name.as_deref())
            .unwrap_or("unknown")
    );
    if let Some(model_year) = vehicle.model_year {
        println!("Model year: {model_year}");
    }
    if let Some(registration) = vehicle.registration_number {
        println!("Registration: {registration}");
    }

    Ok(())
}
