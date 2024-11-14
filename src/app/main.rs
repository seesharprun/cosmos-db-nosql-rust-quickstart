use serde::{Deserialize, Serialize};

use dotenv;

use azure_data_cosmos::{CosmosClient, CosmosClientOptions, PartitionKey};
use azure_identity::DefaultAzureCredential;

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: String,
    category: String,
    name: String,
    quantity: i32,
    price: f64,
    clearance: bool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let credential = DefaultAzureCredential::new().unwrap();

    let endpoint = std::env::var("CONFIGURATION__AZURECOSMOSDB__ENDPOINT").expect("Missing CONFIGURATION__AZURECOSMOSDB__ENDPOINT environment variable.");

    let client_options = CosmosClientOptions::default();
    let client_options = Some(client_options);

    let service_client = match CosmosClient::new(&endpoint, credential, client_options) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error creating CosmosClient: {}", e);
            return;
        }
    };

    let database_name = std::env::var("CONFIGURATION__AZURECOSMOSDB__DATABASENAME").expect("Missing CONFIGURATION__AZURECOSMOSDB__DATABASENAME environment variable.");

    let database_client = service_client.database_client(&database_name);

    let container_name = std::env::var("CONFIGURATION__AZURECOSMOSDB__CONTAINERNAME").expect("Missing CONFIGURATION__AZURECOSMOSDB__CONTAINERNAME environment variable.");

    let container_client = database_client.container_client(&container_name);

    let item = Item {
        id: "aaaaaaaa-0000-1111-2222-bbbbbbbbbbbb".to_string(),
        category: "gear-surf-surfboards".to_string(),
        name: "Yamba Surfboard".to_string(),
        quantity: 12,
        price: 850.00,
        clearance: false,
    };

    let partition_key = PartitionKey::from(item.category.clone());

    let upsert_response = container_client.upsert_item(partition_key, item, None).await;
    match upsert_response {
        Ok(i) => {
            println!("Item upserted successfully.");
            let deserialize_response = i.deserialize_body().await;
            match deserialize_response {
                Ok(i) => {
                    let upserted_item = i.unwrap();
                    println!("Upserted item: {:?}", upserted_item);
                },
                Err(e) => {
                    eprintln!("Error deserializing response: {}", e);
                },
            }
        },
        Err(e) => {
            eprintln!("Error upserting item: {}", e);
        },
    }
}