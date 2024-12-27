use serde::{Deserialize, Deserializer};

#[derive(Debug)]
#[derive(Deserialize)]
pub struct EnvironmentVariables {
    port: u16,
    host_address: String
}