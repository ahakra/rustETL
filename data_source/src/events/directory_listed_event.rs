use serde::{Deserialize, Serialize};
use crate::traits::event_trait::StorableEvent;

#[derive(Serialize,Clone,Deserialize)]
pub struct DirectoryListedEvent {
    pub directory: String,
    pub files: Vec<String>,
}
impl StorableEvent for DirectoryListedEvent {
    fn event_type(&self) -> &str {
        "DirectoryListedEvent"
    }

    fn serialize(&self) -> String {
        // Use serde to serialize the event to a JSON string
        serde_json::to_string(self).unwrap()
    }
}