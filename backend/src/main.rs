mod routes;
mod handlers;
mod storage;

use tokio::net::TcpListener;
use tracing_subscriber;
use routes::create_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_routes();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Lite Wiki API running at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}