use crate::expression::Expression;
use crate::expression::ExpressionPiece;
use crate::token::TokenType;
use crate::tokenizer::Tokenizer;
use std::fmt::Display;
use crate::type_parser::Type_;
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
pub struct Param {
    pub name: String,
    pub type_: String,
    pub default_value: Expression,
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

// we'e soon move this to its own file
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Var {
    name: String,
    type_: Type_,
    default_value: Expression,
}

impl Var {
    fn new(t: &mut Tokenizer) -> Self {
        let type_ = Type_::new(t);
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        if t.optionaly_expect_char('=') {
            let default_value = Expression::new(t, '\n', '}'); //} is bc for now this appears in a function body wich ends with }
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value,
            };
        } else {
            if t.current_char() != '\n' {
                t.expect_char('\n');
            }
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value: Expression(ExpressionPiece::Placeholder(false)),
            };
        }
    }
    fn display(&self) {
        println!("Var {} {}", self.type_.to_string(), self.name);
    }
}



use crate::expression::FunctionCall;
use crate::until;
pub enum ValidInFunctionBody {
    Expression(Expression),
    FunctionCall(FunctionCall),
    Var(Var),
}
// we'e soon move this to its own file


pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<ValidInFunctionBody>,
    pub return_type: Type_,
}

impl Function {
    pub fn new(t: &mut Tokenizer) -> Self {
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        t.expect_char('(');
        let params = comp![Param::new(t); until t.optionaly_expect_char(')')];
        let return_type = Type_::new(t);
        let mut res = Self {
            name,
            params,
            body: Vec::new(),
            return_type,
        };
        res.parse_body(t);
        res
    }
    pub fn display(&self) {
        println!("Function {} (", self.name);
        for field in &self.params {
            println!("    {} {},", field.type_, field.name);
        }
        println!(")");
        for field in &self.body {
            match field {
                ValidInFunctionBody::Expression(expression) => {
                    println!("{:?}", expression);
                }
                ValidInFunctionBody::FunctionCall(function_call) => {
                    println!("{:?}", function_call);
                }
                ValidInFunctionBody::Var(var) => {
                    println!("{:?}", var);
                }
            }
        }
        self.return_type.display();
        println!("}}");
    }

    pub fn parse_body(&mut self, t: &mut Tokenizer) {
        t.expect_char('{');
        t.eat_all_spaces();
        dbg!(t.peek_next_word());
        while !t.optionaly_expect_char('}') {
            t.eat_all_spaces();
            dbg!(t.peek_next_word());
            match t.peek_next_word() {
                "const" => {
                    println!("found token const");
                    t.expect(TokenType::KEYWORD);
                    let var = Var::new(t);
                    self.body.push(ValidInFunctionBody::Var(var));
                }
                "let" => {
                    println!("found token let");
                    t.expect(TokenType::KEYWORD);
                    let var = Var::new(t);
                    self.body.push(ValidInFunctionBody::Var(var));
                }
                _ => {
                    println!("didnt find token const or let");
                    let expression = Expression::new(t, '\n', '}');
                    self.body.push(ValidInFunctionBody::Expression(expression));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_parser() {
        let mut t = Tokenizer {
            code: "function sub(int a, int b){}
    
    
    
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
            code: "function sub(int a = 9, int b = 2 + 3){}
    
    
    
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
    fn test_that_parsing_function_body_doesnt_panic() {
        let mut t = Tokenizer {
            code: "function sub(int a = 9, int b = 2 + 3){
                const a = 9
                let b = 2
                a = b+9
            }
    
    
    
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
    fn post_type_system_upgrade_var_test() {
        let mut t = Tokenizer {
            code: "
                const []int  a = 9          
    
    
            "
            .to_string(),
            parse_index: 0,
        };

        assert_eq!(t.expect(TokenType::KEYWORD), "const");
        let var = Var::new(&mut t);
        var.display();
        assert_eq!(var.name, "a");
}
}


