# GraphQL Endpoint Analysis

> **Historical analysis:** Endpoint observations below were captured in November
> 2025 and may no longer match the private upstream schema. The README describes
> the currently supported queries.

Date: 2025-11-27

## Summary

This document analyzes the Polestar GraphQL API endpoints and compares the currently implemented endpoints in polestar-api-rs with those available in the pypolestar Python library.

## Introspection Results

The Polestar GraphQL API has introspection **DISABLED**. This is a security measure that prevents automated discovery of the schema. All endpoint discovery was done through:

1. Manual API documentation analysis
2. Review of the pypolestar Python reference implementation
3. Existing documentation in this repository

## Available GraphQL Endpoints

### Endpoint 1: GetConsumerCarsV2 (Basic)

**Status**: IMPLEMENTED

**Operation Name**: `GetConsumerCarsV2`

**Endpoint**: `https://pc-api.polestar.com/eu-north-1/mystar-v2`

**Variables**: None (or optional `locale` parameter)

**Purpose**: Retrieve basic vehicle information for all vehicles owned by the authenticated user

**Fields Retrieved**:
- vin
- internalVehicleIdentifier
- registrationNo
- registrationDate
- factoryCompleteDate
- content.model.name
- content.specification.battery
- content.specification.torque
- software.version
- software.versionTimestamp

**Implementation**: `src/client.rs:146` - `get_vehicle()` method

---

### Endpoint 2: GetConsumerCarsV2 (Verbose)

**Status**: NOT IMPLEMENTED

**Operation Name**: `GetConsumerCarsV2`

**Endpoint**: `https://pc-api.polestar.com/eu-north-1/mystar-v2`

**Variables**: None (or optional `locale` parameter)

**Purpose**: Retrieve comprehensive vehicle information including service history, features, and detailed specifications

**Additional Fields Beyond Basic Version**:
- salesType
- currentPlannedDeliveryDate
- market, originalMarket
- metaOrderNumber
- **serviceHistory** - Complete service records with:
  - claimType, mileage, operations, parts
  - orderNumber, orderStartDate, orderEndDate
  - statusDMS, symptomCode, vehicleAge, workshopId
- **content** (expanded):
  - exteriorDetails, performancePackage
  - performanceOptimizationSpecification (power, torque, acceleration)
  - plusPackage, pilotPackage, towbar
  - Full specification details (brakes, suspension, tireSizes, etc.)
- **wltpNedcData** - WLTP/NEDC emissions and range data
- **energy** - Energy consumption metrics
- drivetrain, numberOfDoors, numberOfSeats
- maxTrailerWeight, curbWeight
- hasPerformancePackage, numberOfCylinders, cylinderVolume
- transmission, numberOfGears, structureWeek
- **latestClaimStatus** - Most recent service claim
- **commonStatusPoint**, **brandStatus** - Vehicle status codes
- **features** - Complete feature list with images
- electricalEngineNumbers

**Implementation**: None

**Priority**: MEDIUM - Provides rich data for advanced use cases

---

### Endpoint 3: CarTelematicsV2

**Status**: IMPLEMENTED

**Operation Name**: `CarTelematicsV2`

**Endpoint**: `https://pc-api.polestar.com/eu-north-1/mystar-v2`

**Variables**:
- `vins`: [String!]! - Array of VIN numbers

**Purpose**: Retrieve real-time telemetry data for specified vehicles

**Fields Retrieved**:
- **health**:
  - vin, timestamp (seconds, nanos)
  - brakeFluidLevelWarning
  - daysToService, distanceToServiceKm
  - engineCoolantLevelWarning, oilLevelWarning
  - serviceWarning
- **battery**:
  - vin, timestamp (seconds, nanos)
  - batteryChargeLevelPercentage
  - chargingStatus
  - estimatedChargingTimeToFullMinutes
  - estimatedDistanceToEmptyKm
- **odometer**:
  - vin, timestamp (seconds, nanos)
  - odometerMeters

