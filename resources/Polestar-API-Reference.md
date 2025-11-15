---
Date created: 2025-07-19
---

# Polestar API Documentation

Website: https://www.polestar.com/us/login/explore-car/uuid

More Reference: https://github.com/pypolestar/polestar_api


---

## API Endpoint 1: CarTelematicsV2

### Request Details
| Property           | Value                                            |
| ------------------ | ------------------------------------------------ |
| **Method**         | POST                                             |
| **URL**            | https://pc-api.polestar.com/eu-north-1/mystar-v2 |
| **Status**         | 200                                              |
| **Version**        | HTTP/2                                           |
| **Content-Type**   | application/json                                 |
| **Content-Length** | 801                                              |

### GraphQL Query
```json
{
  "operationName": "CarTelematicsV2",
  "variables": {
    "vins": ["XXXXXXXXXXXXXXX"]
  },
  "query": "query CarTelematicsV2($vins: [String!]!) {\n  carTelematicsV2(vins: $vins) {\n    battery {\n      vin\n      timestamp {\n        seconds\n        nanos\n      }\n      batteryChargeLevelPercentage\n      chargingStatus\n      estimatedChargingTimeToFullMinutes\n      estimatedDistanceToEmptyKm\n      estimatedDistanceToEmptyMiles\n    }\n    health {\n      vin\n      timestamp {\n        seconds\n        nanos\n      }\n      daysToService\n      distanceToServiceKm\n      serviceWarning\n      brakeFluidLevelWarning\n      engineCoolantLevelWarning\n      oilLevelWarning\n    }\n    odometer {\n      vin\n      timestamp {\n        seconds\n        nanos\n      }\n      odometerMeters\n    }\n  }\n}"
}
```

### Response
```json
{
  "data": {
    "carTelematicsV2": {
      "battery": [
        {
          "vin": "LPSED3KA1NL059445",
          "timestamp": {
            "seconds": "1752903173",
            "nanos": 100928162
          },
          "batteryChargeLevelPercentage": 63,
          "chargingStatus": "CHARGING_STATUS_IDLE",
          "estimatedChargingTimeToFullMinutes": 0,
          "estimatedDistanceToEmptyKm": 220,
          "estimatedDistanceToEmptyMiles": 140
        }
      ],
      "health": [
        {
          "vin": "LPSED3KA1NL059445",
          "timestamp": {
            "seconds": "1752600923",
            "nanos": 673959558
          },
          "daysToService": null,
          "distanceToServiceKm": null,
          "serviceWarning": null,
          "brakeFluidLevelWarning": null,
          "engineCoolantLevelWarning": null,
          "oilLevelWarning": null
        }
      ],
      "odometer": [null]
    }
  }
}
```

