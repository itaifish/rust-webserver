
use tokio::net::TcpListener;
use crate::webserver::router;

#[tokio::main]
pub async fn main() {
    let router = router::init_router();
    let listener = init_listener().await;
    axum::serve(listener, router).await.unwrap();
}


async fn init_listener() -> TcpListener {
    TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap()
}

