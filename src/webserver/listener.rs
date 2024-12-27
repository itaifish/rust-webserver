use tokio::net::TcpListener;

pub(crate) async fn init_listener() -> TcpListener {
    TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap()
}