### Request Headers
| Header | Value |
|--------|-------|
| Host | pc-api.polestar.com |
| User-Agent | Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:139.0) Gecko/20100101 Firefox/139.0 |
| Accept | */* |
| Accept-Language | en-US,en;q=0.5 |
| Accept-Encoding | gzip, deflate, br, zstd |
| Referer | https://www.polestar.com/ |
| content-type | application/json |
| authorization | Bearer [JWT_TOKEN_REDACTED] |
| Content-Length | 801 |
| Origin | https://www.polestar.com |
| Connection | keep-alive |
| Sec-Fetch-Dest | empty |
| Sec-Fetch-Mode | cors |
| Sec-Fetch-Site | same-site |
| Priority | u=4 |

### Response Headers
| Header | Value |
|--------|-------|
| date | Sun, 20 Jul 2025 01:25:09 GMT |
| content-type | application/json; charset=UTF-8 |
| cf-ray | 961ea9bb7992d039-SJC |
| cf-cache-status | HIT |
| access-control-allow-origin | * |
| age | 12 |
| cache-control | s-maxage=180 |
| last-modified | Sun, 20 Jul 2025 01:24:57 GMT |
| strict-transport-security | max-age=15780000; includeSubDomains |
| vary | Accept-Encoding |
| via | 1.1 912d83c7c9b4676eb19f09c9bfabda24.cloudfront.net (CloudFront) |
| access-control-expose-headers | x-amzn-RequestId,x-amzn-ErrorType,x-amz-user-agent,x-amzn-ErrorMessage,Date,x-amz-schema-version |
| x-amz-cf-id | MPPJ_iUKliB2_wdMPKM1fAF2-BhxEsIKFex205SVv5v-g3LbSmBkJQ== |
| x-amz-cf-pop | SFO5-P2 |
| x-amzn-appsync-tokensconsumed | 1 |
| x-amzn-requestid | fe19b9cc-8a57-44e3-b451-6824e948c577 |
| x-cache | Miss from cloudfront |
| server | cloudflare |
| content-encoding | br |
| X-Firefox-Spdy | h2 |

---

## API Endpoint 2: GetConsumerCarsV2

### Request Details
| Property | Value |
|----------|-------|
| **Method** | POST |
| **URL** | https://pc-api.polestar.com/eu-north-1/mystar-v2 |
| **Status** | 200 |
| **Version** | HTTP/2 |
| **Content-Type** | application/json |
| **Content-Length** | 1850 |

### GraphQL Query
```json
{
  "operationName": "GetConsumerCarsV2",
  "variables": {
    "locale": "en_US"
  },
  "query": "query GetConsumerCarsV2($locale: String) {\n  getConsumerCarsV2(locale: $locale) {\n    vin\n    primaryDriver\n    internalVehicleIdentifier\n    registrationNo\n    market\n    currentPlannedDeliveryDate\n    deliveryDate\n    edition\n    pno34\n    owners {\n      information {\n        ownerType\n        polestarId\n      }\n    }\n    hasPerformancePackage\n    software {\n      performanceOptimization {\n        value\n      }\n    }\n    content {\n      images {\n        studio {\n          url\n          angles\n        }\n      }\n      exterior {\n        name\n      }\n      model {\n        code\n        name\n      }\n      interior {\n        name\n      }\n      wheels {\n        name\n      }\n      dimensions {\n        wheelbase {\n          label\n          value\n        }\n        groundClearanceWithPerformance {\n          value\n          label\n        }\n        groundClearanceWithoutPerformance {\n          value\n          label\n        }\n        dimensions {\n          label\n          value\n        }\n      }\n      specification {\n        totalHp\n        torque\n        totalKw\n        battery\n        trunkCapacity {\n          value\n          label\n        }\n      }\n      motor {\n        name\n      }\n      performanceOptimizationSpecification {\n        power {\n          value\n          unit\n        }\n        torqueMax {\n          value\n          unit\n        }\n        acceleration {\n          value\n          unit\n          description\n        }\n      }\n    }\n    modelYear\n    commercialModelYear\n    computedModelYear\n    numberOfDoors\n    features {\n      code\n      description\n      name\n      type\n    }\n    fuelType\n    originalMarket\n    userIsPrimaryDriver\n  }\n}"
}
```

### Response
```json
{
  "data": {
    "getConsumerCarsV2": [
      {
        "vin": "LPSED3KA1NL059445",
        "primaryDriver": "2830fa23-93aa-4dc2-a97f-8e5a2ad8444a",
        "internalVehicleIdentifier": "c1182427-69b7-41a1-8cbe-1332944c743c",
        "registrationNo": null,
        "market": "US",
        "currentPlannedDeliveryDate": "2022-01-03",
        "deliveryDate": "2022-01-10",
        "edition": null,
        "pno34": "534EDPP0E13101900RFA000      00000001147001162XPLUSSXPSC01",
        "owners": [
          {
            "information": {
              "ownerType": "private",
              "polestarId": "2830fa23-93aa-4dc2-a97f-8e5a2ad8444a"
            }
          }
        ],
        "hasPerformancePackage": false,
        "software": {
          "performanceOptimization": {
            "value": false
          }
        },
        "content": {
          "images": {
            "studio": {
              "url": "https://cas.polestar.com/image/dynamic/MY22_2108/534/summary-transparent-v2/ED/01900/RFA000/R149/JT02/BD02/EV05/JB11/2G03/_/001162/_/default.png?market=us",
              "angles": ["0", "1", "2", "3", "4", "5"]
            }
          },
          "exterior": {
            "name": "Void"
          },
          "model": {
            "code": "534",
            "name": "Polestar 2"
          },
          "interior": {
            "name": "Slate WeaveTech with Black Ash deco"
          },
          "wheels": {
            "name": "20\" 4-V Spoke Black Diamond Cut Alloy Wheel"
          },
          "dimensions": {
            "wheelbase": {
              "label": "Wheelbase",
              "value": "2,735 mm"
            },
            "groundClearanceWithPerformance": {
              "value": "146 mm",
              "label": " Ground Clearance With Performance"
            },
            "groundClearanceWithoutPerformance": {
              "value": "151 mm",
              "label": "Ground Clearance W/O Performance"
            },
            "dimensions": {
              "label": "Dimensions (L/H/W)",
              "value": "4.6m/1.48m/1.98m"
            }
          },
          "specification": {
            "totalHp": "408 hp",
            "torque": "660 Nm / 487 lb-ft",
            "totalKw": "300 kW",
            "battery": "78 kWh",
            "trunkCapacity": {
              "value": "438 litre (front and back combined)",
              "label": null
            }
          },
          "motor": {
            "name": "Long range Dual motor - AWD"
          },
          "performanceOptimizationSpecification": null
        },
        "modelYear": "2022",
        "commercialModelYear": "2022",
        "computedModelYear": "2022",
        "numberOfDoors": 5,
        "features": [
          {
            "code": "RFA000",
            "description": "<p>Slate WeaveTech with Black Ash deco</p>\n",
            "name": "Slate WeaveTech with Black Ash deco",
            "type": "interior"
          },
          {
            "code": "01900",
            "description": "",
            "name": "Void",
            "type": "exterior"
          },
          {
            "code": "ED",
            "description": "",
            "name": "Long range Dual motor - AWD",
            "type": "motor"
          },
          {
            "code": "001162",
            "description": "<ul>\n<li>Pixel LED headlights</li>\n<li>Light sequences</li>\n<li>LED front fog lights with cornering lights</li>\n<li>360° camera</li>\n<li>Driver assistance with Pilot Assist, Adaptive Cruise Control, and Emergency Stop Assist</li>\n<li>Driver awareness with Blind Spot Information System (BLIS) with steering assist, Cross Traffic Alert with brake support, and Rear Collision Warning & Mitigation</li>\n<li>Automatically dimmed exterior mirrors</li>\n<li>Park Assist side</li>\n</ul>\n",
            "name": "Pilot",
            "type": "pilotPackage"
          },
          {
            "code": "001147",
            "description": "",
            "name": "20\" 4-V Spoke Black Diamond Cut Alloy Wheel",
            "type": "wheels"
          },
          {
            "code": "XPLUSS",
            "description": "<ul>\n<li>Heat pump</li>\n<li>Fixed panoramic sunroof including projected Polestar symbol</li>\n<li>Harman Kardon Premium Sound</li>\n<li>WeaveTech (Charcoal or Slate) seats with Black Ash decor inlays</li>\n<li>Full power seats including 4-way power lumbar support (memory on driver seat), manual cushion extension and backrest storage nets</li>\n<li>Heated rear seat, steering wheel and washer fluid nozzles</li>\n<li>Interior high level illumination</li>\n<li>Inductive charging 15W for smart phone</li>\n<li>Grocery bag holder</li>\n</ul>\n",
            "name": "Plus",
            "type": "plusPackage"
          },
          {
            "code": "534",
            "description": "",
            "name": "Polestar 2",
            "type": "model"
          }
        ],
        "fuelType": "All-Electric",
        "originalMarket": "US",
        "userIsPrimaryDriver": true
      }
    ]
  }
}
```

### Request Headers
| Header | Value |
|--------|-------|
| Host | pc-api.polestar.com |
| User-Agent | Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:139.0) Gecko/20100101 Firefox/139.0 |
| Accept | */* |
| Accept-Language | en-US,en;q=0.5 |
| Accept-Encoding | gzip, deflate, br, zstd |
| Referer | https://www.polestar.com/ |
| content-type | application/json |
| authorization | Bearer [JWT_TOKEN_REDACTED] |
| Content-Length | 1850 |
| Origin | https://www.polestar.com |
| Connection | keep-alive |
| Sec-Fetch-Dest | empty |
| Sec-Fetch-Mode | cors |
| Sec-Fetch-Site | same-site |
| Priority | u=4 |
| TE | trailers |

