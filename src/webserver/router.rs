use axum::extract::State;
use axum::Router;
use axum::routing::{any, get};
use crate::models::app_state::AppState;
use crate::models::environment_vars::EnvironmentVariables;

pub fn init_router(environment_variables: &EnvironmentVariables) -> Router {
	let state = AppState {
		environment: environment_variables.clone()
	};
	Router::new()
		.route("/", get(hello_world))
		.with_state(state)
		//.route("/ws", any(ws_handler))
}

async fn hello_world(state: State<AppState>) -> String {
	format!("Hello world! @ port {}", state.environment.port)
}
