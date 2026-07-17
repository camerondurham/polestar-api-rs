//! # polestar-api
//!
//! A lightweight, type-safe Rust wrapper for the Polestar vehicle GraphQL API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use polestar_api::PolestarClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = PolestarClient::new("your_username", "your_password")?;
//!     let telemetry = client.get_telemetry("YOUR_VIN").await?;
//!     if let Some(charge) = telemetry
//!         .battery
//!         .and_then(|battery| battery.charge_level_percentage)
//!     {
//!         println!("Battery: {charge}%");
//!     }
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod auth;
pub mod client;
pub mod error;
pub mod graphql;
pub mod models;
pub mod redact;

// Re-export main types for convenience
pub use client::PolestarClient;
pub use error::{PolestarError, Result};
pub use models::{telemetry::Telemetry, vehicle::Vehicle};
