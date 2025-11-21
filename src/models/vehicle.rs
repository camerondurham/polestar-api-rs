//! Vehicle data models.

use serde::{Deserialize, Serialize};

/// Complete vehicle information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    /// Vehicle Identification Number (VIN).
    pub vin: String,

    /// Internal vehicle identifier.
    #[serde(rename = "internalVehicleIdentifier")]
    pub internal_vehicle_identifier: Option<String>,

    /// Registration number.
    #[serde(rename = "registrationNo")]
    pub registration_number: Option<String>,

    /// Market (e.g., "US", "EU").
    pub market: Option<String>,

    /// Vehicle content and specifications.
    pub content: VehicleContent,
}

/// Vehicle content and specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleContent {
    /// Model information.
    pub model: ModelInfo,

    /// Vehicle images.
    pub images: Option<Images>,

    /// Vehicle specifications.
    pub specification: Option<VehicleSpecifications>,
}

/// Model information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model code.
    pub code: Option<String>,

    /// Model name.
    pub name: Option<String>,
}

/// Vehicle images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
    /// Studio images.
    pub studio: Option<StudioImages>,
}

/// Studio images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioImages {
    /// Image URL.
    pub url: Option<String>,

    /// Available angles.
    pub angles: Option<Vec<String>>,
}

/// Vehicle specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSpecifications {
    /// Motor specifications.
    pub motor: Option<MotorSpec>,

    /// Battery specifications.
    pub battery: Option<String>,

    /// Torque specification.
    pub torque: Option<String>,
}

/// Motor specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorSpec {
    /// Power output.
    pub power: Option<String>,

    /// Torque.
    pub torque: Option<String>,

    /// 0-100 km/h acceleration time.
    pub acceleration: Option<String>,
}

/// Battery specifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatterySpec {
    /// Battery capacity.
    pub capacity: Option<String>,

    /// Range.
    pub range: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_deserialization() {
        let json = r#"{
            "vin": "TEST123",
            "market": "US",
            "content": {
                "model": {
                    "code": "P2",
                    "name": "Polestar 2"
                }
            }
        }"#;

        let vehicle: Vehicle = serde_json::from_str(json).unwrap();
        assert_eq!(vehicle.vin, "TEST123");
        assert_eq!(vehicle.market, Some("US".to_string()));
        assert_eq!(vehicle.content.model.name, Some("Polestar 2".to_string()));
    }
}
