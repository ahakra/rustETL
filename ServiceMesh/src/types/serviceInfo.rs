use chrono::{Utc, DateTime, TimeZone};
use super::ServiceAdapters::ServiceAdapters;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceInfo {
    Id:String,
    ServiceType :String,
    UpdateTime:i64,
    Adapters: Vec<ServiceAdapters>,
  }