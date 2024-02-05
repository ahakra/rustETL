#[derive(Debug)]
pub enum FieldValue {
    Integer(i32),
    Float(f64),
    Text(String),
    Long(i64),
}