**Implementation**: `src/client.rs:84` - `get_telemetry()` method

---

### Endpoint 4: getCarSpecifications

**Status**: NOT IMPLEMENTED

**Operation Name**: `getCarSpecifications`

**Endpoint**: `https://cms-api.polestar.com/`

**Variables**:
- `locale`: SiteLocale (e.g., "en_US")

**Purpose**: Retrieve vehicle specification metadata structure and labels

**Fields Retrieved**:
- title (key, value)
- specificationGroups:
  - groupId (e.g., "carId", "style", "motor", "battery", "features")
  - label (key, value)
  - rows:
    - specificationId
    - label (key, value)
- chargeport.value

**Implementation**: None

**Priority**: LOW - Metadata for display purposes, not core functionality

**Note**: This endpoint uses a different base URL (CMS API) and may require different authentication

---

## Implementation Comparison

### Current Implementation (polestar-api-rs)

| Endpoint | Status | Method | Location |
|----------|--------|--------|----------|
| GetConsumerCarsV2 (Basic) | IMPLEMENTED | `get_vehicle(vin)` | `src/client.rs:146` |
| CarTelematicsV2 | IMPLEMENTED | `get_telemetry(vin)` | `src/client.rs:84` |

**Total Implemented**: 2 endpoints

### pypolestar Reference Implementation

| Endpoint | Status | Method |
|----------|--------|--------|
| GetConsumerCarsV2 (Basic) | IMPLEMENTED | `QUERY_GET_CONSUMER_CARS_V2` |
| GetConsumerCarsV2 (Verbose) | IMPLEMENTED | `QUERY_GET_CONSUMER_CARS_V2_VERBOSE` |
| CarTelematicsV2 | IMPLEMENTED | `QUERY_TELEMATICS_V2` |

**Total Implemented**: 3 query variants (2 unique endpoints)

---

## Unimplemented Endpoints in polestar-api-rs

### High Priority

None - Core functionality is implemented

### Medium Priority

#### 1. GetConsumerCarsV2 (Verbose Mode)

**Justification**: Provides significantly more data than basic mode:
- Service history tracking
- Complete feature list with images
- WLTP/NEDC emissions data
- Performance specifications
- Weight and towing capacity
- Detailed status information

**Implementation Effort**: LOW
- Same endpoint as existing implementation
- Only requires expanding the GraphQL query string
- Add new fields to Vehicle model structs

**Recommended Approach**:
1. Add `verbose` parameter to `get_vehicle()` method
2. Create expanded query constant `GET_CONSUMER_CARS_V2_VERBOSE`
3. Extend `Vehicle` struct with optional fields for verbose data
4. Update deserialization logic

**Use Cases**:
- Fleet management applications
- Service tracking
- Detailed vehicle analysis
- Historical data collection

### Low Priority

#### 2. getCarSpecifications (CMS API)

**Justification**: Metadata endpoint for UI display

**Implementation Effort**: MEDIUM
- Different base URL (CMS API)
- May require different authentication mechanism
- Primarily useful for building UI labels

**Recommended Approach**:
1. Investigate CMS API authentication requirements
2. Add CMS-specific client method
3. Create specification models
4. Feature-gate behind "metadata" feature flag

---

## Additional Endpoints Not Found

The following endpoints were **NOT** found in either implementation:

### Mutations

No mutation operations were discovered. The Polestar API appears to be read-only from a GraphQL perspective. Vehicle control operations (if they exist) may use different endpoints or protocols.

### Subscriptions

No subscription operations were discovered. Real-time updates likely require polling or are not available via this API.

### Other Query Operations

No additional query operations beyond those documented above were found in the pypolestar reference implementation.

---

## API Characteristics

### Authentication

- **Type**: OAuth2/OIDC with Bearer tokens
- **Provider**: `https://polestarid.eu.polestar.com`
- **Token Type**: JWT
- **Refresh**: Automatic token refresh implemented in `src/auth.rs`

