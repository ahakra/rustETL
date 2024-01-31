use super::protocolUsed:: ProtocolUsed;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceAdapters{
    pub  id :String,
    pub address :String,
    pub    protocol_used:ProtocolUsed ,
    pub    protocol_description :String,
    pub   service_info_id :String,
}