use std::collections::HashMap;

use shared_lib::record_mapping_types::field_values::FieldValue;

use super::{condition_type::ConditionType, operator::Operator};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Clone,Deserialize,PartialEq,PartialOrd)]
pub struct Condition  {
    pub sub_cond:Vec<Condition>,
    pub condition_type:ConditionType,

    pub operator:Operator,    
    pub field_name:String,
    pub field_value:FieldValue,

}



// Recursive function to evaluate conditions

impl Condition{

    pub fn evaluate_condition(condition:Condition, record:&HashMap<String,FieldValue>) -> bool {

        match condition.condition_type{
    
            ConditionType::AND => {
                for sub_cond in condition.sub_cond {
    
                    if !Self::evaluate_condition(sub_cond, &record) {
                                    return false
                    }               
            }       
            return true
        },
        ConditionType::OR => {
            for sub_cond in condition.sub_cond {

                if Self::evaluate_condition(sub_cond, &record) {
                                return false
                }               
        }       
        return true
    },
    _ =>{
        match condition.operator  {
            Operator::Equal => {
                if let Some(field_name) = record.get(&condition.field_name){
                    return field_name == &condition.field_value
                } else{
                    return false
                }
            },
            Operator::LessThanOrEqual => {
                if let Some(field_name) = record.get(&condition.field_name){
                    return field_name <= &condition.field_value
                } else{
                    return false
                }
            },
            Operator::GreaterThanOrEqual =>  {
                if let Some(field_name) = record.get(&condition.field_name){
                    return field_name >= &condition.field_value
                } else{
                    return false
                }
            },
            Operator::GreaterThan =>  {
                if let Some(field_name) = record.get(&condition.field_name){
                    return field_name > &condition.field_value
                } else{
                    return false
                }
            },
            Operator::LessThan =>  {
                if let Some(field_name) = record.get(&condition.field_name){
                    return field_name < &condition.field_value
                } else{
                    return false
                }
            },
            Operator::NotEqual =>  {
                if let Some(field_name) = record.get(&condition.field_name){
                    return field_name != &condition.field_value
                } else{
                    return false
                }
            },
            Operator::Empty =>  {
                if let Some(_field_name) = record.get(&condition.field_name){
                    return false
                } else{
                    return true
                }
            },
            Operator::NonEmpty => {
                if let Some(_field_name) = record.get(&condition.field_name){
                    return true
                } else{
                    return false
                }
            },
        }
    }

     
    
    }
    } 
}
