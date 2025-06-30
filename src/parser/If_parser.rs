use crate::parser::code_block::CodeBlock;
use crate::parser::code_block::ValidInCodeBlock;
use crate::parser::expression::ExpressionPiece;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;
use crate::parser::expression::Expression;
use crate::comp;
use crate::SyntaxNode;
use std::sync::Mutex;

#[derive(Debug)]
pub struct If {
    pub condition: Expression,
    pub body: Vec<ValidInCodeBlock>,
}
impl If {
    /// Creates a new If node, pushes it to the parser context, parses the body, and pops it after parsing.
    ///
    /// # Context-walking logic (future):
    /// To resolve a variable/type, iterate backwards through the context stack (Vec<SyntaxNode>),
    /// checking each scope for the definition. The nearest enclosing scope wins. This enables
    /// shadowing and proper scoping for variables/types.
    pub fn new(t: &mut Tokenizer, parser_context: &mut Vec<SyntaxNode>) -> Self {
        let condition = if t.optionaly_expect_char('(') {
            let res = Expression::new(t, ',', ')');
            t.expect_char(')');
            res
        } else {
            Expression::new(t, '\n', '{')
        };
        t.eat_all_spaces();
        parser_context.push(SyntaxNode::If(If { condition, body: vec![] }));
        let mut node = match parser_context.pop().unwrap() {
            SyntaxNode::If(i) => i,
            _ => unreachable!("Expected If node on context stack"),
        };
        node.parse_body(t, parser_context);
        node
    }
    pub fn display(&self) {
        println!("displaying If statement");
        dbg!(&self.condition);
        for field in &self.body {
            println!("{:?}", field);
        }
    }
}

impl CodeBlock for If{
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
            code: "\nif (a + b){\n    a  = 9\n    b = 2\n}\n".to_string(),
            parse_index: 0,
        };
        let mut context = vec![];
        assert_eq!(t.expect(TokenType::KEYWORD), "if");
        let _If = If::new(&mut t, &mut context);
        assert_eq!(_If.body.len(), 2);
    }
}
