use tokio::net::TcpListener;
use crate::models::environment_vars::EnvironmentVariables;

pub async fn init_listener(environment_variables: &EnvironmentVariables) -> TcpListener {
    TcpListener::bind(format!("{}:{}", environment_variables.host_address, environment_variables.port))
        .await
        .unwrap()
}