use crate::traits::{repo_manager_trait::RepoManagerTrait, 
            service_adapter_repository_trait::ServiceAdapterRepositoryTrait,
            service_info_repository_traits::ServiceInfoRepositoryTrait
            };

pub struct RepoManager<T: ServiceInfoRepositoryTrait, K: ServiceAdapterRepositoryTrait> {
    service_info_repository: T,
    service_adapter_repository:K,
}

impl<T, K> RepoManagerTrait<T, K> for RepoManager<T, K>
where
    T: ServiceInfoRepositoryTrait,
    K: ServiceAdapterRepositoryTrait,
    {
     fn new(service_info_repository: T,service_adapter_repository :K) -> Self {
        Self { service_info_repository,service_adapter_repository }
    }
  
}

