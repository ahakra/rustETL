use reqwest::{self, Error, Response};

use shared_lib::service_mesh_types::service_info::ServiceInfo;
pub async fn register(){
    let client = reqwest::Client::new();
  
    let info = ServiceInfo{
        id: "13336663313".to_string(),
        service_name:"CDR".to_string(),
        service_type: "DataSource".to_string(),
        update_time:1707202588,
          
          
    };
    let res = client.post("http://localhost:3030/info")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&info)
        .send()
        .await;

        match res {
            Ok(response) => {
                println!("Register Response: {:?}", response.status());
            }
            Err(err) => {
                eprintln!("Request error for register: {:?}", err);
            }
        }
}

pub async fn update_health() -> Result<Response,Error>{
    let info = ServiceInfo{
        id: "13336663313".to_string(),
        service_name:"CDR".to_string(),
        service_type: "DataSource".to_string(),
        update_time:1707202588,
          
          
    };
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3030/info/health/13336663313")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .json(&info)
    
    .send()
    .await;
    res
    

   
}


pub async fn delete(){
    let info = ServiceInfo{
        id: "13336663313".to_string(),
        service_name:"CDR".to_string(),
        service_type: "DataSource".to_string(),
        update_time:1707202588,
          
          
    };
    let client = reqwest::Client::new();
    let res = client.delete("http://localhost:3030/info/13336663313")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .json(&info)
    .send()
    .await;

   match res {
       Ok(response) => {
        println!("Delete Response: {:?}", response.status());
       }
       Err(err) => {
           eprintln!("Request error for delete: {:?}", err);
       }
   }
}