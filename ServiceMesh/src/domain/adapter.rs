use sharedLib::service_mesh_types::serviceAdapters::ServiceAdapters;

use crate:: traits::{domain::ServiceAdapterDomainTrait, repository::ServiceAdapterRepositoryTrait};

pub struct ServiceAdapterDomain<T: ServiceAdapterRepositoryTrait> {
    repo: T,
}

impl <T>ServiceAdapterDomainTrait<T> for  ServiceAdapterDomain<T> where T :ServiceAdapterRepositoryTrait {
     fn new(repo: T) -> Self {
        Self { repo }
    }

    async fn create_service_adapter(&self, service: &ServiceAdapters) -> Result<(), sqlx::Error> {
       self.repo.create_service_adapter(service).await
    }

    async fn get_service_adapter_by_id(&self, id: String) -> Result<Option<ServiceAdapters>, sqlx::Error> {
        self.repo.get_service_adapter_by_id(id).await
    }

    async fn get_service_adapter_by_service_info_id(
        &self,
        service_info_id: String,
    ) -> Result<Option<Vec<ServiceAdapters>>, sqlx::Error>{
       self.repo.get_service_adapter_by_service_info_id(service_info_id).await
    }

    async fn delete_service_adapter(   &self, id: &str) -> Result<(), sqlx::Error> {
        self.repo.delete_service_adapter(id).await
    }
    async fn delete_service_adapter_by_service_info_id(   &self, id: &str) -> Result<(), sqlx::Error> {
        self.repo.delete_service_adapter_by_service_info(&id).await
    }
}