mod routes;
mod handlers;
mod storage;
mod utils;
mod config;

use tokio::net::TcpListener;
use tracing_subscriber;
use dotenv::dotenv;

use routes::create_routes;
use utils::check_environment_directories;

#[tokio::main]
async fn main() {
    dotenv().ok();
    check_environment_directories();

    tracing_subscriber::fmt::init();

    let app = create_routes();

    // 모든 인터페이스에서 수신 가능하도록 수정
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Lite Wiki API running at http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}