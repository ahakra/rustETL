use warp::http::StatusCode;
use shared_lib::service_mesh_types::service_info::ServiceInfo;
use crate::domain::adapter::ServiceAdapterDomain;
use crate::domain::info::ServiceInfoDomain;
use crate::repo::adapters::ServiceAdapterRepostiory;
use crate::repo::info::ServiceInfoRepository;
use crate::traits::domain::{ServiceAdapterDomainTrait, ServiceInfoDomainTrait};

use  super::super::helpers::errors;

pub async fn get_info_by_id(
    id: String,
    service :ServiceInfoDomain<ServiceInfoRepository>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_info_by_id(id).await;
    match result {
        Ok(data) => {
            Ok(warp::reply::json(&data))
        }
        Err(_) => {
            // Handle the error case
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
}

pub async fn get_info_by_type(
    service_type: String,
    service :ServiceInfoDomain<ServiceInfoRepository>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_info_by_type(service_type).await;
    match result {
        Ok(data) => {
            Ok(warp::reply::json(&data))
        }
        Err(_) => {
            // Handle the error case
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
   
}

pub async fn update_info_health(
    id:String,
    service :ServiceInfoDomain<ServiceInfoRepository>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.update_service_info_health(&id).await;
    
    match result {
        Ok(_data) => {
            Ok(warp::reply::with_status("health updated", StatusCode::OK))
        }
        Err(_) => {
            // Handle the error case
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
}

pub async fn delete_service_info(
    id :String,
    service_info :ServiceInfoDomain<ServiceInfoRepository>,
    service_adapter :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
   let delet_infoersult =service_info.delete_service_info(&id).await;
    let _service_adapters = service_adapter.delete_service_adapter_by_service_info_id(&id).await;

    match delet_infoersult {
        Ok(_data) => {
            Ok(warp::reply::with_status("service deleted", StatusCode::OK))
        }
        Err(_) => {
            // Handle the error case
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
}



pub async fn create_service_info(
    service_info :ServiceInfoDomain<ServiceInfoRepository>,
    service :ServiceInfo,

) -> Result<impl warp::Reply, warp::Rejection> {
    let create_service =service_info.create_service_info(&service).await;

    match create_service {
        Ok(result) => Ok(warp::reply::json(&result)),
        Err(error) => {
            if let Some(database_error) = error.as_database_error() {
                let error_code = database_error.code();
                
                if let Some(error_code) = error_code {
                    if error_code == "23505" {
                        return Err(warp::reject::custom(errors::Error::DuplicateKey));
                    }
                }
            }
            // If it's not the specific error code, reject with the original error
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
    
   
}
