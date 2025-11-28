//! Example: Fetch verbose vehicle information
//!
//! This example demonstrates how to retrieve extended vehicle data including
//! service history, emissions data, features, and detailed specifications.
//!
//! Usage:
//! ```bash
//! cargo run --example verbose_vehicle_info
//! ```

use polestar_api::PolestarClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Get credentials from environment
    let username = env::var("POLESTAR_USERNAME")
        .expect("POLESTAR_USERNAME must be set");
    let password = env::var("POLESTAR_PASSWORD")
        .expect("POLESTAR_PASSWORD must be set");
    let vin = env::var("POLESTAR_VIN")
        .expect("POLESTAR_VIN must be set");

    println!("Polestar API - Verbose Vehicle Information Example");
    println!("==================================================\n");

    // Create client
    println!("Authenticating with Polestar API...");
    let client = PolestarClient::new(&username, &password)?;

    // Fetch verbose vehicle information
    println!("Fetching verbose vehicle information for VIN: {}\n", vin);
    let vehicle = client.get_vehicle_verbose(&vin).await?;

    // Display basic information
    println!("=== BASIC INFORMATION ===");
    println!("VIN: {}", vehicle.vin);
    if let Some(market) = &vehicle.market {
        println!("Market: {}", market);
    }
    if let Some(model_year) = &vehicle.model_year {
        println!("Model Year: {}", model_year);
    }
    println!("Model: {}", vehicle.content.model.name.as_ref().unwrap_or(&"N/A".to_string()));
    if let Some(model_code) = &vehicle.content.model.code {
        println!("Model Code: {}", model_code);
    }
    println!();

    // Display dates
    println!("=== IMPORTANT DATES ===");
    if let Some(factory_date) = &vehicle.factory_complete_date {
        println!("Factory Complete: {}", factory_date);
    }
    if let Some(delivery_date) = &vehicle.delivery_date {
        println!("Delivery Date: {}", delivery_date);
    }
    if let Some(reg_date) = &vehicle.registration_date {
        println!("Registration Date: {}", reg_date);
    }
    println!();

    // Display specifications
    println!("=== SPECIFICATIONS ===");
    if let Some(spec) = &vehicle.content.specification {
        if let Some(battery) = &spec.battery {
            println!("Battery: {}", battery);
        }
        if let Some(torque) = &spec.torque {
            println!("Torque: {}", torque);
        }
        if let Some(total_hp) = &spec.total_hp {
            println!("Total HP: {}", total_hp);
        }
        if let Some(total_kw) = &spec.total_kw {
            println!("Total kW: {}", total_kw);
        }
    }
    println!();

    // Display performance
    if let Some(perf) = &vehicle.content.performance_optimization_specification {
        println!("=== PERFORMANCE OPTIMIZATION ===");
        if let Some(power) = &perf.power {
            if let (Some(value), Some(unit)) = (&power.value, &power.unit) {
                println!("Power: {} {}", value, unit);
            }
        }
        if let Some(torque) = &perf.torque_max {
            if let (Some(value), Some(unit)) = (&torque.value, &torque.unit) {
                println!("Max Torque: {} {}", value, unit);
            }
        }
        if let Some(accel) = &perf.acceleration {
            if let Some(value) = &accel.value {
                println!("0-100: {} {}", value, accel.unit.as_ref().unwrap_or(&"".to_string()));
            }
        }
        println!();
    }

    // Display packages
    println!("=== PACKAGES & OPTIONS ===");
    if let Some(exterior) = &vehicle.content.exterior {
        if let Some(name) = &exterior.name {
            println!("Exterior: {}", name);
        }
    }
    if let Some(interior) = &vehicle.content.interior {
        if let Some(name) = &interior.name {
            println!("Interior: {}", name);
        }
    }
    if let Some(wheels) = &vehicle.content.wheels {
        if let Some(name) = &wheels.name {
            println!("Wheels: {}", name);
        }
    }
    if let Some(plus) = &vehicle.content.plus_package {
        if let Some(name) = &plus.name {
            println!("Plus Package: {}", name);
        }
    }
    if let Some(pilot) = &vehicle.content.pilot_package {
        if let Some(name) = &pilot.name {
            println!("Pilot Package: {}", name);
        }
    }
    println!();

    // Display emissions data
    if let Some(wltp) = &vehicle.wltp_nedc_data {
        println!("=== WLTP/NEDC EMISSIONS DATA ===");
        if let Some(range) = &wltp.wltp_elec_range {
            println!("Electric Range (WLTP): {} {}", range, wltp.wltp_elec_range_unit.as_ref().unwrap_or(&"".to_string()));
        }
        if let Some(consumption) = &wltp.wltp_elec_energy_consumption {
            println!("Energy Consumption (WLTP): {} {}", consumption, wltp.wltp_elec_energy_unit.as_ref().unwrap_or(&"".to_string()));
        }
        if let Some(co2) = &wltp.wltp_weighted_combined_co2 {
            println!("CO2 Emissions (WLTP): {} {}", co2, wltp.wltp_co2_unit.as_ref().unwrap_or(&"".to_string()));
        }
        println!();
    }

    // Display energy data
    if let Some(energy) = &vehicle.energy {
        println!("=== ENERGY DATA ===");
        if let Some(range) = &energy.elec_range {
            println!("Electric Range: {} {}", range, energy.elec_range_unit.as_ref().unwrap_or(&"".to_string()));
        }
        if let Some(consumption) = &energy.elec_energy_consumption {
            println!("Energy Consumption: {} {}", consumption, energy.elec_energy_unit.as_ref().unwrap_or(&"".to_string()));
        }
        println!();
    }

    // Display vehicle details
    if let Some(fuel_type) = &vehicle.fuel_type {
        println!("=== VEHICLE DETAILS ===");
        println!("Fuel Type: {}", fuel_type);
        if let Some(drivetrain) = &vehicle.drivetrain {
            println!("Drivetrain: {}", drivetrain);
        }
        if let Some(doors) = vehicle.number_of_doors {
            println!("Doors: {}", doors);
        }
        if let Some(seats) = vehicle.number_of_seats {
            println!("Seats: {}", seats);
        }
        if let Some(transmission) = &vehicle.transmission {
            println!("Transmission: {}", transmission);
        }
        println!();
    }

    // Display weights
    if let (Some(curb), Some(trailer)) = (&vehicle.curb_weight, &vehicle.max_trailer_weight) {
        println!("=== WEIGHTS & CAPACITIES ===");
        if let (Some(value), Some(unit)) = (&curb.value, &curb.unit) {
            println!("Curb Weight: {} {}", value, unit);
        }
        if let (Some(value), Some(unit)) = (&trailer.value, &trailer.unit) {
            println!("Max Trailer Weight: {} {}", value, unit);
        }
        println!();
    }

    // Display service history
    if let Some(history) = &vehicle.service_history {
        println!("=== SERVICE HISTORY ===");
        println!("Total Service Records: {}", history.len());
        for (i, record) in history.iter().enumerate() {
            println!("\nService Record {}:", i + 1);
            if let Some(order_num) = &record.order_number {
                println!("  Order Number: {}", order_num);
            }
            if let Some(mileage) = record.mileage {
                println!("  Mileage: {} {}", mileage, record.mileage_unit.as_ref().unwrap_or(&"".to_string()));
            }
            if let Some(date) = &record.order_start_date {
                println!("  Date: {}", date);
            }
            if let Some(ops) = &record.operations {
                println!("  Operations: {}", ops.len());
            }
            if let Some(parts) = &record.parts {
                println!("  Parts: {}", parts.len());
            }
        }
        println!();
    }

    // Display features
    if let Some(features) = &vehicle.features {
        println!("=== FEATURES ({}) ===", features.len());
        for feature in features {
            if let Some(name) = &feature.name {
                if let Some(feature_type) = &feature.feature_type {
                    println!("  [{}] {}", feature_type, name);
                } else {
                    println!("  {}", name);
                }
            }
        }
        println!();
    }

    // Display software info
    if let Some(software) = &vehicle.software {
        println!("=== SOFTWARE ===");
        if let Some(version) = &software.version {
            println!("Version: {}", version);
        }
        if let Some(perf_opt) = &software.performance_optimization {
            if let Some(enabled) = perf_opt.value {
                println!("Performance Optimization: {}", if enabled { "Enabled" } else { "Disabled" });
            }
        }
        println!();
    }

    println!("Successfully retrieved verbose vehicle information!");

    Ok(())
}
