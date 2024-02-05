use sharedLib::service_mesh_types::serviceInfo::ServiceInfo;
use sqlx::postgres::PgPool;
use chrono::{Utc, NaiveDateTime};

use crate::traits::repository::ServiceInfoRepositoryTrait;

#[derive(Debug,Clone)]
pub struct ServiceInfoRepository {
    pool: PgPool,
}

impl ServiceInfoRepositoryTrait for ServiceInfoRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

     async fn create_service_info(&self, service: &ServiceInfo) -> Result<(), sqlx::Error> {
      
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

     async fn get_service_info_by_id(&self, id: String) -> Result<Option<ServiceInfo>, sqlx::Error> {
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
    
     async fn get_service_info_by_type(
        &self,
        service_type: String,
    ) -> Result<Option<Vec<ServiceInfo>>, sqlx::Error> {
        let result: Vec<ServiceInfo> = sqlx::query_as!(
            ServiceInfo,
            r#"
            SELECT id, service_type, update_time
            FROM service_info
            WHERE service_type = $1
            "#,
            service_type
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(Some(result))
    }
    
     async fn update_service_info_health(&self,id:&str) -> Result<(), sqlx::Error> {
        let current_timestamp: NaiveDateTime = Utc::now().naive_utc();
    
        sqlx::query!(
            r#"
            UPDATE service_info
            SET update_time = $1
            where id = $2
            "#,
            current_timestamp.timestamp()
            ,id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }

     async fn delete_service_info(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM service_info
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
    // pub async fn update_service_info(
    //     pool: &sqlx::PgPool,
    //     id: &str,
    //     new_service_info: &ServiceInfo,
    // ) -> Result<(), sqlx::Error> {
        
    //     sqlx::query!(
    //         r#"
    //         UPDATE service_info
    //         SET service_type = $1, update_time = $2
    //         WHERE id = $3
    //         "#,
    //         new_service_info.service_type,
    //         new_service_info.update_time,
    //         id
    //     )
    //     .execute(pool)
    //     .await?;
    
    //     Ok(())
    // }
    // Implement other CRUD operations as needed
}
