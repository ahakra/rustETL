use serde::{Deserialize, Serialize};
use crate::traits::event_trait::StorableEvent;

#[derive(Serialize,Clone,Deserialize)]
pub struct FilesReadEvent {
    pub file_name: String,
    pub file_content :Vec<u8>,
}
impl StorableEvent for FilesReadEvent {
    fn event_type(&self) -> &str {
        "FilesReadEvent"
    }

    fn serialize(&self) -> String {
        // Use serde to serialize the event to a JSON string
        serde_json::to_string(self).unwrap()
    }
}