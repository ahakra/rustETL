pub mod repo;
pub mod traits;
pub mod domain;
use std::{env, str::FromStr};
use tokio;




use sqlx::postgres::PgConnectOptions;

use crate::{domain::info::ServiceInfoDomain, traits:: {
                     repository::ServiceAdapterRepositoryTrait, domain::ServiceInfoDomainTrait, repository::ServiceInfoRepositoryTrait}};
#[tokio::main]
async fn main()  {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment"); 
	println!("database_url {}", database_url);
    
    let options = PgConnectOptions::from_str(&database_url).unwrap();

    // Create a connection pool using the specified options
    let pool = sqlx::PgPool::connect_with(options).await.unwrap();
    let new_service_info_repo = repo::info::ServiceInfoRepository::new(pool.clone());
    let new_service_adapter_repo = repo::adapters::ServiceAdapterRepostiory::new(pool.clone());
 
    let serviceInfodomain = ServiceInfoDomain::new(new_service_info_repo);
    let xx = serviceInfodomain.get_service_info_by_id("aa".to_string()).await;
    println!("{:?}",xx);
 }