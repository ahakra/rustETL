use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub enum Operator{
    Equal,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
    LessThan,
    NotEqual,
    Empty,
    NonEmpty,
}