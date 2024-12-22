use super::error::ParserError;

#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
    Integer,
    Double,
    Float,
    Char,
    String,
}

impl DataType {
    pub fn parse(
        str: &'static str
    ) -> Result<DataType, ParserError> {
        match str {
            "int" => Ok(DataType::Integer),
            "double" => Ok(DataType::Double),
            "float" => Ok(DataType::Float),
            "char" => Ok(DataType::Char),
            "string" => Ok(DataType::String),
            _ => Err(ParserError::NotADataType(str))
        }
    }
}