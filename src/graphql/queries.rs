//! GraphQL query strings for the Polestar API endpoints.

/// GraphQL query for fetching vehicle telemetry data.
///
/// Returns battery status, charging information, odometer, and health data.
pub const CAR_TELEMETRICS_V2: &str = r#"
query CarTelematicsV2($vin: String!) {
    getCarTelematicsV2(vin: $vin) {
        data {
            batteryChargeLevelPercentage
            batteryChargeStatus
            chargingPowerWatts
            estimatedChargingTimeToFullMinutes
            estimatedDistanceToEmptyKm
            odometerMeters
            averageSpeedKmPerHour
            serviceWarningStatus
            internalVehicleIdentifier
            eventUpdatedTimestamp {
                iso
                unix
            }
        }
    }
}
"#;

/// GraphQL query for fetching complete vehicle consumer data.
///
/// Returns vehicle specifications, features, images, and configuration.
pub const GET_CONSUMER_CARS_V2: &str = r#"
query GetConsumerCarsV2($vin: String!) {
    getConsumerCarsV2(vin: $vin) {
        vin
        internalVehicleIdentifier
        registrationNo
        market
        content {
            model {
                code
                name
            }
            images {
                studio {
                    url
                    angles
                }
            }
            specification {
                motor {
                    power
                    torque
                    acceleration
                }
                battery {
                    capacity
                    range
                }
            }
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queries_not_empty() {
        assert!(!CAR_TELEMETRICS_V2.is_empty());
        assert!(!GET_CONSUMER_CARS_V2.is_empty());
    }

    #[test]
    fn test_telemetry_query_structure() {
        assert!(CAR_TELEMETRICS_V2.contains("CarTelematicsV2"));
        assert!(CAR_TELEMETRICS_V2.contains("$vin: String!"));
        assert!(CAR_TELEMETRICS_V2.contains("batteryChargeLevelPercentage"));
    }

    #[test]
    fn test_vehicle_query_structure() {
        assert!(GET_CONSUMER_CARS_V2.contains("GetConsumerCarsV2"));
        assert!(GET_CONSUMER_CARS_V2.contains("$vin: String!"));
        assert!(GET_CONSUMER_CARS_V2.contains("content"));
    }
}
