use crate:: traits::{repository::ServiceInfoRepositoryTrait, domain::ServiceInfoDomainTrait};
use sharedLib::service_mesh_types::serviceInfo::ServiceInfo;

pub struct ServiceInfoDomain<T: ServiceInfoRepositoryTrait> {
    repo: T,
}

impl <T>ServiceInfoDomainTrait<T> for  ServiceInfoDomain<T> where T :ServiceInfoRepositoryTrait {
    fn new(repo: T) -> Self {
        
        Self { repo }

    }

    async fn create_service_info(&self, service: &ServiceInfo) -> Result<(), sqlx::Error> {
        self.repo.create_service_info(service).await
    }

    async fn get_service_info_by_id(&self, id: String) -> Result<Option<sharedLib::service_mesh_types::serviceInfo::ServiceInfo>, sqlx::Error> {
       self.repo.get_service_info_by_id(id).await
    }

    async  fn get_service_info_by_type(
       &self,
       service_type: String,
       ) -> Result<Option<Vec<ServiceInfo>>, sqlx::Error> {
        self.repo.get_service_info_by_type(service_type).await
    }

    async  fn update_service_info_health(&self,id:&str) -> Result<(), sqlx::Error> {
        self.repo.update_service_info_health(id).await
    }

    async  fn delete_service_info(&self, id: &str) -> Result<(), sqlx::Error> {
        self.repo.delete_service_info(id).await
    }

}
