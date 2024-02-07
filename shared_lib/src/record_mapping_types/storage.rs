use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use crate::record_mapping_types::field_values::FieldValue;

#[derive(Debug,Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub struct  Storage {
    record:HashMap<String, FieldValue>,
    data_base:String,
    collection:String,

}
