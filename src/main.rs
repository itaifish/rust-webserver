use crate::startup::load_environments;
use crate::models::app_state::AppState;

mod webserver;
mod startup;
mod models;

#[tokio::main]
async fn main() {
    let envs = load_environments();
    webserver::start::main(envs)
        .await
        .unwrap();
}