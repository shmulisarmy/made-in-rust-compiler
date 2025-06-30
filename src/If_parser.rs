use crate::code_block::CodeBlock;
use crate::code_block::ValidInCodeBlock;
use crate::expression::ExpressionPiece;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;

use crate::expression::Expression;
use crate::comp;

#[derive(Debug)]
pub struct If {
    pub condition: Expression,
    pub body: Vec<ValidInCodeBlock>,
}

impl If {
    pub fn new(t: &mut Tokenizer) -> Self {
        let condition = if t.optionaly_expect_char('(') {
            let res = Expression::new(t, ',', ')');
            t.expect_char(')');
            res
        } else {
            Expression::new(t, '\n', '{')
        };

        t.eat_all_spaces();
        let mut res = Self { condition, body: vec![] };
        res.parse_body(t);
        res
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
            code: "
            if (a + b){
                a  = 9
                b = 2
            }
            "
            .to_string(),
            parse_index: 0,
        };

        assert_eq!(t.expect(TokenType::KEYWORD), "if");
        let _If = If::new(&mut t);
        assert_eq!(_If.body.len(), 2);
    }
}
