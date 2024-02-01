use crate:: traits::{service_adapter_domain_trait::ServiceAdapterDomainTrait, service_adapter_repository_trait::ServiceAdapterRepositoryTrait};

pub struct ServiceAdapterDomain<T: ServiceAdapterRepositoryTrait> {
    repo: T,
}

impl <T>ServiceAdapterDomainTrait<T> for  ServiceAdapterDomain<T> where T :ServiceAdapterRepositoryTrait {
    fn new(repo: T) -> Self {
        Self { repo }
    }
}