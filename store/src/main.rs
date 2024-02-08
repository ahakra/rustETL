use std::collections::HashMap;

use shared_lib::record_mapping_types::field_values::FieldValue;

use crate::repo::mongodb::MongoRepo;

pub mod repo;

#[tokio::main]
async fn main() {
    const DATABASE:&str = "EDR";
    const COLLECTION:&str ="cdr";
    let  mut record =HashMap::new();
    record.insert("ss".to_string(), FieldValue::Text("ss".to_string()));

    let repo = MongoRepo::new("mongodb://amd:ak@localhost:27017".to_string()).await.unwrap();
    let _ = repo.insert_record(DATABASE, COLLECTION, record).await;

}
