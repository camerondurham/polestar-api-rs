//! Vehicle data models.

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

    /// Current display name returned by the supported vehicle-summary query.
    #[serde(rename = "modelName")]
    pub model_name: Option<String>,

    /// Market (e.g., "US", "EU").
    pub market: Option<String>,

    /// Vehicle content and specifications.
    #[serde(default)]
    pub content: VehicleContent,

    // Verbose fields below
    /// Sales type.
    #[serde(rename = "salesType")]
    pub sales_type: Option<String>,

    /// Current planned delivery date.
    #[serde(rename = "currentPlannedDeliveryDate")]
    pub current_planned_delivery_date: Option<String>,

    /// Original market.
    #[serde(rename = "originalMarket")]
    pub original_market: Option<String>,

    /// PNO34 code.
    pub pno34: Option<String>,

    /// Model year.
    #[serde(rename = "modelYear")]
    pub model_year: Option<String>,

    /// Meta order number.
    #[serde(rename = "metaOrderNumber")]
    pub meta_order_number: Option<String>,

    /// Factory complete date.
    #[serde(rename = "factoryCompleteDate")]
    pub factory_complete_date: Option<String>,

    /// Registration date.
    #[serde(rename = "registrationDate")]
    pub registration_date: Option<String>,

    /// Delivery date.
    #[serde(rename = "deliveryDate")]
    pub delivery_date: Option<String>,

    /// Service history.
    #[serde(rename = "serviceHistory")]
    pub service_history: Option<Vec<ServiceRecord>>,

    /// Primary driver ID.
    #[serde(rename = "primaryDriver")]
    pub primary_driver: Option<String>,

    /// Primary driver registration timestamp.
    #[serde(rename = "primaryDriverRegistrationTimestamp")]
    pub primary_driver_registration_timestamp: Option<String>,

    /// Vehicle owners.
    pub owners: Option<Vec<Owner>>,

    /// WLTP/NEDC emissions data.
    #[serde(rename = "wltpNedcData")]
    pub wltp_nedc_data: Option<WltpNedcData>,

    /// Energy consumption data.
    pub energy: Option<EnergyData>,

    /// Fuel type.
    #[serde(rename = "fuelType")]
    pub fuel_type: Option<String>,

    /// Drivetrain type.
    pub drivetrain: Option<String>,

    /// Number of doors.
    #[serde(rename = "numberOfDoors")]
    pub number_of_doors: Option<i32>,

    /// Number of seats.
    #[serde(rename = "numberOfSeats")]
    pub number_of_seats: Option<i32>,

    /// Motor information.
    pub motor: Option<MotorInfo>,

    /// Maximum trailer weight.
    #[serde(rename = "maxTrailerWeight")]
    pub max_trailer_weight: Option<ValueUnit>,

    /// Curb weight.
    #[serde(rename = "curbWeight")]
    pub curb_weight: Option<ValueUnit>,

    /// Has performance package.
    #[serde(rename = "hasPerformancePackage")]
    pub has_performance_package: Option<bool>,

    /// Number of cylinders.
    #[serde(rename = "numberOfCylinders")]
    pub number_of_cylinders: Option<i32>,

    /// Cylinder volume.
    #[serde(rename = "cylinderVolume")]
    pub cylinder_volume: Option<i32>,

    /// Cylinder volume unit.
    #[serde(rename = "cylinderVolumeUnit")]
    pub cylinder_volume_unit: Option<String>,

    /// Transmission type.
    pub transmission: Option<String>,

    /// Number of gears.
    #[serde(rename = "numberOfGears")]
    pub number_of_gears: Option<i32>,

    /// Structure week.
    #[serde(rename = "structureWeek")]
    pub structure_week: Option<String>,

    /// Software information.
    pub software: Option<SoftwareInfo>,

    /// Latest claim status.
    #[serde(rename = "latestClaimStatus")]
    pub latest_claim_status: Option<ClaimStatus>,

    /// Internal car information.
    #[serde(rename = "internalCar")]
    pub internal_car: Option<InternalCar>,

    /// Edition.
    pub edition: Option<String>,

    /// Common status point.
    #[serde(rename = "commonStatusPoint")]
    pub common_status_point: Option<StatusPoint>,

    /// Brand status.
    #[serde(rename = "brandStatus")]
    pub brand_status: Option<StatusPoint>,

    /// Intermediate destination code.
    #[serde(rename = "intermediateDestinationCode")]
    pub intermediate_destination_code: Option<String>,

    /// Partner destination code.
    #[serde(rename = "partnerDestinationCode")]
    pub partner_destination_code: Option<String>,

    /// Vehicle features.
    pub features: Option<Vec<Feature>>,

    /// Electrical engine numbers.
    #[serde(rename = "electricalEngineNumbers")]
    pub electrical_engine_numbers: Option<Vec<ElectricalEngineNumber>>,
}

