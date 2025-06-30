use crate::project_basic_utils::token::TokenType;
use crate::project_basic_utils::tokenizer::Tokenizer;
use crate::expression::Expression;


use crate::comp;


pub struct While {
    pub condition: Expression,
    pub body: Vec<Expression>,
}

impl While {
    pub fn new(t: &mut Tokenizer) -> Self {
        t.expect_char('(');
        let condition = Expression::new(t, ',', ')');
        t.expect_char(')');

        t.eat_all_spaces();

        t.expect_char('{');
        t.eat_all_spaces();
        let body = comp![Expression::new(t, '\n', '}'); until t.optionaly_expect_char('}')];
        let res = Self { condition, body };
        res
    }
    pub fn display(&self) {
        println!("displaying While statement");
        dbg!(&self.condition);
        for field in &self.body {
            println!("{:?}", field);
        }
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
            while (a + b){
                a  = 9
                b = 2
            }
            "
            .to_string(),
            parse_index: 0,
        };

        assert_eq!(t.expect(TokenType::KEYWORD), "while");
        let _while = While::new(&mut t);
        assert_eq!(_while.body.len(), 2);
    }
}
