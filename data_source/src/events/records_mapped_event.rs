use std::collections::HashMap;
use sharedLib::record_mapping_types::field_values::FieldValue;

pub struct RecordsMappedEvent {
    pub file_name: String,
    pub  fields :Vec<HashMap<String, FieldValue>>
}
