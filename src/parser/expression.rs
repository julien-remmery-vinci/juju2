use super::{datatype::DataType, operation::Operation};

#[derive(PartialEq, Debug)]
pub enum Expression {
    VariableDef(VariableDefExpression),
    VariableInit(VariableInitExpression),
    Operation(OperationExpression),
    FunctionDef(FunctionDefExpression),
    FunctionCall(FunctionCallExpression),
    Return(ReturnExpression),
}

#[derive(PartialEq, Debug)]
pub enum DynamicValue {
    Integer(i32),
    Double(f64),
    Float(f32),
    Char(char),
    String(String),
    Var(String)
}

#[derive(PartialEq, Debug)]
pub struct VariableDefExpression {
    pub data_type: DataType,
    pub identifier: String,
}

#[derive(PartialEq, Debug)]
pub struct VariableInitExpression {
    pub identifier: String,
    pub value: DynamicValue,
}

#[derive(PartialEq, Debug)]
pub struct OperationExpression {
    pub operation: Operation,
    pub lhs: DynamicValue,
    pub rhs: DynamicValue,
}

#[derive(PartialEq, Debug)]
pub struct FunctionDefExpression {
    pub return_type: DataType,
    pub identifier: String,
    pub params: Vec<DynamicValue>,
}

#[derive(PartialEq, Debug)]
pub struct FunctionCallExpression {
    pub identifier: String,
    pub params: Vec<DynamicValue>,
}

#[derive(PartialEq, Debug)]
pub enum ReturnValue {
    Integer(i32),
    Double(f64),
    Float(f32),
    Char(char),
    String(String),
    Var(String)
}

#[derive(PartialEq, Debug)]
pub struct ReturnExpression {
    pub value: ReturnValue,
}