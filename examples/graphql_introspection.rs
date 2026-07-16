//! GraphQL introspection tool to discover available API endpoints
//!
//! This tool performs GraphQL introspection to discover all available
//! queries, mutations, and types in the Polestar API.

use polestar_api::{PolestarClient, PolestarError};
use serde_json::Value;
use std::env;

const INTROSPECTION_QUERY: &str = r#"
{
  __schema {
    types {
      kind
      name
      description
      fields {
        name
        description
        args {
          name
          description
          type {
            name
            kind
          }
        }
        type {
          name
          kind
        }
      }
    }
  }
}
"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Load credentials from environment
    let username = env::var("POLESTAR_USERNAME").expect("POLESTAR_USERNAME must be set");
    let password = env::var("POLESTAR_PASSWORD").expect("POLESTAR_PASSWORD must be set");

    println!("Polestar GraphQL API Introspection Tool");
    println!("==========================================\n");

    // Create client
    println!("Authenticating with Polestar API...");
    let client = PolestarClient::new(&username, &password)?;

    // Perform introspection
    println!("Performing GraphQL introspection...\n");

    match introspect_api(&client).await {
        Ok(schema) => {
            analyze_schema(&schema);
        }
        Err(e) => {
            eprintln!("ERROR: Introspection failed: {}", e);
            eprintln!("\nThis could mean:");
            eprintln!("  - The API doesn't allow introspection");
            eprintln!("  - Authentication failed");
            eprintln!("  - Network issues");
            return Err(e.into());
        }
    }

    Ok(())
}

