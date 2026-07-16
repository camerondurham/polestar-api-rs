//! GraphQL query strings for the Polestar API endpoints.

/// GraphQL query for fetching current vehicle telemetry data.
///
/// The response groups battery, health, and odometer samples by VIN. Individual
/// samples may be `null` when a vehicle or backend does not expose that signal.
pub const CAR_TELEMETRICS_V2: &str = r#"
query CarTelematicsV2($vins: [String!]!) {
    carTelematicsV2(vins: $vins) {
        health {
            vin
            brakeFluidLevelWarning
            daysToService
            distanceToServiceKm
            engineCoolantLevelWarning
            oilLevelWarning
            serviceWarning
            timestamp {
                seconds
                nanos
            }
        }
        battery {
            vin
            chargingStatusV2
            batteryChargeLevelPercentage
            estimatedChargingTimeToFullMinutes
            estimatedDistanceToEmptyKm
            timestamp {
                seconds
                nanos
            }
        }
        odometer {
            vin
            odometerMeters
            timestamp {
                seconds
                nanos
            }
        }
    }
}
"#;

/// GraphQL query for vehicles associated with the authenticated account.
///
/// Polestar has reduced the dependable private vehicle-information surface, so
/// this intentionally requests only fields used by the maintained clients.
pub const GET_CONSUMER_CARS_V2: &str = r#"
query GetConsumerCarsV2 {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelYear
        modelName
        pno34
        structureWeek
    }
}
"#;

/// Compatibility alias for callers that used the former verbose query.
///
/// The upstream API no longer offers a stable verbose vehicle-information
/// contract; this alias now returns the supported vehicle summary fields.
pub const GET_CONSUMER_CARS_V2_VERBOSE: &str = GET_CONSUMER_CARS_V2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queries_are_not_empty() {
        assert!(!CAR_TELEMETRICS_V2.is_empty());
        assert!(!GET_CONSUMER_CARS_V2.is_empty());
    }

    #[test]
    fn telemetry_query_uses_current_fields() {
        assert!(CAR_TELEMETRICS_V2.contains("CarTelematicsV2"));
        assert!(CAR_TELEMETRICS_V2.contains("$vins: [String!]!"));
        assert!(CAR_TELEMETRICS_V2.contains("chargingStatusV2"));
        assert!(!CAR_TELEMETRICS_V2.contains("estimatedDistanceToEmptyMiles"));
    }

    #[test]
    fn vehicle_query_stays_on_supported_summary_fields() {
        assert!(GET_CONSUMER_CARS_V2.contains("modelName"));
        assert!(GET_CONSUMER_CARS_V2.contains("structureWeek"));
        assert!(!GET_CONSUMER_CARS_V2.contains("serviceHistory"));
    }
}
