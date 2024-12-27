use crate::startup::load_environments;

mod webserver;
mod startup;
mod models;

#[tokio::main]
async fn main() {
    let envs = load_environments();
    webserver::start::main(&envs)
        .await
        .unwrap();
}