use serde_derive::{Deserialize, Serialize};

#[derive(Debug,Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub enum FieldValue {
    Integer(i32),
    Float(f64),
    Text(String),
    Long(i64),
}