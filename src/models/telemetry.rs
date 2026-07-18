//! Telemetry data models.

use serde::{Deserialize, Serialize};

/// Complete telemetry response from the API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryResponse {
    /// Battery data array.
    #[serde(default)]
    pub battery: Vec<Option<Battery>>,
    /// Health data array.
    #[serde(default)]
    pub health: Vec<Option<Health>>,
    /// Odometer data array.
    #[serde(default)]
    pub odometer: Vec<Option<Odometer>>,
}

/// Flattened telemetry data for single vehicle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    /// Battery and charging information.
    pub battery: Option<Battery>,
    /// Vehicle health and status.
    pub health: Option<Health>,
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
    #[serde(rename = "chargingStatusV2", alias = "chargingStatus")]
    pub charge_status: Option<String>,
    /// Estimated time to full charge in minutes.
    #[serde(rename = "estimatedChargingTimeToFullMinutes")]
    pub estimated_charging_time_minutes: Option<i64>,
    /// Estimated distance to empty in kilometers.
    #[serde(rename = "estimatedDistanceToEmptyKm")]
    pub estimated_distance_to_empty_km: Option<i64>,
    /// Estimated distance to empty in miles.
    #[serde(default, rename = "estimatedDistanceToEmptyMiles")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_current_response_with_null_health() {
        let response: TelemetryResponse = serde_json::from_value(serde_json::json!({
            "health": [null],
            "battery": [{
                "vin": "ABCDEFGHJKLMNPRST1",
                "batteryChargeLevelPercentage": 79,
                "chargingStatusV2": "CHARGING_STATUS_V2_IDLE",
                "estimatedChargingTimeToFullMinutes": 0,
                "estimatedDistanceToEmptyKm": 390,
                "timestamp": { "seconds": "1747822967", "nanos": 996856149 }
            }],
            "odometer": [{
                "vin": "ABCDEFGHJKLMNPRST1",
                "odometerMeters": 11131000,
                "timestamp": { "seconds": "1747765507", "nanos": 288842041 }
            }]
        }))
        .unwrap();

        assert!(response.health[0].is_none());
        assert_eq!(
            response.battery[0]
                .as_ref()
                .and_then(|battery| battery.charge_level_percentage),
            Some(79)
        );
    }

    #[test]
    fn accepts_legacy_charging_status_name() {
        let battery: Battery = serde_json::from_value(serde_json::json!({
            "vin": "ABCDEFGHJKLMNPRST2",
            "batteryChargeLevelPercentage": 68,
            "chargingStatus": "CHARGING_STATUS_IDLE",
            "timestamp": { "seconds": "1738053874", "nanos": 0 }
        }))
        .unwrap();

        assert_eq!(
            battery.charge_status.as_deref(),
            Some("CHARGING_STATUS_IDLE")
        );
    }
}
