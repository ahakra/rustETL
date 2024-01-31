pub mod repo;
use std::env;

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
fn main()  {
    
    let database_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL in the environment"); 
	
     // Create a connection pool with the specified database URL
    //  let pool = PgPoolOptions::new()
    //      .max_connections(5)
    //      .connect(database_url)
    //      ;
 
    // Create PostgreSQL connection options with the specified database URL
    let options = PgConnectOptions::new()
        .host("localhost")
        .port(5432)
        .database("database_name")
        .username("username")
        .password("password");

    // Create a connection pool using the specified options
    let pool = sqlx::PgPool::connect_with(options);
   
 }