### Response Headers
| Header | Value |
|--------|-------|
| date | Sun, 20 Jul 2025 01:32:43 GMT |
| content-type | application/json; charset=UTF-8 |
| cf-ray | 961eb4d02a37cecd-SJC |
| cf-cache-status | DYNAMIC |
| access-control-allow-origin | * |
| cache-control | s-maxage=180 |
| content-encoding | gzip |
| strict-transport-security | max-age=15780000; includeSubDomains |
| vary | accept-encoding |
| via | 1.1 87e907bf938f21f1b962d1401b077d14.cloudfront.net (CloudFront) |
| access-control-expose-headers | x-amzn-RequestId,x-amzn-ErrorType,x-amz-user-agent,x-amzn-ErrorMessage,Date,x-amz-schema-version |
| x-amz-cf-id | nzciRXdfITbq67KrLLMgtRNuArJ8-2-2w9sVccZLimZa2TdaEgggSw== |
| x-amz-cf-pop | SFO5-P2 |
| x-amzn-appsync-tokensconsumed | 1 |
| x-amzn-requestid | da696355-417c-419b-80d3-6e8e620e9819 |
| x-cache | Miss from cloudfront |
| server | cloudflare |
| X-Firefox-Spdy | h2 |

---

## API Endpoint 3: getCarSpecifications

