
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceInfo {
  
    pub  id:String,
    pub  service_name:String,
    pub  service_type :String,
    pub  update_time:i64,
  //  Adapters: Vec<ServiceAdapters>,
  }