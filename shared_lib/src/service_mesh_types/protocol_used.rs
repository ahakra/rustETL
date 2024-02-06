use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub  enum ProtocolUsed {
    WebApi,
    GRPC ,
    KAFKA ,
  }
  impl From<i32> for ProtocolUsed {
    fn from(value: i32) -> Self {
        match value {
            0 => ProtocolUsed::WebApi,
            1 => ProtocolUsed::GRPC,
            2 => ProtocolUsed::KAFKA,
            _ => panic!("Unexpected value for ProtocolUsed: {}", value),
        }
    }
}

impl Into<i32> for ProtocolUsed {
  fn into(self) -> i32 {
      match self {
          ProtocolUsed::WebApi => 0,
          ProtocolUsed::GRPC => 1,
          ProtocolUsed::KAFKA => 2,
      }
  }
}
