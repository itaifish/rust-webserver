
use tokio::net::TcpListener;
use log::{error, info};
use crate::webserver::{listener, router};

#[tokio::main]
pub async fn main() {
    colog::init();
    let router = router::init_router();
    let listener = listener::init_listener().await;
    info!("Server started on port 3000 🚀 ");
    axum::serve(listener, router).await.unwrap_or({
        error!("Server encountered an unexpected error");
    });
}




