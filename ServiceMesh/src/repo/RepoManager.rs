use crate::traits::{service_adapter_repository_trait::ServiceAdapterRepositoryTrait, service_info_repository_traits::ServiceInfoRepositoryTrait};
use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use sqlx::postgres::PgPool;
pub struct RepoManager<T: ServiceInfoRepositoryTrait, K: ServiceAdapterRepositoryTrait> {
    service_info_repository: T,
    service_adapter_repository:K,
}

impl<T: ServiceInfoRepositoryTrait,K:ServiceAdapterRepositoryTrait> RepoManager<T,K> {
    pub fn new(service_info_repository: T,service_adapter_repository :K) -> Self {
        Self { service_info_repository,service_adapter_repository }
    }
    pub async fn get_service_info_by_id(&self, id: String) -> Result<Option<ServiceInfo>, sqlx::Error> {
        self.service_info_repository.get_service_info_by_id(id).await
    }
}