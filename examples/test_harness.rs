//! CLI test harness for the Polestar API.
//!
//! Usage:
//! ```bash
//! cargo run --example test_harness --features cli -- \
//!   --username "your_email@example.com" \
//!   --password "your_password" \
//!   --vin "YOUR_VIN" \
//!   --endpoint telemetry
//! ```

#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use polestar_api::PolestarClient;
#[cfg(feature = "cli")]
use std::time::Instant;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "polestar-test-harness")]
#[command(about = "Test harness for Polestar API")]
struct Args {
    /// Polestar account username (email)
    #[arg(short, long, env = "POLESTAR_USERNAME")]
    username: String,

    /// Polestar account password
    #[arg(short, long, env = "POLESTAR_PASSWORD")]
    password: String,

    /// Vehicle VIN
    #[arg(short, long, env = "POLESTAR_VIN")]
    vin: String,

    /// Endpoint to test
    #[arg(short, long, value_enum)]
    endpoint: Endpoint,

    /// Pretty-print JSON output
    #[arg(short, long, default_value_t = true)]
    pretty: bool,

    /// Show timing information
    #[arg(long, default_value_t = true)]
    timing: bool,
}

#[cfg(feature = "cli")]
#[derive(clap::ValueEnum, Clone)]
enum Endpoint {
    Telemetry,
    Vehicle,
    All,
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let client = PolestarClient::new(args.username, args.password)?;

    match args.endpoint {
        Endpoint::Telemetry => {
            test_telemetry(&client, &args.vin, args.pretty, args.timing).await?;
        }
        Endpoint::Vehicle => {
            test_vehicle(&client, &args.vin, args.pretty, args.timing).await?;
        }
        Endpoint::All => {
            test_telemetry(&client, &args.vin, args.pretty, args.timing).await?;
            println!("\n---\n");
            test_vehicle(&client, &args.vin, args.pretty, args.timing).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "cli")]
async fn test_telemetry(
    client: &PolestarClient,
    vin: &str,
    pretty: bool,
    timing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Telemetry API...");

    let start = Instant::now();
    let result = client.get_telemetry(vin).await?;
    let elapsed = start.elapsed();

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }

    if timing {
        println!("\nElapsed: {:?}", elapsed);
    }

    println!("\n✓ Telemetry API test passed");
    if let Some(charge) = result.battery.charge_level_percentage {
        println!("  Battery: {}%", charge);
    }

    Ok(())
}

#[cfg(feature = "cli")]
async fn test_vehicle(
    client: &PolestarClient,
    vin: &str,
    pretty: bool,
    timing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Vehicle API...");

    let start = Instant::now();
    let result = client.get_vehicle(vin).await?;
    let elapsed = start.elapsed();

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }

    if timing {
        println!("\nElapsed: {:?}", elapsed);
    }

    println!("\n✓ Vehicle API test passed");
    println!("  VIN: {}", result.vin);

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("This example requires the 'cli' feature to be enabled.");
    eprintln!("Run with: cargo run --example test_harness --features cli");
    std::process::exit(1);
}
