use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;
use log::{error, info};
use crate::models::environment_vars::EnvironmentVariables;
use crate::webserver::{listener, router};

pub async fn main(environment_variables: &EnvironmentVariables) -> std::io::Result<()> {
    colog::init();
    let router = router::init_router();
    let listener = listener::init_listener(environment_variables).await;
    info!("Server started with address {}:{} 🚀", environment_variables.host_address, environment_variables.port);
    let res = axum::serve(listener, router).await;
    info!("Server has stopped with graceful shutdown");
    res
}




