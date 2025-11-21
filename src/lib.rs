//! # polestar-api-rs
//!
//! A lightweight, type-safe Rust wrapper for the Polestar vehicle GraphQL API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use polestar_api_rs::PolestarClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = PolestarClient::new("your_username", "your_password")?;
//!     let telemetry = client.get_telemetry("YOUR_VIN").await?;
//!     println!("Battery: {:.1}%",
//!         telemetry.battery.charge_level_percentage.unwrap_or(0.0));
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

// Re-export main types for convenience
pub use client::PolestarClient;
pub use error::{PolestarError, Result};
pub use models::{telemetry::Telemetry, vehicle::Vehicle};
