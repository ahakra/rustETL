use super::protocolUsed:: ProtocolUsed;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceAdapters{
    id :String,
    address :String,
    protocol_used:ProtocolUsed ,
    ProtocolDescription :String,
    ServiceInfoId :String,
}