use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use sqlx::postgres::PgPool;


#[derive(Debug)]
pub struct ServiceRepository {
    pool: PgPool,
}

impl ServiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_service(&self, service: &ServiceInfo) -> Result<(), sqlx::Error> {
      
        sqlx::query!(
            r#"
            INSERT INTO service_info (id, service_type, update_time)
            VALUES ($1, $2, $3)
            "#,
            service.id,
            service.service_type,
            service.update_time
           
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_service_by_id(&self, id: String) -> Result<Option<ServiceInfo>, sqlx::Error> {
        let result = sqlx::query_as!(
            ServiceInfo,
            r#"
            SELECT id, service_type, update_time
            FROM service_info
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result)
    }

    // Implement other CRUD operations as needed
}
