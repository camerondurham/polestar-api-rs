//! Telemetry data models.

use serde::{Deserialize, Serialize};

/// Complete telemetry response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    /// Battery and charging information.
    #[serde(flatten)]
    pub battery: Battery,

    /// Odometer and speed information.
    #[serde(flatten)]
    pub odometer: Odometer,

    /// Vehicle health and status.
    #[serde(flatten)]
    pub health: Health,

    /// Event timestamp.
    #[serde(rename = "eventUpdatedTimestamp")]
    pub event_updated_timestamp: Option<Timestamp>,
}

/// Battery and charging data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    /// Battery charge level as a percentage (0-100).
    #[serde(rename = "batteryChargeLevelPercentage")]
    pub charge_level_percentage: Option<f64>,

    /// Current charging status (e.g., "charging", "idle").
    #[serde(rename = "batteryChargeStatus")]
    pub charge_status: Option<String>,

    /// Current charging power in watts.
    #[serde(rename = "chargingPowerWatts")]
    pub charging_power_watts: Option<f64>,

    /// Estimated time to full charge in minutes.
    #[serde(rename = "estimatedChargingTimeToFullMinutes")]
    pub estimated_charging_time_minutes: Option<i64>,

    /// Estimated distance to empty in kilometers.
    #[serde(rename = "estimatedDistanceToEmptyKm")]
    pub estimated_distance_to_empty_km: Option<f64>,
}

/// Odometer and driving statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Odometer {
    /// Average speed in km/h.
    #[serde(rename = "averageSpeedKmPerHour")]
    pub average_speed_kmh: Option<f64>,

    /// Total distance traveled in meters.
    #[serde(rename = "odometerMeters")]
    pub odometer_meters: Option<i64>,
}

/// Vehicle health and status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    /// Service warning status.
    #[serde(rename = "serviceWarningStatus")]
    pub service_warning_status: Option<String>,

    /// Internal vehicle identifier.
    #[serde(rename = "internalVehicleIdentifier")]
    pub internal_vehicle_identifier: Option<String>,
}

/// Timestamp in both ISO and Unix formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    /// ISO 8601 formatted timestamp.
    pub iso: Option<String>,

    /// Unix timestamp (seconds since epoch).
    pub unix: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_deserialization() {
        let json = r#"{
            "batteryChargeLevelPercentage": 85.5,
            "batteryChargeStatus": "charging"
        }"#;

        let battery: Battery = serde_json::from_str(json).unwrap();
        assert_eq!(battery.charge_level_percentage, Some(85.5));
        assert_eq!(battery.charge_status, Some("charging".to_string()));
    }

    #[test]
    fn test_battery_null_fields() {
        let json = r#"{
            "batteryChargeLevelPercentage": null
        }"#;

        let battery: Battery = serde_json::from_str(json).unwrap();
        assert_eq!(battery.charge_level_percentage, None);
    }
}