/// Vehicle content and specifications.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleContent {
    /// Model information.
    pub model: ModelInfo,

    /// Vehicle images.
    pub images: Option<Images>,

    /// Vehicle specifications.
    pub specification: Option<VehicleSpecifications>,

    // Verbose fields
    /// Exterior color/package.
    pub exterior: Option<Package>,

    /// Exterior details.
    #[serde(rename = "exteriorDetails")]
    pub exterior_details: Option<Package>,

    /// Interior package.
    pub interior: Option<Package>,

    /// Performance package.
    #[serde(rename = "performancePackage")]
    pub performance_package: Option<Package>,

    /// Performance optimization specification.
    #[serde(rename = "performanceOptimizationSpecification")]
    pub performance_optimization_specification: Option<PerformanceOptimizationSpecification>,

    /// Wheels package.
    pub wheels: Option<Package>,

    /// Plus package.
    #[serde(rename = "plusPackage")]
    pub plus_package: Option<Package>,

    /// Pilot package.
    #[serde(rename = "pilotPackage")]
    pub pilot_package: Option<Package>,

    /// Motor details.
    pub motor: Option<MotorDetails>,

    /// Vehicle dimensions.
    pub dimensions: Option<Dimensions>,

    /// Towbar package.
    pub towbar: Option<Package>,
}

/// Model information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

    // Verbose fields
    /// Body type.
    #[serde(rename = "bodyType")]
    pub body_type: Option<String>,

    /// Brakes.
    pub brakes: Option<String>,

    /// Combustion engine.
    #[serde(rename = "combustionEngine")]
    pub combustion_engine: Option<String>,

    /// Electric motors.
    #[serde(rename = "electricMotors")]
    pub electric_motors: Option<String>,

    /// Performance.
    pub performance: Option<String>,

    /// Suspension.
    pub suspension: Option<String>,

    /// Tire sizes.
    #[serde(rename = "tireSizes")]
    pub tire_sizes: Option<String>,

    /// Total horsepower.
    #[serde(rename = "totalHp")]
    pub total_hp: Option<String>,

    /// Total kilowatts.
    #[serde(rename = "totalKw")]
    pub total_kw: Option<String>,

    /// Trunk capacity.
    #[serde(rename = "trunkCapacity")]
    pub trunk_capacity: Option<LabelValue>,
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

// Verbose mode structs

/// Service record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRecord {
    /// Claim type.
    #[serde(rename = "claimType")]
    pub claim_type: Option<String>,

    /// Market.
    pub market: Option<String>,

    /// Mileage.
    pub mileage: Option<i32>,

    /// Mileage unit.
    #[serde(rename = "mileageUnit")]
    pub mileage_unit: Option<String>,

    /// Service operations.
    pub operations: Option<Vec<ServiceOperation>>,

    /// Order end date.
    #[serde(rename = "orderEndDate")]
    pub order_end_date: Option<String>,

    /// Order number.
    #[serde(rename = "orderNumber")]
    pub order_number: Option<String>,

    /// Order start date.
    #[serde(rename = "orderStartDate")]
    pub order_start_date: Option<String>,

    /// Service parts.
    pub parts: Option<Vec<ServicePart>>,

    /// Status DMS.
    #[serde(rename = "statusDMS")]
    pub status_dms: Option<String>,

    /// Symptom code.
    #[serde(rename = "symptomCode")]
    pub symptom_code: Option<String>,

    /// Vehicle age.
    #[serde(rename = "vehicleAge")]
    pub vehicle_age: Option<i32>,

    /// Workshop ID.
    #[serde(rename = "workshopId")]
    pub workshop_id: Option<String>,
}

/// Service operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOperation {
    /// Operation ID.
    pub id: Option<String>,

    /// Operation code.
    pub code: Option<String>,

    /// Operation description.
    pub description: Option<String>,

    /// Quantity.
    pub quantity: Option<i32>,

    /// Performed date.
    #[serde(rename = "performedDate")]
    pub performed_date: Option<String>,
}

/// Service part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePart {
    /// Part ID.
    pub id: Option<String>,

    /// Part code.
    pub code: Option<String>,

    /// Part description.
    pub description: Option<String>,

    /// Quantity.
    pub quantity: Option<i32>,

    /// Performed date.
    #[serde(rename = "performedDate")]
    pub performed_date: Option<String>,
}

