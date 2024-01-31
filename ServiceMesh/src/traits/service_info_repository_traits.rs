use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use sqlx::postgres::PgPool;
pub trait ServiceInfoRepositoryTrait {
    fn new(pool: PgPool) -> Self;
     async fn create_service_info(&self, service: &ServiceInfo) -> Result<(), sqlx::Error>;
     async fn get_service_info_by_id(&self, id: String) -> Result<Option<ServiceInfo>, sqlx::Error>;
     async  fn get_service_info_by_type(
        &self,
        service_type: String,
    ) -> Result<Option<Vec<ServiceInfo>>, sqlx::Error>;
     async  fn update_service_info_health(&self,) -> Result<(), sqlx::Error>;
     async  fn delete_service_info(&self, id: &str) -> Result<(), sqlx::Error>;
}