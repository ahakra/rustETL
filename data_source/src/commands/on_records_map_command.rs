use std::collections::HashMap;
use sharedLib::record_mapping_types::field_values::FieldValue;

use crate::events::records_mapped_event;

#[derive(Debug)]
pub struct OnRecordMapCommand {
    pub file_name:String,
    
    pub file_content :Vec<u8>,


}


impl OnRecordMapCommand {
   pub fn apply(file_name:String,file_content:Vec<u8>) {
    let mut event = records_mapped_event::RecordsMappedEvent{file_name:file_name,fields:HashMap::new()};

    let content_string = String::from_utf8_lossy(&file_content);
    // Loop through content_string line by line
    for (_line_number, line) in content_string.lines().enumerate() {
        // Split each line by commas
        let parts: Vec<&str> = line.split(',').collect();

       
          
            let first_name = parts[1].trim();
            event.fields.insert("first_name".to_string(), FieldValue::Text(first_name.to_string()));

            let last_name = parts[2].trim();
            event.fields.insert("last_name".to_string(), FieldValue::Text(last_name.to_string()));
                
            
    
   }
}
}
