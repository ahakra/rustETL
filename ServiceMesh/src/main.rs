pub mod repo;
pub mod traits;
pub mod domain;
pub mod routes;
pub mod helpers;
use std::{env, str::FromStr};
use tokio;
use warp::{cors, Filter};
use sqlx::postgres::PgConnectOptions;
use warp::http::Method;


use crate::{domain::info::ServiceInfoDomain, helpers::errors, traits:: {
                       repository::ServiceAdapterRepositoryTrait,
                       domain::ServiceInfoDomainTrait,
                       repository::ServiceInfoRepositoryTrait}
                    };
use crate::domain::adapter::ServiceAdapterDomain;
use crate::traits::domain::ServiceAdapterDomainTrait;

#[tokio::main]
async fn main() {


    let cors = warp::cors()
            .allow_any_origin()
            .allow_header("content-type")
            .allow_methods(
            &[Method::PUT, Method::DELETE, Method::GET, Method::POST]
            );


    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment");
    println!("database_url {}", database_url);

    let options = PgConnectOptions::from_str(&database_url).unwrap();

    // Create a connection pool using the specified options
    let pool = sqlx::PgPool::connect_with(options).await.unwrap();
    let new_service_info_repo = repo::info::ServiceInfoRepository::new(pool.clone());
    let new_service_adapter_repo = repo::adapters::ServiceAdapterRepostiory::new(pool.clone());

    let service_adapter_domain = warp::any().map(move || ServiceAdapterDomain::new(new_service_adapter_repo.clone()));
    let service_info_domain = warp::any().map(move || ServiceInfoDomain::new(new_service_info_repo.clone()));





    let info_prefix = warp::path("info");
    let adapter_prefix = warp::path("adapter");

    let get_info_by_id = warp::get()
        .and(info_prefix.clone().and(warp::path("id")))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(service_info_domain.clone())
        .and_then(crate::routes::info::get_info_by_id);//.recover(errors::return_error);

    let get_info_by_type = warp::get()
        .and(info_prefix.clone().and(warp::path("type")))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(service_info_domain.clone())
        .and_then(crate::routes::info::get_info_by_type);

    let update_info_health = warp::post()
        .and(info_prefix.clone().and(warp::path("health")))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
       
        .and(service_info_domain.clone())
        .and_then(crate::routes::info::update_info_health);//.recover(errors::return_error);

    let delete_service_info = warp::delete()
        .and(info_prefix.and(warp::path::param::<String>()))
        .and(warp::path::end())
        .and(service_info_domain.clone())
        .and(service_adapter_domain.clone())
        .and_then(crate::routes::info::delete_service_info);//.recover(errors::return_error);

    let create_service_info = warp::post()
        .and(info_prefix.clone())
        .and(warp::path::end())
        .and(service_info_domain.clone())
        .and(warp::body::json())
        .and_then(crate::routes::info::create_service_info).recover(errors::return_error);
    
   


  let info_routes = get_info_by_id
                                                    .or(get_info_by_type)
                                                    .or(update_info_health)
                                                    .or(delete_service_info)
                                                    .or(create_service_info);

    
    let create_service_adapter = warp::post()
    .and(adapter_prefix.clone())
    .and(warp::path::end())
    .and(service_adapter_domain.clone())
    .and(warp::body::json())
    .and_then(crate::routes::adapters::create_service_adapter);//.recover(errors::return_error);

    let get_adapter_by_id = warp::get()
    .and(adapter_prefix.clone().and(warp::path("id")))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(service_adapter_domain.clone())
    .and_then(crate::routes::adapters::get_adapter_by_id);//.recover(errors::return_error);

    let delete_service_adapter = warp::delete()
    .and(adapter_prefix.and(warp::path::param::<String>()))
    .and(warp::path::end())
    .and(service_adapter_domain.clone())
    .and_then(crate::routes::adapters::delete_service_adapter).recover(errors::return_error);

    
    let delete_service_adapter_by_service_info = warp::delete()
    .and(adapter_prefix.clone().and(warp::path("info")))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(service_adapter_domain.clone())
    .and_then(crate::routes::adapters::delete_service_adapter_by_info_id);//.recover(errors::return_error);

      
    let get_service_adapter_by_service_info = warp::get()
    .and(adapter_prefix.clone().and(warp::path("info")))
    .and(warp::path::param::<String>())
    .and(warp::path::end())
    .and(service_adapter_domain.clone())
    .and_then(crate::routes::adapters::get_service_adapter_by_info_id);//.recover(errors::return_error);


    let adapter_routes = create_service_adapter
                        .or(get_adapter_by_id)
                        .or(delete_service_adapter)
                        .or(delete_service_adapter_by_service_info)
                        .or(get_service_adapter_by_service_info);

  let routes = info_routes.or(adapter_routes).with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

}

