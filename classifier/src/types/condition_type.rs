use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub enum ConditionType {
    AND,
    OR,
}