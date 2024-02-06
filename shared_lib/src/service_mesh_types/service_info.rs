
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct ServiceInfo {
  #[sqlx(rename = "id")]
    pub id:String,
    pub service_type :String,
    pub  update_time:i64,
  //  Adapters: Vec<ServiceAdapters>,
  }