pub mod repo;
pub mod traits;
use std::{env, str::FromStr};
use tokio;
use dotenv::dotenv;



use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::traits::ServiceInfoRepostoryTrait::ServiceInfoRepositoryImpl;
#[tokio::main]
async fn main()  {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment"); 
	println!("database_url {}", database_url);
    
    let options = PgConnectOptions::from_str(&database_url).unwrap();

    // Create a connection pool using the specified options
    let pool = sqlx::PgPool::connect_with(options).await.unwrap();
    let new_service_repo = repo::ServiceInfoRepository::ServiceInfoRepository::new(pool);
   // let service = new_service_repo.get_service_info_by_id("aa".to_string()).await.unwrap();
   // println!("{:?}", service);

   // let repository = ServiceInfoRepositoryImpl::new(pool);
    let manager = repo::RepoManager::RepoManager::new(new_service_repo) ;
    let r = manager.get_service_info_by_id("aa".to_string()).await.unwrap();
    println!("{:?}", r);
 }