use serde::{Deserialize, Deserializer};

#[derive(Debug)]
#[derive(Deserialize)]
pub struct EnvironmentVariables {
    pub port: u16,
    pub host_address: String
}