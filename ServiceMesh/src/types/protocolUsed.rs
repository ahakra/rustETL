use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub  enum ProtocolUsed {
    WebApi,
    GRPC ,
    KAFKA ,
  }
  