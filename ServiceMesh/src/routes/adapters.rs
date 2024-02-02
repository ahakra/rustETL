
use warp::http::StatusCode;

use crate::domain::adapter::ServiceAdapterDomain;
use crate::repo::adapters::ServiceAdapterRepostiory;

use crate::traits::domain::ServiceAdapterDomainTrait;

pub async fn create_service_adapter(
    service_adapter :ServiceAdapterDomain<ServiceAdapterRepostiory>,
    service :sharedLib::serviceMeshTypes::serviceAdapters::ServiceAdapters,

) -> Result<impl warp::Reply, warp::Rejection> {
    let create_service =service_adapter.create_service_adapter(&service).await;

    Ok(warp::reply::json(
        &create_service.unwrap()
        ))
}

pub async fn get_adapter_by_id(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_adapter_by_id(id).await;
    Ok(warp::reply::json(
        &result.unwrap()
        ))
}

pub async fn delete_service_adapter(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.delete_service_adapter(&id).await;
    Ok(warp::reply::with_status("adapter deleted", StatusCode::OK))
}

pub async fn delete_service_adapter_by_info_id(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.delete_service_adapter_by_service_info_id(&id).await;
    Ok(warp::reply::with_status("adapters deleted", StatusCode::OK))
}

pub async fn get_service_adapter_by_info_id(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_adapter_by_service_info_id(id).await;
    Ok(warp::reply::json(
        &result.unwrap()
        ))
    }