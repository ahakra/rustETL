use mongodb::bson::{self, Document};
use shared_lib::evaluator::condition::Condition;
use serde_derive::{Deserialize, Serialize};
use super::classification::Classification;
#[derive(Debug,Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub struct Classifier{
    pub condition:Condition,
    pub classification: Classification,
    pub order:i32,
    pub record_type:String,
}

impl Classifier {
    pub fn to_document(&self) -> Result<Document, mongodb::error::Error> {
        bson::to_document(&self).map_err(|err| mongodb::error::Error::from(err))
    }
    pub fn from_document(document: Document) -> Result<Classifier, mongodb::error::Error> {
        bson::from_document(document).map_err(|err| mongodb::error::Error::from(err))
    }
}