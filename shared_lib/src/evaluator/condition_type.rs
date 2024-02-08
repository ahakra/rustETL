use serde_derive::{Deserialize, Serialize};

#[derive(Debug,Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub enum ConditionType {
    AND,
    OR,
    None,
}