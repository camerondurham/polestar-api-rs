//! GraphQL query strings for the Polestar API endpoints.

/// GraphQL query for fetching vehicle telemetry data.
///
/// Returns battery status, charging information, odometer, and health data.
pub const CAR_TELEMETRICS_V2: &str = r#"
query CarTelematicsV2($vins: [String!]!) {
    carTelematicsV2(vins: $vins) {
        battery {
            vin
            timestamp {
                seconds
                nanos
            }
            batteryChargeLevelPercentage
            chargingStatus
            estimatedChargingTimeToFullMinutes
            estimatedDistanceToEmptyKm
            estimatedDistanceToEmptyMiles
        }
        health {
            vin
            timestamp {
                seconds
                nanos
            }
            daysToService
            distanceToServiceKm
            serviceWarning
            brakeFluidLevelWarning
            engineCoolantLevelWarning
            oilLevelWarning
        }
        odometer {
            vin
            timestamp {
                seconds
                nanos
            }
            odometerMeters
        }
    }
}
"#;

/// GraphQL query for fetching complete vehicle consumer data.
///
/// Returns vehicle specifications, features, images, and configuration.
pub const GET_CONSUMER_CARS_V2: &str = r#"
query GetConsumerCarsV2 {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        market
        content {
            model {
                code
                name
            }
            specification {
                battery
                torque
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
        assert!(CAR_TELEMETRICS_V2.contains("$vins: [String!]!"));
        assert!(CAR_TELEMETRICS_V2.contains("carTelematicsV2"));
        assert!(CAR_TELEMETRICS_V2.contains("batteryChargeLevelPercentage"));
        assert!(CAR_TELEMETRICS_V2.contains("chargingStatus"));
    }

    #[test]
    fn test_vehicle_query_structure() {
        assert!(GET_CONSUMER_CARS_V2.contains("GetConsumerCarsV2"));
        assert!(GET_CONSUMER_CARS_V2.contains("getConsumerCarsV2"));
        assert!(GET_CONSUMER_CARS_V2.contains("content"));
    }
}