### Request Details
| Property | Value |
|----------|-------|
| **Method** | POST |
| **URL** | https://cms-api.polestar.com/ |
| **Status** | 200 |
| **Version** | HTTP/2 |
| **Content-Type** | application/json |
| **Content-Length** | 500 |

### GraphQL Query
```json
{
  "operationName": "getCarSpecifications",
  "variables": {
    "locale": "en_US"
  },
  "query": "query getCarSpecifications($locale: SiteLocale) {\n  loggedinCarSpecification(locale: $locale, fallbackLocales: [en]) {\n    title {\n      key\n      value\n    }\n    specificationGroups {\n      groupId\n      label {\n        key\n        value\n      }\n      rows {\n        specificationId\n        label {\n          key\n          value\n        }\n      }\n    }\n    chargeport {\n      value\n    }\n  }\n}"
}
```

### Response
```json
{
  "data": {
    "loggedinCarSpecification": {
      "title": {
        "key": "specifications",
        "value": "Specifications"
      },
      "specificationGroups": [
        {
          "groupId": "carId",
          "label": {
            "key": "Car ID",
            "value": "Car ID"
          },
          "rows": [
            {
              "specificationId": "vin",
              "label": {
                "key": "VIN",
                "value": "VIN"
              }
            },
            {
              "specificationId": "reg",
              "label": {
                "key": "REG (registration number)",
                "value": "LICENSE PLATE"
              }
            }
          ]
        },
        {
          "groupId": "style",
          "label": {
            "key": "Style",
            "value": "Style"
          },
          "rows": [
            {
              "specificationId": "exterior",
              "label": {
                "key": "Exterior",
                "value": "Exterior"
              }
            },
            {
              "specificationId": "interior",
              "label": {
                "key": "Interior",
                "value": "Interior"
              }
            },
            {
              "specificationId": "wheels",
              "label": {
                "key": "Wheels",
                "value": "Wheels"
              }
            }
          ]
        },
        {
          "groupId": "motor",
          "label": {
            "key": "Motor",
            "value": "Motor"
          },
          "rows": [
            {
              "specificationId": "motor",
              "label": {
                "key": "Motor",
                "value": "Motor"
              }
            },
            {
              "specificationId": "totalKw",
              "label": {
                "key": "total kW",
                "value": "Total kW"
              }
            },
            {
              "specificationId": "power",
              "label": {
                "key": "Power",
                "value": "Power"
              }
            },
            {
              "specificationId": "torque",
              "label": {
                "key": "Torque",
                "value": "Torque"
              }
            }
          ]
        },
        {
          "groupId": "battery",
          "label": {
            "key": "Battery",
            "value": "Battery"
          },
          "rows": [
            {
              "specificationId": "battery",
              "label": {
                "key": "Battery",
                "value": "Battery"
              }
            },
            {
              "specificationId": "chargeport",
              "label": {
                "key": "Chargeport",
                "value": "Charging port"
              }
            }
          ]
        },
        {
          "groupId": "features",
          "label": {
            "key": "Features",
            "value": "Features"
          },
          "rows": [
            {
              "specificationId": "packs",
              "label": {
                "key": "Packs",
                "value": "Packs"
              }
            }
          ]
        },
        {
          "groupId": "otherSpecifications",
          "label": {
            "key": "Other specifications",
            "value": "Other specifications"
          },
          "rows": [
            {
              "specificationId": "modelYear",
              "label": {
                "key": "Model year",
                "value": "Model year"
              }
            },
            {
              "specificationId": "dimensions",
              "label": {
                "key": "Dimensions (L/H/W)",
                "value": "Dimensions (L/H/W)"
              }
            },
            {
              "specificationId": "wheelbase",
              "label": {
                "key": "Wheelbase",
                "value": "Wheelbase"
              }
            },
            {
              "specificationId": "trunkCapacity",
              "label": {
                "key": "Trunk capacity",
                "value": "Trunk capacity"
              }
            },
            {
              "specificationId": "groundClearance",
              "label": {
                "key": "Ground clearance",
                "value": "Ground clearance"
              }
            },
            {
              "specificationId": "groundClearanceWithPerformance",
              "label": {
                "key": "Ground clearance with performance package",
                "value": "Ground clearance with performance"
              }
            }
          ]
        }
      ],
      "chargeport": {
        "value": "Type 2 and CCS"
      }
    }
  }
}
```

