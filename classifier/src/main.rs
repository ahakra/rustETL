use std::collections::HashMap;

use shared_lib::record_mapping_types::field_values::FieldValue;

use shared_lib::evaluator::{condition::Condition, condition_type::ConditionType, operator::Operator};
use crate::types::classification::Classification::UNCLASSIFIED;
use crate::types::classifier::Classifier;

pub mod types;

fn main() {
     // Create a record with additional fields
     let mut record: HashMap<String, FieldValue> = HashMap::new();
     record.insert("age".to_string(), FieldValue::Integer(35));
     record.insert("salary".to_string(), FieldValue::Float(50000.0));
     record.insert("name".to_string(), FieldValue::Text("John".to_string()));
 
     // Create a complex condition
     let condition4 = Condition {
         sub_cond: Vec::new(),
         condition_type: ConditionType::None,
         operator: Operator::Equal,
         field_name: "name".to_string(),
         field_value: FieldValue::Text("John".to_string()),
     };
 
     let condition3 = Condition {
         sub_cond: vec![condition4],
         condition_type: ConditionType::AND,
         operator: Operator::LessThanOrEqual,
         field_name: "age".to_string(),
         field_value: FieldValue::Integer(40),
     };
 
     let condition2 = Condition {
         sub_cond: Vec::new(),
         condition_type: ConditionType::None,
         operator: Operator::GreaterThanOrEqual,
         field_name: "salary".to_string(),
         field_value: FieldValue::Float(45000.0),
     };
 
     let condition1 = Condition {
         sub_cond: vec![condition3, condition2],
         condition_type: ConditionType::OR,
         operator: Operator::GreaterThan,
         field_name: "age".to_string(),
         field_value: FieldValue::Integer(30),
     };
 
     // Evaluate the condition
     let result = condition1.evaluate_condition(&record);
 
     // Print the result
     println!("Condition evaluation result: {}", result);

     // Serialize condition1 to a JSON-formatted string
    let condition_json = serde_json::to_string_pretty(&condition1).unwrap();
    println!("Serialized Condition1:\n{}", condition_json);
    let classifier = Classifier{
        condition:serde_json::from_str(&condition_json).unwrap(),
        classification:UNCLASSIFIED,
    };

    let classifier_json = serde_json::to_string_pretty(&classifier).unwrap();
   println!("Serialized classifier:\n: {}", classifier_json);

}
