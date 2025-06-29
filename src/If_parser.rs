use crate::token::TokenType;
use crate::tokenizer::Tokenizer;

use crate::expression::Expression;

macro_rules! comp {
    [tuple_item_1:tt, tuple_item_2:tt; for x in expr] => {
        {
            let mut res = Vec::new();
            for x in expr {
                res.push((tuple_item_1, tuple_item_2));
            }
            res
        }
    };
    [$value:expr; until $cond:expr] => {
        {
            let mut res = Vec::new();
            while !$cond {
                res.push($value);
            }
            res
        }
    };

}

pub struct If {
    pub condition: Expression,
    pub body: Vec<Expression>,
}

impl If {
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
        println!("displaying If statement");
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
