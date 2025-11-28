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

/// GraphQL query for fetching verbose vehicle consumer data.
///
/// Returns extended vehicle information including service history, emissions data,
/// detailed specifications, features, and all available metadata.
pub const GET_CONSUMER_CARS_V2_VERBOSE: &str = r#"
query GetConsumerCarsV2 {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        salesType
        currentPlannedDeliveryDate
        market
        originalMarket
        pno34
        modelYear
        registrationNo
        metaOrderNumber
        factoryCompleteDate
        registrationDate
        deliveryDate
        serviceHistory {
            claimType
            market
            mileage
            mileageUnit
            operations {
                id
                code
                description
                quantity
                performedDate
            }
            orderEndDate
            orderNumber
            orderStartDate
            parts {
                id
                code
                description
                quantity
                performedDate
            }
            statusDMS
            symptomCode
            vehicleAge
            workshopId
        }
        content {
            exterior {
                code
                name
                description
                excluded
            }
            exteriorDetails {
                code
                name
                description
                excluded
            }
            interior {
                code
                name
                description
                excluded
            }
            performancePackage {
                code
                name
                description
                excluded
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
            wheels {
                code
                name
                description
                excluded
            }
            plusPackage {
                code
                name
                description
                excluded
            }
            pilotPackage {
                code
                name
                description
                excluded
            }
            motor {
                name
                description
                excluded
            }
            model {
                name
                code
            }
            specification {
                battery
                bodyType
                brakes
                combustionEngine
                electricMotors
                performance
                suspension
                tireSizes
                torque
                totalHp
                totalKw
                trunkCapacity {
                    label
                    value
                }
            }
            dimensions {
                wheelbase {
                    label
                    value
                }
                groundClearanceWithPerformance {
                    label
                    value
                }
                groundClearanceWithoutPerformance {
                    label
                    value
                }
                dimensions {
                    label
                    value
                }
            }
            towbar {
                code
                name
                description
                excluded
            }
        }
        primaryDriver
        primaryDriverRegistrationTimestamp
        owners {
            id
            registeredAt
            information {
                polestarId
                ownerType
            }
        }
        wltpNedcData {
            wltpCO2Unit
            wltpElecEnergyConsumption
            wltpElecEnergyUnit
            wltpElecRange
            wltpElecRangeUnit
            wltpWeightedCombinedCO2
            wltpWeightedCombinedFuelConsumption
            wltpWeightedCombinedFuelConsumptionUnit
        }
        energy {
            elecRange
            elecRangeUnit
            elecEnergyConsumption
            elecEnergyUnit
            weightedCombinedCO2
            weightedCombinedCO2Unit
            weightedCombinedFuelConsumption
            weightedCombinedFuelConsumptionUnit
        }
        fuelType
        drivetrain
        numberOfDoors
        numberOfSeats
        motor {
            description
            code
        }
        maxTrailerWeight {
            value
            unit
        }
        curbWeight {
            value
            unit
        }
        hasPerformancePackage
        numberOfCylinders
        cylinderVolume
        cylinderVolumeUnit
        transmission
        numberOfGears
        structureWeek
        software {
            version
            versionTimestamp
            performanceOptimization {
                value
                description
                timestamp
            }
        }
        latestClaimStatus {
            mileage
            mileageUnit
            registeredDate
            vehicleAge
        }
        edition
        commonStatusPoint {
            code
            timestamp
            description
        }
        brandStatus {
            code
            timestamp
            description
        }
        intermediateDestinationCode
        partnerDestinationCode
        features {
            type
            code
            name
            description
            excluded
            galleryImage {
                url
                alt
            }
            thumbnail {
                url
                alt
            }
        }
        electricalEngineNumbers {
            number
            placement
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
        assert!(!GET_CONSUMER_CARS_V2_VERBOSE.is_empty());
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

    #[test]
    fn test_verbose_vehicle_query_structure() {
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("GetConsumerCarsV2"));
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("getConsumerCarsV2"));
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("serviceHistory"));
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("wltpNedcData"));
        assert!(GET_CONSUMER_CARS_V2_VERBOSE.contains("features"));
    }
}
