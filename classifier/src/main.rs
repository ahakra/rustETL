use std::collections::HashMap;

use shared_lib::record_mapping_types::field_values::FieldValue;

use shared_lib::evaluator::{condition::Condition, condition_type::ConditionType, operator::Operator};
use crate::repo::mongodb::MongoRepo;
use crate::types::classification::Classification::UNCLASSIFIED;
use crate::types::classifier::Classifier;
use std::time::Duration;
//use kafka::producer::{Producer, Record, RequiredAcks};
pub mod types;
pub mod repo;
#[tokio::main]
async fn main() {
     // Create a record with additional fields
     let mut record: HashMap<String, FieldValue> = HashMap::new();
     record.insert("age".to_string(), FieldValue::Integer(35));
     record.insert("salary".to_string(), FieldValue::Float(50000.0));
     record.insert("name".to_string(), FieldValue::Text("John".to_string()));
 
     // Create a complex condition
     let condition4 = Condition {
         sub_cond: Vec::new(),
         condition_type: ConditionType::AND,
         operator: Operator::StartsWtih,
         field_name: "name".to_string(),
         field_value: FieldValue::Text("J".to_string()),
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
         condition_type: ConditionType::AND,
         operator: Operator::GreaterThanOrEqual,
         field_name: "salary".to_string(),
         field_value: FieldValue::Float(45000.0),
     };
 
     let condition1 = Condition {
         sub_cond: vec![condition3, condition2],
         condition_type: ConditionType::OR,
         operator: Operator::GreaterThan,
         field_name: "age".to_string(),
         field_value: FieldValue::Integer(40),
     };
 
     // Evaluate the condition
     let result = condition1.evaluate_condition(&record);
 
     // Print the result
     println!("Condition evaluation result: {}", result);
     
  
    let condition_json = serde_json::to_string_pretty(&condition1).unwrap();

    let classifier = Classifier{
        condition:serde_json::from_str(&condition_json).unwrap(),
        classification:UNCLASSIFIED,
        record_type:"cdr".to_string(),
        order:1,
    };

    let repo = MongoRepo::new("mongodb://amd:ak@localhost:27017".to_string()).await.unwrap();
    let _insert = repo.insert_record("edr","cdr",classifier).await.unwrap();

    let gettt = repo.get_by_record_type("edr","cdr","cdr").await.unwrap();
    for i in gettt {
        println!("{:?}",i);
    }


//     let classifier_json = serde_json::to_string_pretty(&classifier).unwrap();
//    println!("Serialized classifier:\n: {}", classifier_json);

    // let  buf = serde_json::to_vec(&record).unwrap();
    // let  producer =
    //     Producer::from_hosts(vec!("localhost:9092".to_owned()))
    //         .with_ack_timeout(Duration::from_secs(1))
    //         .with_required_acks(RequiredAcks::One)
    //         .create();

    // match producer{
    //     Ok(mut producer) =>{
    //         let result = &producer.send(&Record::from_value("my-topic", buf));
    //         match result {
    //             Ok(_) => {
    //                 println!("OK");
    //             }
    //             Err(_) => {
    //                 println!("Error: ");
    //             }
    //         }
    //     }
    //     _ => println!("error")
    // }




}
