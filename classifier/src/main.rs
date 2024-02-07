use std::collections::HashMap;

use shared_lib::record_mapping_types::field_values::FieldValue;

use shared_lib::evaluator::{classifier::Condition, condition_type::ConditionType, operator::Operator};


fn main() {
     // Create some sample data
    let mut records: Vec<HashMap<String, FieldValue>> = Vec::new();

    let mut record1: HashMap<String, FieldValue> = [("age".to_string(), FieldValue::Integer(25))]
        .iter()
        .cloned()
        .collect();
    
    let record2: HashMap<String, FieldValue> = [("age".to_string(), FieldValue::Integer(35))]
        .iter()
        .cloned()
        .collect();

    let record3: HashMap<String, FieldValue> = [("age".to_string(), FieldValue::Integer(45))]
        .iter()
        .cloned()
        .collect();

    records.push(record1);
    records.push(record2);
    records.push(record3);



      // Test some conditions

      let condition3 = Condition {
        sub_cond: Vec::new(),
        condition_type: ConditionType::None,
        operator: Operator::LessThanOrEqual,
        field_name: "age".to_string(),
        field_value: FieldValue::Integer(40),
    };


      let condition1 = Condition {
        sub_cond: vec![condition3],
        condition_type: ConditionType::AND,
        operator: Operator::GreaterThanOrEqual,
        field_name: "age".to_string(),
        field_value: FieldValue::Integer(30),
    };

   

    let condition2 = Condition {
        sub_cond: Vec::new(),
        condition_type: ConditionType::OR,
        operator: Operator::LessThanOrEqual,
        field_name: "age".to_string(),
        field_value: FieldValue::Integer(40),
    };

    println!("Records matching condition 1:");
    for record in &records {
        if Condition::evaluate_condition(&condition1, &record) {
            println!("{:?}", record);
        }
    }

    println!("Records matching condition 2:");
    for record in &records {
        if Condition::evaluate_condition(&condition2, &record) {
            println!("{:?}", &record);
        }
    }
}
