
use std::collections::HashMap;

use futures::{StreamExt, TryStreamExt};
use mongodb::{bson::{self, bson, doc, Document}, options::{ClientOptions, FindOptions}, Client};
use serde::Serialize;
use shared_lib::record_mapping_types::field_values::FieldValue;

pub struct MongoRepo {
    pub client: Client,
}
type record =HashMap<String, FieldValue>;
impl MongoRepo {
    pub async fn new(connection: String) -> Result<Self, mongodb::error::Error> {
        let client_options = ClientOptions::parse(connection).await?;

        let client = Client::with_options(client_options)?;

        let repo = MongoRepo {
            client,
        };

        Ok(repo)
    }
   

    pub async fn insert_record (
        &self,
        db_name: &str,
        collection_name: &str,
        record: record,
    ) -> Result<(), mongodb::error::Error> {

     
        let db = self.client.database(db_name);
        let collection = db.collection::<Document>(collection_name);

        let document = bson::to_document(&record)?;

        collection.insert_one(document, None).await?;

        Ok(())
    }

 
    pub async fn delete_record<T: Serialize> (
        &self,
        db_name: &str,
        collection_name: &str,
        record: &record,
    ) -> Result<(), mongodb::error::Error> {
        // Get a handle to a database.
        let db = self.client.database(db_name);
        let collection = db.collection::<Document>(collection_name);

        let document = bson::to_document(record)?;

        collection.delete_one(document, None).await?;

        Ok(())
    }
}
