use serde::forward_to_deserialize_any;
use warp::http::StatusCode;
use sharedLib::serviceMeshTypes::serviceAdapters::ServiceAdapters;
use sharedLib::serviceMeshTypes::serviceInfo::ServiceInfo;
use crate::domain::adapter::ServiceAdapterDomain;
use crate::domain::info::ServiceInfoDomain;
use crate::repo::adapters::ServiceAdapterRepostiory;
use crate::repo::info::ServiceInfoRepository;
use crate::traits::domain::{ServiceAdapterDomainTrait, ServiceInfoDomainTrait};

pub async fn get_info_by_id(
    id: String,
    service :ServiceInfoDomain<ServiceInfoRepository>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_info_by_id(id).await;
    Ok(warp::reply::json(
        &result.unwrap()
        ))
}

pub async fn get_info_by_type(
    service_type: String,
    service :ServiceInfoDomain<ServiceInfoRepository>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_info_by_type(service_type).await;
    Ok(warp::reply::json(
        &result.unwrap()
        ))
}

pub async fn update_info_health(
    id:String,
    service :ServiceInfoDomain<ServiceInfoRepository>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.update_service_info_health(&id).await;
    Ok(warp::reply::with_status("Service health updated", StatusCode::OK))
}

pub async fn delete_service_info(
    id :String,
    service_info :ServiceInfoDomain<ServiceInfoRepository>,
    service_adapter :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
   let deletInfoersult =service_info.delete_service_info(&id).await;
    let serviceAdapters = service_adapter.delete_service_adapter_by_service_info_id(&id).await;

    Ok(warp::reply::with_status("Service and its adapters are deleted", StatusCode::OK))
}



pub async fn create_service_info(
    service_info :ServiceInfoDomain<ServiceInfoRepository>,
    service :ServiceInfo,

) -> Result<impl warp::Reply, warp::Rejection> {
    let create_service =service_info.create_service_info(&service).await;

    Ok(warp::reply::json(
        &create_service.unwrap()
        ))
}
