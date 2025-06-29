use std::fmt::Display;
use crate::tokenizer::Tokenizer;
use crate::token::TokenType;
use crate::expression::Expression;
use crate::expression::ExpressionPiece;



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



#[derive(Debug)]
struct Param {
    name: String,
    type_: String,
    default_value: Expression
}

impl Param {
    fn new(t: &mut Tokenizer) -> Self {
        let type_ = t.expect(TokenType::IDENTIFIER).to_string();
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        if t.optionaly_expect_char('=') {
            let default_value = Expression::new(t, ',', ')');
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value,
            };
        } else {
            if t.current_char() != ')' {
                t.expect_char(',');
            }
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value: Expression(ExpressionPiece::Placeholder(false)),
            };
        }
    }
}


pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
}

impl Function {
    pub fn new(t: &mut Tokenizer) -> Self {
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        t.expect_char('(');
        let params = comp![Param::new(t); until t.optionaly_expect_char(')')];
        return Self { name, params };
    }
    pub fn display(&self) {
        println!("Function {} (", self.name);
        for field in &self.params {
            println!("    {} {},", field.type_, field.name);
        }
        println!(")");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_parser() {
        let mut t = Tokenizer {
            code: "function sub(int a, int b)
    
    
    
            "
            .to_string(),
            parse_index: 0,
        };


        assert_eq!(t.expect(TokenType::KEYWORD), "function");

        let _function = Function::new(&mut t);
        assert_eq!(_function.name, "sub");

        assert_eq!(_function.params.len(), 2);
        assert_eq!(_function.params[0].type_, "int");
        assert_eq!(_function.params[0].name, "a");

        assert_eq!(_function.params[1].type_, "int");
        assert_eq!(_function.params[1].name, "b");
        _function.display();
        
    }


    #[test]
    fn test_function_parser_that_having_default_values_dont_break_it() {
        let mut t = Tokenizer {
            code: "function sub(int a = 9, int b = 2 + 3)
    
    
    
            "
            .to_string(),
            parse_index: 0,
        };


        assert_eq!(t.expect(TokenType::KEYWORD), "function");

        let _function = Function::new(&mut t);
        assert_eq!(_function.name, "sub");

        assert_eq!(_function.params.len(), 2);
        assert_eq!(_function.params[0].type_, "int");
        assert_eq!(_function.params[0].name, "a");

        assert_eq!(_function.params[1].type_, "int");
        assert_eq!(_function.params[1].name, "b");
        _function.display();
        
    }
}









