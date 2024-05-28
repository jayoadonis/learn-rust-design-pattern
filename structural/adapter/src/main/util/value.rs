#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Unknown(ValueData),
    Waiting(ValueData),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ValueData {
    pub code: u8,
    pub value: &'static str,
    pub description: &'static str,
}

impl Value {
    pub const UNKNOWN: ValueData = ValueData {
        code: 0b0000_0001,
        value: "UNK",
        description: "UNKNOWN",
    };
    
    pub const WAITING: ValueData = ValueData {
        code: 0b0000_0010,
        value: "WTG",
        description: "WAITING",
    };
    
    pub fn get_data(&self) -> &ValueData {
        match self {
            Value::Unknown(data) => data,
            Value::Waiting(data) => data,
        }
    }
}