/// Package information (exterior, interior, wheels, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Package code.
    pub code: Option<String>,

    /// Package name.
    pub name: Option<String>,

    /// Package description.
    pub description: Option<String>,

    /// Is excluded.
    pub excluded: Option<bool>,
}

/// Performance optimization specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PerformanceOptimizationSpecification {
    Known(PerformanceOptimizationSpec),
    Other(Value),
}

/// Legacy structured performance specification payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptimizationSpec {
    /// Power specification.
    pub power: Option<ValueUnit>,

    /// Maximum torque.
    #[serde(rename = "torqueMax")]
    pub torque_max: Option<ValueUnit>,

    /// Acceleration specification.
    pub acceleration: Option<AccelerationSpec>,
}

/// Acceleration specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerationSpec {
    /// Value.
    pub value: Option<String>,

    /// Unit.
    pub unit: Option<String>,

    /// Description.
    pub description: Option<String>,
}

/// Motor details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorDetails {
    /// Motor name.
    pub name: Option<String>,

    /// Motor description.
    pub description: Option<String>,

    /// Is excluded.
    pub excluded: Option<bool>,
}

/// Motor information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorInfo {
    /// Motor description.
    pub description: Option<String>,

    /// Motor code.
    pub code: Option<String>,
}

/// Dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    /// Wheelbase.
    pub wheelbase: Option<LabelValue>,

    /// Ground clearance with performance package.
    #[serde(rename = "groundClearanceWithPerformance")]
    pub ground_clearance_with_performance: Option<LabelValue>,

    /// Ground clearance without performance package.
    #[serde(rename = "groundClearanceWithoutPerformance")]
    pub ground_clearance_without_performance: Option<LabelValue>,

    /// Overall dimensions.
    pub dimensions: Option<LabelValue>,
}

/// Label and value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelValue {
    /// Label.
    pub label: Option<String>,

    /// Value.
    pub value: Option<String>,
}

/// Value and unit pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueUnit {
    /// Value.
    pub value: Option<String>,

    /// Unit.
    pub unit: Option<String>,
}

/// Owner information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    /// Owner ID.
    pub id: Option<String>,

    /// Registered at.
    #[serde(rename = "registeredAt")]
    pub registered_at: Option<String>,

    /// Owner information.
    pub information: Option<OwnerInformation>,
}

/// Owner information details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerInformation {
    /// Polestar ID.
    #[serde(rename = "polestarId")]
    pub polestar_id: Option<String>,

    /// Owner type.
    #[serde(rename = "ownerType")]
    pub owner_type: Option<String>,
}

/// WLTP/NEDC emissions data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WltpNedcData {
    /// WLTP CO2 unit.
    #[serde(rename = "wltpCO2Unit")]
    pub wltp_co2_unit: Option<String>,

    /// WLTP electric energy consumption.
    #[serde(rename = "wltpElecEnergyConsumption")]
    pub wltp_elec_energy_consumption: Option<String>,

    /// WLTP electric energy unit.
    #[serde(rename = "wltpElecEnergyUnit")]
    pub wltp_elec_energy_unit: Option<String>,

    /// WLTP electric range.
    #[serde(rename = "wltpElecRange")]
    pub wltp_elec_range: Option<String>,

    /// WLTP electric range unit.
    #[serde(rename = "wltpElecRangeUnit")]
    pub wltp_elec_range_unit: Option<String>,

    /// WLTP weighted combined CO2.
    #[serde(rename = "wltpWeightedCombinedCO2")]
    pub wltp_weighted_combined_co2: Option<String>,

    /// WLTP weighted combined fuel consumption.
    #[serde(rename = "wltpWeightedCombinedFuelConsumption")]
    pub wltp_weighted_combined_fuel_consumption: Option<String>,

    /// WLTP weighted combined fuel consumption unit.
    #[serde(rename = "wltpWeightedCombinedFuelConsumptionUnit")]
    pub wltp_weighted_combined_fuel_consumption_unit: Option<String>,
}

/// Energy consumption data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyData {
    /// Electric range.
    #[serde(rename = "elecRange")]
    pub elec_range: Option<String>,

    /// Electric range unit.
    #[serde(rename = "elecRangeUnit")]
    pub elec_range_unit: Option<String>,

    /// Electric energy consumption.
    #[serde(rename = "elecEnergyConsumption")]
    pub elec_energy_consumption: Option<String>,

    /// Electric energy unit.
    #[serde(rename = "elecEnergyUnit")]
    pub elec_energy_unit: Option<String>,

    /// Weighted combined CO2.
    #[serde(rename = "weightedCombinedCO2")]
    pub weighted_combined_co2: Option<String>,

    /// Weighted combined CO2 unit.
    #[serde(rename = "weightedCombinedCO2Unit")]
    pub weighted_combined_co2_unit: Option<String>,

    /// Weighted combined fuel consumption.
    #[serde(rename = "weightedCombinedFuelConsumption")]
    pub weighted_combined_fuel_consumption: Option<String>,

    /// Weighted combined fuel consumption unit.
    #[serde(rename = "weightedCombinedFuelConsumptionUnit")]
    pub weighted_combined_fuel_consumption_unit: Option<String>,
}

