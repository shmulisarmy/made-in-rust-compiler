use crate::parser::code_block::CodeBlock;
use crate::parser::code_block::ValidInCodeBlock;
use crate::parser::expression::Expression;
use crate::parser::expression::ExpressionPiece;
use crate::previewScannerUtils::looks_like_type;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;
use crate::parser::type_parser::Type_;
use crate::parser::var_parser::Var;
use crate::utils::red;
use crate::parser::while_parser::While;
use crate::parser::If_parser::If;

use crate::comp;

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub type_: Type_,
    pub default_value: Expression,
}

impl Param {
    fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let type_ = Type_::new(t);
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
            t.expect_char_with_backups(',', &[')']);
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value: Expression(ExpressionPiece::Placeholder(false)),
            };
        }
    }
    fn preview_scan(t: &mut Tokenizer) {
        t.eat_spaces();
        dbg!(t.current_char());
        println!("proo");
        if !looks_like_type(t) {
            let next_token = t.next();
            t.user_error(
                next_token.start_index,
                next_token.start_index + next_token.value.len()
            );
            panic!("{} {}", red("expected type found".to_string()), next_token.value);
        } else {
            println!("yes");
        }
        println!("done");
    }
}

// we'e soon move this to its own file


use crate::parser::expression::FunctionCall;
use crate::until;
pub enum ValidInFunctionBody {
    Expression(Expression),
    FunctionCall(FunctionCall),
    Var(Var),
    While(While),
    If(If),
}
// we'e soon move this to its own file

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<ValidInCodeBlock>,
    pub return_type: Type_,
}

impl Function {
    pub fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        t.expect_char('(');
        let params = comp![Param::new(t); until t.optionaly_expect_char(')')];

        let return_type = if looks_like_type(t) {
            Type_::new(t)
        } else {
            Type_ {
                name: "void".to_string(),
                sub_types: Vec::new(),
                is_optional: false,
            }
        };

        let mut res = Self {
            name,
            params,
            body: Vec::new(),
            return_type,
        };
        res.parse_body(t);
        res
    }
    fn preview_scan(t: &mut Tokenizer) {
        use crate::previewScannerUtils::*;
        if !looks_like_identifier(t) {
            let next_token = t.next();
            t.user_error(
                next_token.start_index,
                next_token.start_index + next_token.value.len(),
            );
            panic!("{}", red("expected identifier (function name)".to_string()));
        }
    }
    pub fn display(&self) {
        println!("Function {} (", self.name);
        for field in &self.params {
            println!("    {} {},", field.type_.name, field.name);
        }
        println!(")");
        for field in &self.body {
            match field {
                ValidInCodeBlock::Expression(expression) => {
                    println!("{:?}", expression);
                }
                ValidInCodeBlock::FunctionCall(function_call) => {
                    println!("{:?}", function_call);
                }
                ValidInCodeBlock::Var(var) => {
                    println!("{:?}", var);
                }
                ValidInCodeBlock::While(while_) => {
                    println!("{:?}", while_);
                }
                ValidInCodeBlock::If(if_) => {
                    println!("{:?}", if_);
                }
            }
        }
        self.return_type.display();
        println!("}}");
    }

    
}


impl CodeBlock for Function{
    fn get_body(&self) -> & Vec<ValidInCodeBlock>{
        &self.body
    }
    fn body_ptr(&mut self) -> &mut Vec<ValidInCodeBlock>{
        &mut self.body
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_parser() {
        let mut t = Tokenizer {
            file_name: file!(),
            start_line: line!() as usize,
            code: "
            
            function sub(int a, int b){}



            "
            .to_string(),
            parse_index: 0,
        };

        assert_eq!(t.expect(TokenType::KEYWORD), "function");

        let _function = Function::new(&mut t);
        assert_eq!(_function.name, "sub");

        assert_eq!(_function.params.len(), 2);
        assert_eq!(_function.params[0].type_.name, "int");
        assert_eq!(_function.params[0].name, "a");

        assert_eq!(_function.params[1].type_.name, "int");
        assert_eq!(_function.params[1].name, "b");
        _function.display();
    }

    #[test]
    fn test_function_parser_that_having_default_values_dont_break_it() {
        let mut t = Tokenizer {
            file_name: file!(),
            start_line: line!() as usize,
            code: "function sub(int a = 9, int b = 2 + 3){}



            "
            .to_string(),
            parse_index: 0,
        };

        assert_eq!(t.expect(TokenType::KEYWORD), "function");

        let _function = Function::new(&mut t);
        assert_eq!(_function.name, "sub");

        assert_eq!(_function.params.len(), 2);
        assert_eq!(_function.params[0].type_.name, "int");
        assert_eq!(_function.params[0].name, "a");

        assert_eq!(_function.params[1].type_.name, "int");
        assert_eq!(_function.params[1].name, "b");
        _function.display();
    }
    #[test]
    fn test_that_parsing_function_body_doesnt_panic() {
        let mut t = Tokenizer {
            file_name: file!(),
            start_line: line!() as usize,
            code: "function sub(int a = 9, int b = 2 + 3){
                const int a = 9
                let int b = 2
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
        assert_eq!(_function.params[0].type_.name, "int");
        assert_eq!(_function.params[0].name, "a");

        assert_eq!(_function.params[1].type_.name, "int");
        assert_eq!(_function.params[1].name, "b");
        _function.display();
    }
   
}
