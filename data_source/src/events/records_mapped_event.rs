use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sharedLib::record_mapping_types::field_values::FieldValue;
use crate::traits::event_trait::StorableEvent;

#[derive(Serialize,Clone,Deserialize)]
pub struct RecordsMappedEvent {
    pub file_name: String,
    pub  fields :Vec<HashMap<String, FieldValue>>
}

impl StorableEvent for RecordsMappedEvent {
    fn event_type(&self) -> &str {
        "RecordsMappedEvent"
    }

    fn serialize(&self) -> String {
        // Use serde to serialize the event to a JSON string
        serde_json::to_string(self).unwrap()
    }
}