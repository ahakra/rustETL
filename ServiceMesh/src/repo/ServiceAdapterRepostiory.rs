use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use sqlx::postgres::PgPool;
use chrono::{Utc, NaiveDateTime};

#[derive(Debug)]
pub struct ServiceAdapterRepostiory {
    pool: PgPool,
}

impl ServiceAdapterRepostiory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

}