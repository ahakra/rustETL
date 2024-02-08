
use futures::{StreamExt, TryStreamExt};
use mongodb::{bson::{self, bson, doc, Document}, options::{ClientOptions, FindOptions}, Client};
use serde::Serialize;
use crate::types::classifier::Classifier;
pub struct MongoRepo {
    pub client: Client,
}

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
        classifier: Classifier,
    ) -> Result<(), mongodb::error::Error> {
        // Get a handle to a database.
        let alive = &self.client.list_database_names(None, None).await?;
        for i in alive{
            println!("{}",i);
        }
        let db = self.client.database(db_name);
        let collection = db.collection::<Document>(collection_name);

        let document = bson::to_document(&classifier)?;

        collection.insert_one(document, None).await?;

        Ok(())
    }

    pub async fn get_by_record_type(
        &self,
        db_name: &str,
        collection_name: &str,
        record_type: &str,
    ) -> Result<Vec<Classifier>, mongodb::error::Error> {
         // Get a handle to a database.
         let db = self.client.database(db_name);
         let collection = db.collection::<Document>(collection_name);
 
    
        let filter = doc! { "record_type": record_type};
        let find_options = FindOptions::builder().sort(doc! { "order": 1 }).build();
    
        let cursor = collection.find(filter, find_options).await?;
        let classifiers: Result<Vec<Classifier>, mongodb::error::Error> = cursor
        .try_fold(Vec::new(), |mut acc, item| async {
            let document = item;
            let classifier = Classifier::from_document(document)?;
            acc.push(classifier);
            Ok(acc)
        })
        .await;

        Ok(classifiers?)
        
    

    }
    pub async fn delete_record<T: Serialize> (
        &self,
        db_name: &str,
        collection_name: &str,
        classifier: Classifier,
    ) -> Result<(), mongodb::error::Error> {
        // Get a handle to a database.
        let db = self.client.database(db_name);
        let collection = db.collection::<Document>(collection_name);

        let document = bson::to_document(&classifier)?;

        collection.delete_one(document, None).await?;

        Ok(())
    }
}
