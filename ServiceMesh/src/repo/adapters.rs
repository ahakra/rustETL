use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use sharedLib::serviceMeshTypes::ServiceAdapters::ServiceAdapters;
use sharedLib::serviceMeshTypes::protocolUsed::ProtocolUsed;
use sqlx::postgres::PgPool;
use chrono::{Utc, NaiveDateTime};
use crate::traits::repository::ServiceAdapterRepositoryTrait;

#[derive(Debug)]
pub struct ServiceAdapterRepostiory {
    pool: PgPool,
}

impl ServiceAdapterRepositoryTrait for ServiceAdapterRepostiory {
     fn new(pool: PgPool) -> Self {
        Self { pool }
    }
     async fn create_service_adapter(&self, service: &ServiceAdapters) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO service_adapter (id, address, protocol_used, protocol_description, service_info_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            service.id,
            service.address,
            Into::<i32>::into(service.protocol_used.clone()),
            service.protocol_description,
            service.service_info_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    

    async fn get_service_adapter_by_id(&self, id: String) -> Result<Option<ServiceAdapters>, sqlx::Error> {
        let result = sqlx::query_as!(
            ServiceAdapters,
            r#"
            SELECT id, address, protocol_used, protocol_description, service_info_id
            FROM service_adapter
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    async fn get_service_adapter_by_service_info_id(
        &self,
        service_info_id: String,
    ) -> Result<Option<Vec<ServiceAdapters>>, sqlx::Error> {
        let result: Vec<ServiceAdapters> = sqlx::query_as!(
            ServiceAdapters,
            r#"
            SELECT id, address, protocol_used , protocol_description, service_info_id
            FROM service_adapter
            WHERE service_info_id = $1
            "#,
            service_info_id
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(Some(result))
    }

    async fn delete_service_adapter(   &self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM service_adapter
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
}