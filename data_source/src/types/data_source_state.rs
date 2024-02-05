use serde::{Serialize, Deserialize};
use bson::{doc, oid::ObjectId};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceState {
    #[serde(rename = "_id")] // Map "id" field to "_id" for MongoDB compatibility
    id: ObjectId,
    datasource_id: String, 
    last_imported_file: Option<String>, 
    last_imported_date:i64, 
}