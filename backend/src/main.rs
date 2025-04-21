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

    // 모든 인터페이스에서 수신 가능하도록 수정
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Lite Wiki API running at http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}