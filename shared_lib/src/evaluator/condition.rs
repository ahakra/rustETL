use std::collections::HashMap;

use super::super::record_mapping_types::field_values::FieldValue;

use super::operator;
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

    pub fn evaluate_condition(&self, record: &HashMap<String, FieldValue>) -> bool {
        match self.condition_type {
            ConditionType::AND => {
                let specified_condition = self.evaluate_operator(record);
                let sub_conditions = self
                    .sub_cond
                    .iter()
                    .all(|sub_cond| sub_cond.evaluate_condition(record));

                specified_condition && sub_conditions
            }
            ConditionType::OR => {
                let specified_condition = self.evaluate_operator(record);
                let sub_conditions = self
                    .sub_cond
                    .iter()
                    .any(|sub_cond| sub_cond.evaluate_condition(record));

                specified_condition || sub_conditions
            }
            ConditionType::None => true,
        }
    } 

pub fn evaluate_operator(&self,record:&HashMap<String,FieldValue>,)->bool {
    match self.operator  {
        Operator::Equal => {
            if let Some(field_name) = record.get(&self.field_name){
                return field_name == &self.field_value
            } else{
                return false
            }
        },
        Operator::LessThanOrEqual => {
            if let Some(field_name) = record.get(&self.field_name){
                return field_name <= &self.field_value
            } else{
                return false
            }
        },
        Operator::GreaterThanOrEqual =>  {
            if let Some(field_name) = record.get(&self.field_name){
                return field_name >= &self.field_value
            } else{
                return false
            }
        },
        Operator::GreaterThan =>  {
            if let Some(field_name) = record.get(&self.field_name){
                return field_name > &self.field_value
            } else{
                return false
            }
        },
        Operator::LessThan =>  {
            if let Some(field_name) = record.get(&self.field_name){
                return field_name < &self.field_value
            } else{
                return false
            }
        },
        Operator::NotEqual =>  {
            if let Some(field_name) = record.get(&self.field_name){
                return field_name != &self.field_value
            } else{
                return false
            }
        },
        Operator::Empty =>  {
            if let Some(_field_name) = record.get(&self.field_name){
                return false
            } else{
                return true
            }
        },
        Operator::NonEmpty => {
            if let Some(_field_name) = record.get(&self.field_name){
                return true
            } else{
                return false
            }
        },
    
    }
  }   
}