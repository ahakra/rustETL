
use sharedLib::serviceMeshTypes::ServiceAdapters::ServiceAdapters;

pub trait ServiceAdapterDomainTrait<T> {
    fn new(repo: T) -> Self ;
    async fn create_service_adapter(&self, service: &ServiceAdapters) -> Result<(), sqlx::Error>;

    async fn get_service_adapter_by_id(&self, id: String) -> Result<Option<ServiceAdapters>, sqlx::Error>;

    async fn get_service_adapter_by_service_info_id(
        &self,
        service_info_id: String,
    ) -> Result<Option<Vec<ServiceAdapters>>, sqlx::Error>;

    async fn delete_service_adapter(   &self, id: &str) -> Result<(), sqlx::Error>;
}