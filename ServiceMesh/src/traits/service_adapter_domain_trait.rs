

pub trait ServiceAdapterDomainTrait<T> {
    fn new(repo: T) -> Self ;
}