pub trait ServiceInfoRepositoryImpl {
    fn new(pool: PgPool) -> Self;
    fn create_service_info(&self, service: &ServiceInfo) -> Result<(), sqlx::Error>;
    fn get_service_info_by_id(&self, id: String) -> Result<Option<ServiceInfo>, sqlx::Error>;
    fn get_service_info_by_type(
        &self,
        service_type: String,
    ) -> Result<Option<Vec<ServiceInfo>>, sqlx::Error>;
    fn update_service_info_health(&self) -> Result<(), sqlx::Error>;
    fn delete_service_info(&self, id: &str) -> Result<(), sqlx::Error>;
}