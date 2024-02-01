pub trait RepoManagerTrait<T,K> {
     fn new(service_info_repository_trait: T,service_adapter_repository_trait :K) -> Self;
}