### Rate Limiting

- **Header**: Server may return HTTP 429 (Too Many Requests)
- **Caching**: Response cache control: `s-maxage=180` (3 minutes)
- **Recommendation**: Implement client-side caching with 3-minute TTL

### Introspection

- **Status**: DISABLED
- **Impact**: Cannot automatically discover schema changes
- **Mitigation**: Monitor pypolestar repository for API updates

### Regional Endpoints

- **Current**: `eu-north-1` region
- **Investigation Needed**: Whether other regions exist (US, Asia, etc.)

---

## Recommendations

### Immediate Actions

None - core functionality is complete

### Short-term Enhancements (Next Release)

1. **Add verbose mode to GetConsumerCarsV2**
   - Low implementation effort
   - Significant value for advanced users
   - Maintains API compatibility

2. **Implement response caching**
   - Respect cache-control headers
   - Reduce API load
   - Improve performance
   - Feature-gate behind existing "cache" feature

### Long-term Considerations

1. **Monitor pypolestar for API changes**
   - Set up GitHub watch/notifications
   - Review releases for new endpoints
   - Check for deprecation notices

2. **Investigate CMS API endpoint**
   - Document authentication mechanism
   - Assess value for end users
   - Consider implementation if requested

3. **Explore undocumented endpoints**
   - Monitor web app network traffic
   - Look for mobile app API calls
   - Coordinate with pypolestar maintainers

---

## Testing Strategy

### For New Implementations

1. **Unit Tests**
   - Mock GraphQL responses
   - Test deserialization
   - Validate error handling

2. **Integration Tests**
   - Test against real API (opt-in)
   - Validate response structure
   - Check for schema changes

3. **Example Programs**
   - Create examples/verbose_vehicle_info.rs
   - Demonstrate new functionality
   - Provide usage patterns

---

## References

- pypolestar repository: https://github.com/pypolestar/pypolestar
- pypolestar graphql.py: https://github.com/pypolestar/pypolestar/blob/main/pypolestar/graphql.py
- Polestar API Reference: `resources/Polestar-API-Reference.md`
- Architecture Documentation: `docs/ARCHITECTURE.md`
- Implementation Plan: `docs/PLAN.md`

---

## Appendix: GraphQL Query Strings

### GetConsumerCarsV2 (Basic) - IMPLEMENTED

```graphql
query GetConsumerCarsV2 {
    getConsumerCarsV2 {
        vin
        internalVehicleIdentifier
        registrationNo
        registrationDate
        factoryCompleteDate
        content {
            model { name }
            specification {
                battery
                torque
            }
        }
        software {
            version
            versionTimestamp
        }
    }
}
```

### GetConsumerCarsV2 (Verbose) - NOT IMPLEMENTED

See pypolestar/graphql.py QUERY_GET_CONSUMER_CARS_V2_VERBOSE for complete query (200+ lines)

Key additions:
- serviceHistory with complete service records
- wltpNedcData for emissions data
- energy consumption metrics
- features with images
- performance specifications
- weight and capacity data

### CarTelematicsV2 - IMPLEMENTED

```graphql
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
            timestamp { seconds nanos }
        }
        battery {
            vin
            batteryChargeLevelPercentage
            chargingStatus
            estimatedChargingTimeToFullMinutes
            estimatedDistanceToEmptyKm
            timestamp { seconds nanos }
        }
        odometer {
            vin
            odometerMeters
            timestamp { seconds nanos }
        }
    }
}
```

### getCarSpecifications - NOT IMPLEMENTED

```graphql
query getCarSpecifications($locale: SiteLocale) {
    loggedinCarSpecification(locale: $locale, fallbackLocales: [en]) {
        title {
            key
            value
        }
        specificationGroups {
            groupId
            label {
                key
                value
            }
            rows {
                specificationId
                label {
                    key
                    value
                }
            }
        }
        chargeport {
            value
        }
    }
}
```