### Request Headers
| Header | Value |
|--------|-------|
| Host | cms-api.polestar.com |
| User-Agent | Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:139.0) Gecko/20100101 Firefox/139.0 |
| Accept | */* |
| Accept-Language | en-US,en;q=0.5 |
| Accept-Encoding | gzip, deflate, br, zstd |
| Referer | https://www.polestar.com/ |
| content-type | application/json |
| authorization | Bearer be17dc70aea5f8ed8388f1fa7ee053 |
| Content-Length | 500 |
| Origin | https://www.polestar.com |
| Connection | keep-alive |
| Sec-Fetch-Dest | empty |
| Sec-Fetch-Mode | cors |
| Sec-Fetch-Site | same-site |
| Priority | u=4 |
| TE | trailers |

### Response Headers
| Header                                | Value                                                                                                                                                           |
| ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| date                                  | Sun, 20 Jul 2025 01:32:44 GMT                                                                                                                                   |
| content-type                          | application/json; charset=utf-8                                                                                                                                 |
| cf-ray                                | 961eb4d71e4635d6-SJC                                                                                                                                            |
| cf-cache-status                       | MISS                                                                                                                                                            |
| access-control-allow-origin           | https://www.polestar.com                                                                                                                                        |
| age                                   | 373927                                                                                                                                                          |
| cache-control                         | no-cache                                                                                                                                                        |
| etag                                  | W/"f4ff8e149beb7e1f5965cbe734558823"                                                                                                                            |
| strict-transport-security             | max-age=15780000; includeSubDomains                                                                                                                             |
| vary                                  | Authorization, Accept-Encoding, X-Environment, X-Include-Drafts, X-Exclude-Invalid, X-Visual-Editing, X-Base-Editing-Url, X-Cache-Tags, X-DatoCMS-Trace, Origin |
| access-control-allow-credentials      | true                                                                                                                                                            |
| access-control-allow-headers          | authorization, content-type, x-environment, x-include-drafts, x-exclude-invalid, x-visual-editing, x-base-editing-url, x-cache-tags, x-datocms-trace            |
| access-control-allow-methods          | GET, POST                                                                                                                                                       |
| access-control-expose-headers         | x-ratelimit-limit, x-ratelimit-remaining, x-ratelimit-reset, x-entities, x-complexity, x-max-complexity, x-cache-tags                                           |
| access-control-max-age                | 1728000                                                                                                                                                         |
| referrer-policy                       | strict-origin-when-cross-origin                                                                                                                                 |
| x-analysis-fields-count               | 10                                                                                                                                                              |
| x-analysis-introspection              | 0                                                                                                                                                               |
| x-analysis-item-types-count           | 4                                                                                                                                                               |
| x-analysis-successful                 | 1                                                                                                                                                               |
| x-batch                               | 0                                                                                                                                                               |
| x-cache                               | MISS                                                                                                                                                            |
| x-cacheable-on-cdn                    | true                                                                                                                                                            |
| x-cacheable-on-cdn-query-length-limit | 424/12000                                                                                                                                                       |
| x-complexity                          | 438                                                                                                                                                             |
| x-content-type-options                | nosniff                                                                                                                                                         |
| x-environment                         | master                                                                                                                                                          |
| x-frame-options                       | SAMEORIGIN                                                                                                                                                      |
| x-max-complexity                      | 2393010326                                                                                                                                                      |
| x-permitted-cross-domain-policies     | none                                                                                                                                                            |
| x-query-digest                        | 51ddeeadb75c80c3db398249935b1061                                                                                                                                |
| x-queue-time                          | 1ms                                                                                                                                                             |
| x-request-digest                      | f26b23cb26972e97caeb3d1d92c40e6e                                                                                                                                |
| x-request-id                          | 223666bc-ceb4-40ee-a1ba-45f13f2e5d80                                                                                                                            |
| x-runtime                             | 0.141690                                                                                                                                                        |
| x-timings-execution                   | 0.023                                                                                                                                                           |
| x-timings-preanalysis                 | 0.064                                                                                                                                                           |
| x-timings-schema-generation           | 0.028                                                                                                                                                           |
| x-timings-total                       | 0.115                                                                                                                                                           |
| x-xss-protection                      | 0                                                                                                                                                               |
| server                                | cloudflare                                                                                                                                                      |
| content-encoding                      | br                                                                                                                                                              |
| X-Firefox-Spdy                        | h2                                                                                                                                                              |

---

## API Summary

### Base URLs
- **Main API**: `https://pc-api.polestar.com/eu-north-1/mystar-v2`
- **CMS API**: `https://cms-api.polestar.com/`

### Authentication
- Uses Bearer token authentication
- Different tokens for different endpoints:
  - Main API: JWT token (long format)
  - CMS API: Shorter token format

### Common Headers
- **Content-Type**: `application/json`
- **Origin**: `https://www.polestar.com`
- **Referer**: `https://www.polestar.com/`
- **User-Agent**: Firefox browser

### GraphQL Operations
1. **CarTelematicsV2**: Vehicle telemetry data (battery, health, odometer)
2. **GetConsumerCarsV2**: Complete vehicle information and specifications
3. **getCarSpecifications**: Vehicle specification metadata and structure
