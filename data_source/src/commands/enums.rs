use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Commands {
    ONDIRECTORYLISTCOMMAND,
    ONFILEREADCOMMAND
}

impl Commands {

 pub fn stringify(&self) -> String{
    match &self {
        &Self::ONDIRECTORYLISTCOMMAND => {"ONDIRECTORYLISTCOMMAND".to_string()},
        &Self::ONFILEREADCOMMAND => {"ONFILEREADCOMMAND".to_string()}
    }
 }  
}
impl fmt::Display for Commands {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            &Self::ONDIRECTORYLISTCOMMAND => {write!(f,"ONDIRECTORYLISTCOMMAND")},
            &Self::ONFILEREADCOMMAND => {write!(f,"ONFILEREADCOMMAND")}
        }
    }
}
