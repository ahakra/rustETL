use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize)]
pub enum FieldValue {
    Integer(i32),
    Float(f64),
    Text(String),
    Long(i64),
}