async fn introspect_api(_client: &PolestarClient) -> Result<Value, PolestarError> {
    // We need to use the internal post_graphql method
    // Since it's private, we'll do it manually

    let http_client = reqwest::Client::builder()
        .cookie_store(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // First authenticate to get token
    let auth_state = std::sync::Arc::new(polestar_api::auth::AuthState::new(
        std::env::var("POLESTAR_USERNAME").unwrap(),
        std::env::var("POLESTAR_PASSWORD").unwrap(),
    ));

    let token = auth_state.get_valid_token(&http_client).await?;

    let body = serde_json::json!({
        "query": INTROSPECTION_QUERY,
        "variables": {}
    });

    let endpoint = "https://pc-api.polestar.com/eu-north-1/mystar-v2";

    let response = http_client
        .post(endpoint)
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .header("origin", "https://www.polestar.com")
        .json(&body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    // Check for errors
    if let Some(errors) = json.get("errors") {
        if let Some(message) = errors.get(0).and_then(|e| e.get("message")) {
            return Err(PolestarError::GraphQLError(message.to_string()));
        }
    }

    Ok(json)
}

fn analyze_schema(schema: &Value) {
    println!("SCHEMA ANALYSIS");
    println!("==================\n");

    // Extract schema data
    let schema_data = match schema.get("data").and_then(|d| d.get("__schema")) {
        Some(s) => s,
        None => {
            println!("WARNING: No schema data found - introspection may be disabled");
            println!("\nRaw response:");
            println!("{}", serde_json::to_string_pretty(schema).unwrap());
            return;
        }
    };

    // Get query type
    if let Some(query_type) = schema_data.get("queryType") {
        println!(
            "Query Type: {}",
            query_type
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown")
        );
    }

    // Get mutation type
    if let Some(mutation_type) = schema_data.get("mutationType") {
        println!(
            "Mutation Type: {}",
            mutation_type
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("None")
        );
    } else {
        println!("Mutation Type: None");
    }

    // Get subscription type
    if let Some(subscription_type) = schema_data.get("subscriptionType") {
        println!(
            "Subscription Type: {}",
            subscription_type
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("None")
        );
    } else {
        println!("Subscription Type: None");
    }

    println!();

    // Analyze types
    if let Some(types) = schema_data.get("types").and_then(|t| t.as_array()) {
        println!("AVAILABLE TYPES ({})", types.len());
        println!("==================\n");

        // Separate query types
        let mut query_types = Vec::new();
        let mut mutation_types = Vec::new();
        let mut object_types = Vec::new();
        let mut input_types = Vec::new();
        let mut enum_types = Vec::new();

        for type_def in types {
            let kind = type_def.get("kind").and_then(|k| k.as_str()).unwrap_or("");
            let name = type_def
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("Unknown");

            // Skip introspection types
            if name.starts_with("__") {
                continue;
            }

            match kind {
                "OBJECT" => {
                    // Check if this is the Query or Mutation type
                    if name == "Query" {
                        if let Some(fields) = type_def.get("fields").and_then(|f| f.as_array()) {
                            for field in fields {
                                query_types.push(extract_field_info(field));
                            }
                        }
                    } else if name == "Mutation" {
                        if let Some(fields) = type_def.get("fields").and_then(|f| f.as_array()) {
                            for field in fields {
                                mutation_types.push(extract_field_info(field));
                            }
                        }
                    } else {
                        object_types.push((name.to_string(), type_def.clone()));
                    }
                }
                "INPUT_OBJECT" => input_types.push((name.to_string(), type_def.clone())),
                "ENUM" => enum_types.push((name.to_string(), type_def.clone())),
                _ => {}
            }
        }

        // Display queries
        println!("AVAILABLE QUERIES ({})", query_types.len());
        println!("====================\n");

        let implemented_queries = ["carTelematicsV2", "getConsumerCarsV2"];

        for field_info in &query_types {
            let status = if implemented_queries.contains(&field_info.name.as_str()) {
                "[IMPLEMENTED]"
            } else {
                "[NOT IMPLEMENTED]"
            };

            println!("{} {}", status, field_info.name);
            if let Some(desc) = &field_info.description {
                println!("   Description: {}", desc);
            }
            if !field_info.args.is_empty() {
                println!("   Arguments:");
                for arg in &field_info.args {
                    println!("     - {}: {}", arg.name, arg.type_name);
                    if let Some(desc) = &arg.description {
                        println!("       {}", desc);
                    }
                }
            }
            println!("   Returns: {}", field_info.return_type);
            println!();
        }

        // Display mutations if any
        if !mutation_types.is_empty() {
            println!("\nAVAILABLE MUTATIONS ({})", mutation_types.len());
            println!("======================\n");

            for field_info in &mutation_types {
                println!("[NOT IMPLEMENTED] {}", field_info.name);
                if let Some(desc) = &field_info.description {
                    println!("   Description: {}", desc);
                }
                if !field_info.args.is_empty() {
                    println!("   Arguments:");
                    for arg in &field_info.args {
                        println!("     - {}: {}", arg.name, arg.type_name);
                    }
                }
                println!("   Returns: {}", field_info.return_type);
                println!();
            }
        }

        // Summary
        println!("\nSUMMARY");
        println!("==========");
        println!("Total Queries: {}", query_types.len());
        println!(
            "Implemented: {}",
            query_types
                .iter()
                .filter(|f| implemented_queries.contains(&f.name.as_str()))
                .count()
        );
        println!(
            "Not Implemented: {}",
            query_types
                .iter()
                .filter(|f| !implemented_queries.contains(&f.name.as_str()))
                .count()
        );
        println!("Total Mutations: {}", mutation_types.len());
        println!("Total Object Types: {}", object_types.len());
        println!("Total Input Types: {}", input_types.len());
        println!("Total Enum Types: {}", enum_types.len());
    }
}

#[derive(Debug)]
struct FieldInfo {
    name: String,
    description: Option<String>,
    args: Vec<ArgInfo>,
    return_type: String,
}

#[derive(Debug)]
struct ArgInfo {
    name: String,
    description: Option<String>,
    type_name: String,
}

fn extract_field_info(field: &Value) -> FieldInfo {
    let name = field
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("Unknown")
        .to_string();

    let description = field
        .get("description")
        .and_then(|d| d.as_str())
        .map(|s| s.to_string());

    let mut args = Vec::new();
    if let Some(args_array) = field.get("args").and_then(|a| a.as_array()) {
        for arg in args_array {
            args.push(ArgInfo {
                name: arg
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string(),
                description: arg
                    .get("description")
                    .and_then(|d| d.as_str())
                    .map(|s| s.to_string()),
                type_name: extract_type_name(arg.get("type")),
            });
        }
    }

    let return_type = extract_type_name(field.get("type"));

    FieldInfo {
        name,
        description,
        args,
        return_type,
    }
}

fn extract_type_name(type_ref: Option<&Value>) -> String {
    match type_ref {
        Some(t) => {
            let kind = t.get("kind").and_then(|k| k.as_str()).unwrap_or("");
            match kind {
                "NON_NULL" => {
                    format!("{}!", extract_type_name(t.get("ofType")))
                }
                "LIST" => {
                    format!("[{}]", extract_type_name(t.get("ofType")))
                }
                _ => t
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
            }
        }
        None => "Unknown".to_string(),
    }
}
