use lite_wiki_backend::config::BASE_URL;
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;
use dotenvy::dotenv;

use lite_wiki_backend::routes::create_routes;
use lite_wiki_backend::utils::check_environment_directories;

#[tokio::main]
async fn main() {
    // Load environment data from .env
    dotenv().ok();

    // Ensure required data path exists
    check_environment_directories();

    // Initialize tracing subscriber (logger)
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Create the application routes
    let app = create_routes();

    // Bind to address
    let bind_address = "0.0.0.0:3000";
    let base_url = BASE_URL.clone();

    match TcpListener::bind(&bind_address).await {
        Ok(listner) => {
            info!("Lite Wiki is running at http://{}", &base_url);
            if let Err(e) = axum::serve(listner, app).await {
                error!("Server error: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to bind to address {}: {}", &bind_address, e);
        }
    }
}