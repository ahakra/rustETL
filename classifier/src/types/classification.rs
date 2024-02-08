use serde_derive::{Deserialize, Serialize};

#[derive(Debug,Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub enum Classification {
    VALID,
    INVALID,
    DISCARDED,
    BAD,
    UNCLASSIFIED,
    UNIDENTIFIED,
}