use shared_lib::evaluator::condition::Condition;
use serde_derive::{Deserialize, Serialize};
use super::classification::Classification;
#[derive(Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub struct Classifier{
    pub condition:Condition,
    pub classification: Classification,

}

