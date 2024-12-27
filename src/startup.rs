use config::Config;
use crate::models::environment_vars::EnvironmentVariables;

pub fn load_environments() -> EnvironmentVariables {
    Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap()
        .try_deserialize::<EnvironmentVariables>()
        .unwrap()
}