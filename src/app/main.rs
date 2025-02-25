use dotenv;
use socketioxide::SocketIo;
use socketioxide::extract::SocketRef;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

mod item;
mod cosmos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", |socket: SocketRef| {        
        println!("Socket connected: {:?}", socket.id);
        socket.on("start", move |socket: SocketRef| {
            let handle_message = move |message: String| {                        
                match socket.emit("new_message", &message) {
                    Ok(_) => {
                        println!("{}", message);
                    }
                    Err(e) => {
                        println!("Failed to emit new message: {:?}", e);
                    }
                }
            };

            tokio::spawn(async move {
                match cosmos::run(
                    std::env::var("CONFIGURATION__AZURECOSMOSDB__ENDPOINT").expect("Missing CONFIGURATION__AZURECOSMOSDB__ENDPOINT environment variable."),
                    std::env::var("CONFIGURATION__AZURECOSMOSDB__DATABASENAME").expect("Missing CONFIGURATION__AZURECOSMOSDB__DATABASENAME environment variable."),
                    std::env::var("CONFIGURATION__AZURECOSMOSDB__CONTAINERNAME").expect("Missing CONFIGURATION__AZURECOSMOSDB__CONTAINERNAME environment variable."),
                    handle_message,
                ).await {
                    Ok(_) => (),
                    Err(e) => println!("Error running Azure Cosmos DB for NoSQL script: {:?}", e),
                }
            });
        });
    });

    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let address = "0.0.0.0:3030".parse::<SocketAddr>().unwrap();

    let listener = TcpListener::bind(address).await.unwrap();

    println!("Listening on {}", address);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}