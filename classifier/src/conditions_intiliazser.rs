use shared_lib::{evaluator::{condition::Condition, condition_type::ConditionType, operator::Operator}, record_mapping_types::field_values::FieldValue};
use lazy_static::lazy_static;
use crate::types::classifier::Classifier;
use crate::types::classification::Classification::{UNCLASSIFIED, VALID, DISCARDED};

lazy_static! {
    pub static ref CONDITION_6667: Condition = Condition {
        sub_cond: Vec::new(),
        condition_type: ConditionType::AND,
        operator: Operator::StartsWith,  
        field_name: "MSISDN".to_string(),
        field_value: FieldValue::Text("6667".to_string()),
    };
    
    pub static ref CONDITION_6666: Condition = Condition {
        sub_cond: Vec::new(),
        condition_type: ConditionType::AND,
        operator: Operator::StartsWith, 
        field_name: "MSISDN".to_string(),
        field_value: FieldValue::Text("6666".to_string()),
    };

    pub static ref CONDITION_LEEANNE: Condition = Condition {
        sub_cond: Vec::new(),
        condition_type: ConditionType::AND,
        operator: Operator::StartsWith,  
        field_name: "first_name".to_string(),
        field_value: FieldValue::Text("Leeanne".to_string()),
    };

    pub static ref CLASSIFIER_6667: Classifier = Classifier {
        condition: CONDITION_6667.clone(),
        classification: UNCLASSIFIED,
        record_type: "cdr".to_string(),
        order: 1,
    };

    pub static ref CLASSIFIER_6666: Classifier = Classifier {
        condition: CONDITION_6666.clone(),  
        classification: VALID,
        record_type: "cdr".to_string(),
        order: 1,
    };

    pub static ref CLASSIFIER_LEEANNE: Classifier = Classifier {
        condition: CONDITION_LEEANNE.clone(), 
        classification: DISCARDED,
        record_type: "cdr".to_string(),
        order: 1,
    };
}
