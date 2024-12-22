use super::expression::Expression;

pub struct Program {
    pub expressions: Vec<Expression>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            expressions: Vec::new()
        }
    }

    pub fn add_exp(
        &mut self,
        expression: Expression
    ) -> bool {
        self.expressions.push(expression);
        true
    }

    pub fn remove_exp(
        &mut self,
        expression: Expression
    ) -> bool {
        if self.expressions.is_empty() {
            return false;
        }
        if !self.expressions.contains(&expression) {
            return false;
        }
        self.expressions.retain(|e| *e != expression);
        true
    }
}