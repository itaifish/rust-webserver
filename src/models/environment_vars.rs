use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct EnvironmentVariables {
    pub port: u16,
    pub host_address: String
}