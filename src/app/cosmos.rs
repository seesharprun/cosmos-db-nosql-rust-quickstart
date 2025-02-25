use serde_json;
use azure_data_cosmos::{CosmosClient, PartitionKey, Query};
use azure_identity::ManagedIdentityCredential;
use crate::item::Item;
use futures::stream::StreamExt;

pub async fn run<F>(
    endpoint: String,
    database_name: String,
    container_name: String,
    callback: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(String),
{
    callback("Current Status:\tStarting...".to_string());

    // <create_client>
    let credential = ManagedIdentityCredential::new()?;

    let client = CosmosClient::new(&endpoint, credential, None)?;
    // </create_client>

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
        
        container.upsert_item(partition_key, item.clone(), None).await?;

        callback(format!("Upserted item:\t{}", item.id));
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

        container.upsert_item(partition_key, item.clone(), None).await?;

        callback(format!("Upserted item:\t{}", item.id));
    }

    {
        let item_id = "aaaaaaaa-0000-1111-2222-bbbbbbbbbbbb";
        let item_partition_key = "gear-surf-surfboards";

        let response = container.read_item(item_partition_key, item_id, None).await?;

        let item: Item = response.into_json_body().await?;

        callback(format!("Read item:\t{}\t{}", item.id, item.category));
    }

    {
        let item_partition_key = "gear-surf-surfboards";

        let query = Query::from("SELECT * FROM c WHERE c.category = @category")
            .with_parameter("@category", item_partition_key)?;

        let mut pager = container.query_items::<Item>(query, item_partition_key, None)?;
        
        callback("Run query:".to_string());

        while let Some(page_response) = pager.next().await {
            let page = page_response?.into_body().await?;
            for item in page.items {
                callback(serde_json::to_string_pretty(&item).unwrap());
            }
        }
    }
    
    callback("Current Status:\tStopping...".to_string());

    Ok(())
}