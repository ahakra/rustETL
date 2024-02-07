use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub enum Classification {
    VALID,
    INVALID,
    DISCARDED,
    BAD,
    UNCLASSIFIED,
    UNIDENTIFIED,
}