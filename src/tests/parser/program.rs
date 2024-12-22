#[cfg(test)]
// Tests the program functions (new, add_exp, remove_exp)
mod program_tests {
    use crate::parser::{datatype::DataType, error::ParserError, expression::{ Expression, VariableDefExpression}, program::Program};

    // Identifier used for test expression initialization
    const SETUP_VAR_NAME: &str = "setup_var_name";

    // Helper function to create a VariableDef Expression
    fn setup_variable_def_exp(identifier: &str, data_type: DataType) -> Expression {
        Expression::VariableDef(VariableDefExpression {
            data_type,
            identifier: identifier.to_string(),
        })
    }
    
    // Helper function to create an empty program
    fn setup_empty_program() -> Program {
        Program::new()
    }
    
    // Helper function to create a program with one expression, identifier set to SETUP_VAR_NAME
    fn setup_program() -> Program {
        Program {
            expressions: vec![setup_variable_def_exp(SETUP_VAR_NAME, DataType::Integer)]
        }
    }

    // Tests the program initialization with empty vector
    #[test]
    fn new() -> Result<(), String> {
        let program = Program::new();
        assert_eq!(program.expressions.len(), 0, "Program should initialize with an empty Vec.");
        Ok(())
    }

    // Add an expression to the program
    #[test]
    fn add_exp() -> Result<(), ParserError> {
        let mut program: Program = setup_empty_program();
        let def_exp = setup_variable_def_exp(SETUP_VAR_NAME, DataType::Integer);
        assert!(program.add_exp(def_exp));
        assert!(program.expressions.len() == 1);
        Ok(())
    }

    // Remove an expression from the program
    #[test]
    fn remove_exp() -> Result<(), ParserError> {
        let mut program: Program = setup_program();
        let def_exp = setup_variable_def_exp(SETUP_VAR_NAME, DataType::Integer);
        assert!(program.remove_exp(def_exp));
        Ok(())
    }

    // Remove an expression from an empty program
    #[test]
    fn remove_exp_empty() -> Result<(), ParserError> {
        let mut program: Program = setup_empty_program();
        let def_exp = setup_variable_def_exp(SETUP_VAR_NAME, DataType::Integer);
        assert!(!program.remove_exp(def_exp));
        Ok(())
    }

    // Remove an expression not contained in a program
    #[test]
    fn remove_not_contains() -> Result<(), ParserError> {
        let mut program: Program = setup_program();
        let def_exp = setup_variable_def_exp("new_var", DataType::Integer);
        assert!(!program.remove_exp(def_exp));
        Ok(())
    }
}