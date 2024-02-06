use std::collections::HashMap;
use std::io;
use sharedLib::record_mapping_types::field_values::FieldValue;
use crate::events::records_mapped_event::RecordsMappedEvent;

#[derive(Debug)]
pub struct OnRecordMapCommand {
    pub file_name: String,
    pub file_content: Vec<u8>,
}

impl OnRecordMapCommand {
    pub fn apply(&mut self) -> Result<RecordsMappedEvent, io::Error> {
        let mut event = RecordsMappedEvent {
            file_name: self.file_name.clone(),
            fields: Vec::new(),
        };

        let content_string = String::from_utf8_lossy(&self.file_content);
        let mut lines = content_string.lines();

        // Skip the first line (header)
        lines.next();

        // Loop through content_string line by line
        for (line_number, line) in lines.enumerate() {
            // Split each line by commas
            let mut hash_map = HashMap::new();
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() >= 4 {
                hash_map.insert("type".to_string(), FieldValue::Text("cdr".to_string()));
                hash_map.insert("file_name".to_string(), FieldValue::Text(self.file_name.to_string()));
                hash_map.insert("data_source_id".to_string(), FieldValue::Text("13336663313".to_string()));
                let id = parts[0].trim();
               
                if let Ok(integer_value) = id.parse::<i32>() {
                    hash_map.insert("id".to_string(), FieldValue::Integer(integer_value));
                }

                let first_name = parts[1].trim();
                hash_map.insert("first_name".to_string(), FieldValue::Text(first_name.to_string()));

                let last_name = parts[2].trim();
                hash_map.insert("last_name".to_string(), FieldValue::Text(last_name.to_string()));

                let msisdn = parts[3].trim();
                hash_map.insert("msisdn".to_string(), FieldValue::Text(msisdn.to_string()));

                event.fields.push(hash_map);
            } else {
                // Handle the case where the line doesn't have enough fields
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid line format at line {}", line_number + 2)));
            }
        }

        Ok(event)
    }
}
