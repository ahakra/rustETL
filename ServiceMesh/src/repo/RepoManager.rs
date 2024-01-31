use crate::traits::ServiceInfoRepostoryTrait::ServiceInfoRepositoryImpl;
use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use sqlx::postgres::PgPool;
pub struct RepoManager<T: ServiceInfoRepositoryImpl> {
    repository: T,
}

impl<T: ServiceInfoRepositoryImpl> RepoManager<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
    pub async fn get_service_info_by_id(&self, id: String) -> Result<Option<ServiceInfo>, sqlx::Error> {
        self.repository.get_service_info_by_id(id).await
    }
}