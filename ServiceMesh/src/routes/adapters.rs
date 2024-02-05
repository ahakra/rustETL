
use warp::http::StatusCode;

use crate::domain::adapter::ServiceAdapterDomain;
use crate::repo::adapters::ServiceAdapterRepostiory;

use crate::traits::domain::ServiceAdapterDomainTrait;
use  super::super::helpers::errors;


pub async fn create_service_adapter(
    service_adapter :ServiceAdapterDomain<ServiceAdapterRepostiory>,
    service :sharedLib::service_mesh_types::serviceAdapters::ServiceAdapters,

) -> Result<impl warp::Reply, warp::Rejection> {
    let create_service =service_adapter.create_service_adapter(&service).await;

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

pub async fn get_adapter_by_id(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_adapter_by_id(id).await;
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

pub async fn delete_service_adapter(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.delete_service_adapter(&id).await;
    match result {
        Ok(data) => {
            Ok(warp::reply::with_status("adapter deleted", StatusCode::OK))
        }
        Err(_) => {
            // Handle the error case
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
    
}

pub async fn delete_service_adapter_by_info_id(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.delete_service_adapter_by_service_info_id(&id).await;
    match result {
        Ok(data) => {
            Ok(warp::reply::with_status("adapters deleted", StatusCode::OK))
        }
        Err(_) => {
            // Handle the error case
            Err(warp::reject::custom(errors::Error::SQLError))
        }
    }
}

pub async fn get_service_adapter_by_info_id(
    id: String,
    service :ServiceAdapterDomain<ServiceAdapterRepostiory>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let result = service.get_service_adapter_by_service_info_id(id).await;
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