use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MongoDbConfig {
    #[serde(rename = "uri")]
    uri: String,
    #[serde(rename = "database")]
    database: String,
    #[serde(rename = "username")]
    username: String,
    #[serde(rename = "password")]
    password: String,
}