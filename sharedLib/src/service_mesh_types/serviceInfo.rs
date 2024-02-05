use chrono::{Utc, DateTime, TimeZone};
use super::serviceAdapters::ServiceAdapters;
use serde_derive::{Deserialize, Serialize};
use sqlx::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct ServiceInfo {
  #[sqlx(rename = "id")]
    pub id:String,
    pub service_type :String,
    pub  update_time:i64,
  //  Adapters: Vec<ServiceAdapters>,
  }