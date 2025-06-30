use crate::parser::code_block::{CodeBlock, ValidInCodeBlock};
use crate::parser::function_parser::ValidInFunctionBody;
use crate::project_basic_utils::token::TokenType;
use crate::project_basic_utils::tokenizer::Tokenizer;
use crate::parser::expression::Expression;

use crate::comp;
use crate::SyntaxNode;
use std::sync::Mutex;

#[derive(Debug)]
pub struct While {
    pub condition: Expression,
    pub body: Vec<ValidInCodeBlock>,
}

impl While {
    /// Creates a new While node, pushes it to the parser context, parses the body, and pops it after parsing.
    ///
    /// # Context-walking logic (future):
    /// To resolve a variable/type, iterate backwards through the context stack (Vec<SyntaxNode>),
    /// checking each scope for the definition. The nearest enclosing scope wins. This enables
    /// shadowing and proper scoping for variables/types.
    pub fn new(t: &mut Tokenizer, parser_context: &mut Vec<SyntaxNode>) -> Self {
        t.expect_char('(');
        let condition = Expression::new(t, ',', ')');
        t.expect_char(')');
        t.eat_all_spaces();
        parser_context.push(SyntaxNode::While(While { condition, body: vec![] }));
        let mut node = match parser_context.pop().unwrap() {
            SyntaxNode::While(w) => w,
            _ => unreachable!("Expected While node on context stack"),
        };
        node.parse_body(t, parser_context);
        node
    }
    pub fn display(&self) {
        println!("displaying While statement");
        dbg!(&self.condition);
        for field in &self.body {
            println!("{:?}", field);
        }
    }
}

impl CodeBlock for While{
    fn get_body(&self) -> & Vec<ValidInCodeBlock>{
        &self.body
    }

    fn body_ptr(&mut self) -> &mut Vec<ValidInCodeBlock> {
        &mut self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_without_panic() {
        let mut t = Tokenizer {
            file_name: file!(),
            start_line: line!() as usize,
            code: "\nwhile (a + b){\n    a  = 9\n    b = 2\n}\n".to_string(),
            parse_index: 0,
        };
        let mut context = vec![];
        assert_eq!(t.expect(TokenType::KEYWORD), "while");
        let _while = While::new(&mut t, &mut context);
        assert_eq!(_while.body.len(), 2);
    }
}
