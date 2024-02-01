pub mod repo;
pub mod traits;
pub mod domain;
use std::{env, str::FromStr};
use tokio;




use sqlx::postgres::PgConnectOptions;

use crate::traits:: {repo_manager_trait::RepoManagerTrait,
                     service_adapter_repository_trait::ServiceAdapterRepositoryTrait,
                     service_info_repository_traits::ServiceInfoRepositoryTrait};
#[tokio::main]
async fn main()  {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment"); 
	println!("database_url {}", database_url);
    
    let options = PgConnectOptions::from_str(&database_url).unwrap();

    // Create a connection pool using the specified options
    let pool = sqlx::PgPool::connect_with(options).await.unwrap();
    let new_service_info_repo = repo::ServiceInfoRepository::ServiceInfoRepository::new(pool.clone());
    let new_service_adapter_repo = repo::ServiceAdapterRepostiory::ServiceAdapterRepostiory::new(pool.clone());
   // let service = new_service_repo.get_service_info_by_id("aa".to_string()).await.unwrap();
   // println!("{:?}", service);

    
    let manager = repo::RepoManager::RepoManager::new(new_service_info_repo,new_service_adapter_repo) ;
   // let r = manager.get_service_info_by_id("aa".to_string()).await.unwrap();
   // println!("{:?}", r);
 }