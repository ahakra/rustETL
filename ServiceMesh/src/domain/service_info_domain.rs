use crate:: traits::{service_info_repository_traits::ServiceInfoRepositoryTrait, service_info_domain_trait::ServiceInfoDomainTrait};

pub struct ServiceInfoDomain<T: ServiceInfoRepositoryTrait> {
    repo: T,
}

impl <T>ServiceInfoDomainTrait<T> for  ServiceInfoDomain<T> where T :ServiceInfoRepositoryTrait {
    fn new(repo: T) -> Self {
        
        Self { repo }

    }

}
