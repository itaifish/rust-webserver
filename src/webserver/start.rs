use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;
use log::{error, info};
use crate::models::app_state::AppState;
use crate::models::environment_vars::EnvironmentVariables;
use crate::webserver::{listener, router};

pub async fn main(env: EnvironmentVariables) -> std::io::Result<()> {
    colog::init();
    let router = router::init_router(&env);
    let listener = listener::init_listener(&env).await;
    info!("Server started with address {}:{} 🚀", env.host_address, env.port);
    let res = axum::serve(listener, router).await;
    info!("Server has stopped with graceful shutdown");
    res
}




