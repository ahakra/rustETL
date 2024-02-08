use shared_lib::{evaluator::{condition::Condition, condition_type::ConditionType, operator::Operator}, record_mapping_types::field_values::FieldValue};
use lazy_static::lazy_static;
lazy_static! {
    pub static ref condition1 : Condition= Condition {
    sub_cond: Vec::new(),
    condition_type: ConditionType::AND,
    operator: Operator::StartsWtih,
    field_name: "MSISDN".to_string(),
    field_value: FieldValue::Text("66".to_string()),
};


}

