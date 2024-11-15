use futures::StreamExt;
use serde_json;
use azure_data_cosmos::{CosmosClient, PartitionKey};
use azure_identity::DefaultAzureCredential;
use crate::item::Item;

pub async fn run<F>(
    endpoint: String,
    database_name: String,
    container_name: String,
    callback: F,
)
where
    F: Fn(String),
{
    callback("Current Status:\tStarting...".to_string());

    let credential = DefaultAzureCredential::new().unwrap();

    let client = match CosmosClient::new(&endpoint, credential, None) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error creating CosmosClient: {}", e);
            return;
        }
    };
    callback("Client created".to_string());

    let database = client.database_client(&database_name);
    callback(format!("Get database:\t {}", database_name));

    let container = database.container_client(&container_name);
    callback(format!("Get container:\t {}", container_name));

    {
        let item = Item {
            id: "aaaaaaaa-0000-1111-2222-bbbbbbbbbbbb".to_string(),
            category: "gear-surf-surfboards".to_string(),
            name: "Yamba Surfboard".to_string(),
            quantity: 12,
            price: 850.00,
            clearance: false,
        };

        let partition_key = PartitionKey::from(item.category.clone());

        let upsert_response = container.upsert_item(partition_key, item, None).await;

        match upsert_response {
            Ok(r) => {
                let deserialize_response = r.deserialize_body().await;
                match deserialize_response {
                    Ok(i) => {
                        let upserted_item = i.unwrap();
                        callback(format!("Upserted item:\t{}", upserted_item.id));
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
    
    {
        let item = Item {
            id: "bbbbbbbb-1111-2222-3333-cccccccccccc".to_string(),
            category: "gear-surf-surfboards".to_string(),
            name: "Kiama Classic Surfboard".to_string(),
            quantity: 25,
            price: 790.00,
            clearance: true,
        };

        let partition_key = PartitionKey::from(item.category.clone());

        let upsert_response = container.upsert_item(partition_key, item, None).await;

        match upsert_response {
            Ok(r) => {
                let deserialize_response = r.deserialize_body().await;
                match deserialize_response {
                    Ok(i) => {
                        let upserted_item = i.unwrap();
                        callback(format!("Upserted item:\t{}", upserted_item.id));
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

    {
        let item_id = "aaaaaaaa-0000-1111-2222-bbbbbbbbbbbb";
        let item_partition_key = "gear-surf-surfboards";

        let read_response = container.read_item::<Item>(item_partition_key, item_id, None).await;

        match read_response {
            Ok(r) => {
                let deserialize_response = r.deserialize_body().await;
                match deserialize_response {
                    Ok(i) => {
                        let read_item = i.unwrap();
                        callback(format!("Read item:\t{}\t{}", read_item.id, read_item.category));
                    },
                    Err(e) => {
                        eprintln!("Error deserializing response: {}", e);
                    },
                }
            },
            Err(e) => {
                eprintln!("Error reading item: {}", e);
            },
        }
    }

    {
        let item_partition_key = "gear-surf-surfboards";

        let partition_key = PartitionKey::from(item_partition_key);

        let query = format!("SELECT * FROM c WHERE c.category = '{}'", item_partition_key);

        let page_response = container.query_items::<Item>(&query, partition_key, None);
        
        callback("Run query:".to_string());
        match page_response {
            Ok(mut page) => {
                while let Some(item) = page.next().await {
                    match item {
                        Ok(i) => {
                            let deserialize_response = i.deserialize_body().await;
                            match deserialize_response {
                                Ok(page) => {
                                    for item in page.items {
                                        callback(serde_json::to_string_pretty(&item).unwrap());
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Error deserializing item: {}", e);
                                },
                            }
                        },
                        Err(e) => {
                            eprintln!("Error querying item: {}", e);
                        },
                    }
                }
            },
            Err(e) => {
                eprintln!("Error querying items: {}", e);
            },
        }
    }
    
    callback("Current Status:\tStopping...".to_string());
}
