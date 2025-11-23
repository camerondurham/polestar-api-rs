//! Telemetry data models.

use serde::{Deserialize, Serialize};

/// Complete telemetry response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryResponse {
    /// Battery data array.
    pub battery: Vec<Battery>,
    /// Health data array.
    pub health: Vec<Health>,
    /// Odometer data array.
    pub odometer: Vec<Option<Odometer>>,
}

/// Flattened telemetry data for single vehicle.
#[derive(Debug, Clone)]
pub struct Telemetry {
    /// Battery and charging information.
    pub battery: Battery,
    /// Vehicle health and status.
    pub health: Health,
    /// Odometer information.
    pub odometer: Option<Odometer>,
}

/// Battery and charging data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    /// VIN.
    pub vin: String,
    /// Timestamp.
    pub timestamp: Timestamp,
    /// Battery charge level as a percentage (0-100).
    #[serde(rename = "batteryChargeLevelPercentage")]
    pub charge_level_percentage: Option<i64>,
    /// Current charging status.
    #[serde(rename = "chargingStatus")]
    pub charge_status: Option<String>,
    /// Estimated time to full charge in minutes.
    #[serde(rename = "estimatedChargingTimeToFullMinutes")]
    pub estimated_charging_time_minutes: Option<i64>,
    /// Estimated distance to empty in kilometers.
    #[serde(rename = "estimatedDistanceToEmptyKm")]
    pub estimated_distance_to_empty_km: Option<i64>,
    /// Estimated distance to empty in miles.
    #[serde(rename = "estimatedDistanceToEmptyMiles")]
    pub estimated_distance_to_empty_miles: Option<i64>,
}

/// Odometer data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Odometer {
    /// VIN.
    pub vin: String,
    /// Timestamp.
    pub timestamp: Timestamp,
    /// Total distance traveled in meters.
    #[serde(rename = "odometerMeters")]
    pub odometer_meters: Option<i64>,
}

/// Vehicle health and status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    /// VIN.
    pub vin: String,
    /// Timestamp.
    pub timestamp: Timestamp,
    /// Days to service.
    #[serde(rename = "daysToService")]
    pub days_to_service: Option<i64>,
    /// Distance to service in km.
    #[serde(rename = "distanceToServiceKm")]
    pub distance_to_service_km: Option<i64>,
    /// Service warning.
    #[serde(rename = "serviceWarning")]
    pub service_warning: Option<String>,
    /// Brake fluid level warning.
    #[serde(rename = "brakeFluidLevelWarning")]
    pub brake_fluid_level_warning: Option<String>,
    /// Engine coolant level warning.
    #[serde(rename = "engineCoolantLevelWarning")]
    pub engine_coolant_level_warning: Option<String>,
    /// Oil level warning.
    #[serde(rename = "oilLevelWarning")]
    pub oil_level_warning: Option<String>,
}

/// Timestamp with seconds and nanoseconds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    /// Seconds since epoch.
    pub seconds: String,
    /// Nanoseconds.
    pub nanos: i64,
}