/// Software information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareInfo {
    /// Software version.
    pub version: Option<String>,

    /// Version timestamp.
    #[serde(rename = "versionTimestamp")]
    pub version_timestamp: Option<String>,

    /// Performance optimization.
    #[serde(rename = "performanceOptimization")]
    pub performance_optimization: Option<PerformanceOptimization>,
}

/// Performance optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceOptimization {
    /// Is enabled.
    pub value: Option<bool>,

    /// Description.
    pub description: Option<String>,

    /// Timestamp.
    pub timestamp: Option<String>,
}

/// Claim status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimStatus {
    /// Mileage.
    pub mileage: Option<i32>,

    /// Mileage unit.
    #[serde(rename = "mileageUnit")]
    pub mileage_unit: Option<String>,

    /// Registered date.
    #[serde(rename = "registeredDate")]
    pub registered_date: Option<String>,

    /// Vehicle age.
    #[serde(rename = "vehicleAge")]
    pub vehicle_age: Option<i32>,
}

/// Internal car information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalCar {
    /// Origin.
    pub origin: Option<String>,

    /// Registered at.
    #[serde(rename = "registeredAt")]
    pub registered_at: Option<String>,
}

/// Status point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPoint {
    /// Status code.
    pub code: Option<String>,

    /// Timestamp.
    pub timestamp: Option<String>,

    /// Description.
    pub description: Option<String>,
}

/// Vehicle feature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    /// Feature type.
    #[serde(rename = "type")]
    pub feature_type: Option<String>,

    /// Feature code.
    pub code: Option<String>,

    /// Feature name.
    pub name: Option<String>,

    /// Feature description.
    pub description: Option<String>,

    /// Is excluded.
    pub excluded: Option<bool>,

    /// Gallery image.
    #[serde(rename = "galleryImage")]
    pub gallery_image: Option<Image>,

    /// Thumbnail image.
    pub thumbnail: Option<Image>,
}

/// Image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Image URL.
    pub url: Option<String>,

    /// Alt text.
    pub alt: Option<String>,
}

/// Electrical engine number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalEngineNumber {
    /// Engine number.
    pub number: Option<String>,

    /// Placement.
    pub placement: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

    #[test]
    fn test_current_vehicle_summary_deserialization() {
        let json = r#"{
            "vin": "YSMYKEAE7RB000000",
            "internalVehicleIdentifier": "1aaeb452-700e-46f3-9899-395b6219c8a6",
            "registrationNo": "MLB007",
            "modelYear": "2024",
            "modelName": "Polestar 3",
            "pno34": "359...",
            "structureWeek": "202420"
        }"#;

        let vehicle: Vehicle = serde_json::from_str(json).unwrap();
        assert_eq!(vehicle.model_name.as_deref(), Some("Polestar 3"));
        assert_eq!(vehicle.model_year.as_deref(), Some("2024"));
        assert!(vehicle.content.model.name.is_none());
    }

    #[test]
    fn test_performance_optimization_specification_flexible_shape() {
        let json = json!({
            "vin": "YSMYKEAE7RB000000",
            "content": {
                "performanceOptimizationSpecification": true,
                "model": {"code": "P2", "name": "Polestar 2"},
            }
        });

        let vehicle: Vehicle = serde_json::from_value(json).unwrap();
        assert!(matches!(
            vehicle
                .content
                .performance_optimization_specification
                .expect("spec should deserialize"),
            PerformanceOptimizationSpecification::Other(_)
        ));

        let json = r#"{
            "vin": "YSMYKEAE7RB000000",
            "content": {
                "performanceOptimizationSpecification": {
                    "power": { "value": "201", "unit": "kW" },
                    "torqueMax": { "value": "420", "unit": "Nm" }
                },
                "model": {"code":"P2","name":"Polestar 2"}
            }
        }"#;

        let vehicle: Vehicle = serde_json::from_str(json).unwrap();
        assert!(matches!(
            vehicle
                .content
                .performance_optimization_specification
                .expect("spec should deserialize"),
            PerformanceOptimizationSpecification::Known(_)
        ));
    }
}
