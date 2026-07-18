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

/// Verbose query for richer vehicle details, including performance-upgrade fields.
pub const GET_CONSUMER_CARS_V2_VERBOSE: &str = r#"
query GetConsumerCarsV2Verbose {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        market
        currentPlannedDeliveryDate
        deliveryDate
        pno34
        modelYear
        structureWeek
        hasPerformancePackage
        software {
            performanceOptimization {
                value
            }
        }
        content {
            images {
                studio {
                    url
                    angles
                }
            }
            model {
                code
                name
            }
            performanceOptimizationSpecification {
                power {
                    value
                    unit
                }
                torqueMax {
                    value
                    unit
                }
                acceleration {
                    value
                    unit
                    description
                }
            }
        }
    }
}
"#;

/// Query attempting only top-level performance-package metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE_HAS_PERFORMANCE: &str = r#"
query GetConsumerCarsV2VerboseHasPerformance {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelYear
        modelName
        pno34
        structureWeek
        hasPerformancePackage
    }
}
"#;

/// Query attempting performance-package metadata and software flag.
pub const GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE: &str = r#"
query GetConsumerCarsV2VerboseSoftware {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelYear
        modelName
        pno34
        structureWeek
        hasPerformancePackage
        software {
            performanceOptimization {
                value
            }
        }
    }
}
"#;

/// Query attempting locale-scoped performance metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE_LOCALE: &str = r#"
query GetConsumerCarsV2VerboseLocale {
    getConsumerCarsV2(locale: "en_US") {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        market
        currentPlannedDeliveryDate
        deliveryDate
        pno34
        modelYear
        structureWeek
        hasPerformancePackage
        software {
            performanceOptimization {
                value
            }
        }
        content {
            model {
                code
                name
            }
            performanceOptimizationSpecification {
                power {
                    value
                    unit
                }
                torqueMax {
                    value
                    unit
                }
                acceleration {
                    value
                    unit
                    description
                }
            }
        }
    }
}
"#;

/// Query attempting software and content-only performance metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE_NO_PERFORMANCE: &str = r#"
query GetConsumerCarsV2VerboseNoPerformance {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        market
        pno34
        structureWeek
        software {
            performanceOptimization {
                value
            }
        }
        content {
            model {
                code
                name
            }
            performanceOptimizationSpecification {
                power {
                    value
                    unit
                }
                torqueMax {
                    value
                    unit
                }
                acceleration {
                    value
                    unit
                    description
                }
            }
        }
    }
}
"#;

/// Query attempting only software metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE_ONLY: &str = r#"
query GetConsumerCarsV2VerboseSoftwareOnly {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        structureWeek
        software {
            performanceOptimization {
                value
            }
        }
    }
}
"#;

/// Query attempting scalar software performance optimization field.
pub const GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE_SCALAR: &str = r#"
query GetConsumerCarsV2VerboseSoftwareScalar {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        structureWeek
        software {
            performanceOptimization
        }
    }
}
"#;

/// Query attempting only performance optimization content metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE_CONTENT_ONLY: &str = r#"
query GetConsumerCarsV2VerboseContentOnly {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        structureWeek
        content {
            model {
                code
                name
            }
            performanceOptimizationSpecification {
                power {
                    value
                    unit
                }
                torqueMax {
                    value
                    unit
                }
                acceleration {
                    value
                    unit
                    description
                }
            }
        }
    }
}
"#;

/// Query attempting scalar performance optimization specification metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE_PERFORMANCE_SPEC_SCALAR: &str = r#"
query GetConsumerCarsV2VerbosePerformanceSpecScalar {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        modelName
        structureWeek
        content {
            model {
                code
                name
            }
            performanceOptimizationSpecification
        }
    }
}
"#;

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

    #[test]
    fn verbose_query_includes_performance_fields() {
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("performanceOptimization"));
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("performanceOptimizationSpecification"));
    }

    #[test]
    fn verbose_fallback_queries_include_expected_fields() {
        assert!(GET_CONSUMER_CARS_V2_VERBOSE_HAS_PERFORMANCE.contains("hasPerformancePackage"));
        assert!(
            GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE.contains(
                "software {\n            performanceOptimization {\n                value\n            }\n        }"
            )
        );
    }

    #[test]
    fn verbose_fallback_queries_include_scalar_performance_paths() {
        assert!(GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE_SCALAR.contains("performanceOptimization"));
        assert!(GET_CONSUMER_CARS_V2_VERBOSE_PERFORMANCE_SPEC_SCALAR
            .contains("performanceOptimizationSpecification"));
    }

    #[test]
    fn locale_query_contains_locale_arg() {
        assert!(GET_CONSUMER_CARS_V2_VERBOSE_LOCALE.contains("locale"));
    }
}
