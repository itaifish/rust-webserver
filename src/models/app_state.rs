use crate::models::environment_vars::EnvironmentVariables;

#[derive(Clone)]
pub struct AppState {
    pub(crate) environment: EnvironmentVariables
}