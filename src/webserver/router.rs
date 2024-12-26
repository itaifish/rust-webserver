use axum::Router;
use axum::routing::{any, get};

pub fn init_router() -> Router {
	Router::new()
		.route("/", get(hello_world))
		//.route("/ws", any(ws_handler))
}

async fn hello_world() -> &'static str {
	"Hello world!"
}
