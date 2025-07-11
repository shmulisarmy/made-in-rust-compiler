use std::string;

use crate::parser::code_block::CodeBlock;
use crate::parser::code_block::ValidInCodeBlock;
use crate::parser::expression::Expression;
use crate::parser::expression::ExpressionPiece;
use crate::parser::type_parser::Type_;
use crate::parser::var_parser::Var;

use crate::previewScannerUtils::looks_like_type;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;
use crate::utils::red;

use crate::comp;

#[derive(Debug)]
pub struct Param {
    pub name: &'static str,
    pub type_: Type_,
    pub default_value: Expression,
}

impl Param {
    fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let type_ = Type_::new(t);
        let name = t.expect(TokenType::IDENTIFIER);
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
        // dbg!(t.current_char());
        if !looks_like_type(t) {
            let next_token = t.next();
            t.user_error(
                next_token.start_index,
                next_token.start_index + next_token.value.len(),
            );
            panic!(
                "{} {}",
                red("expected type found".to_string()),
                next_token.value
            );
        }
    }
}

// we'e soon move this to its own file

use crate::parser::expression::FunctionCall;
use crate::until;
// we'e soon move this to its own file

pub struct Function {
    pub name: &'static str,
    pub params: Vec<Param>,
    pub body: Vec<ValidInCodeBlock>,
    pub return_type: Type_,
}

impl Function {
    pub fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let name = t.expect(TokenType::IDENTIFIER);
        t.expect_char('(');
        let params = comp![Param::new(t); until t.optionaly_expect_char(')')];

        let return_type = if looks_like_type(t) {
            Type_::new(t)
        } else {
            Type_ {
                name: "void",
                sub_types: Vec::new(),
                is_optional: false,
                is_pointer: false,
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

    fn parse_body(&mut self, t: &mut Tokenizer) {
        t.expect_char('{');
        until!(t.optionaly_expect_char('}');{
            let next_ident = t.peek_next_word();
            match next_ident {
                "if" => {
                    t.next();
                    self.body.push(ValidInCodeBlock::IfStartMarker);
                    let cur_body_stack_pos = self.body.len()-1;
                    if t.optionaly_expect_char('(') {
                        let expression = Expression::new(t, ')', '{');
                        self.body.push(ValidInCodeBlock::Expression(expression));
                    }  else {
                        let expression = Expression::new(t, '¥', '{');
                        self.body.push(ValidInCodeBlock::Expression(expression));
                    }
                    self.parse_body(t);
                    self.body.push(ValidInCodeBlock::JumpIndex(cur_body_stack_pos));
                }
                "while" => {
                    t.next();
                    self.body.push(ValidInCodeBlock::WhileStartMarker);
                    let cur_body_stack_pos = self.body.len()-1;
                    if t.optionaly_expect_char('(') {
                        let expression = Expression::new(t, ')', '{');
                        self.body.push(ValidInCodeBlock::Expression(expression));
                    }  else {
                        let expression = Expression::new(t, '¥', '{');
                        self.body.push(ValidInCodeBlock::Expression(expression));
                    }
                        self.parse_body(t);
                    self.body.push(ValidInCodeBlock::JumpIndex(cur_body_stack_pos));
                }
                "const" => {
                    t.expect(TokenType::IDENTIFIER);
                    self.body.push(ValidInCodeBlock::Var(Var::new(t)));
                },
                "let" => {
                    t.expect(TokenType::IDENTIFIER);
                    self.body.push(ValidInCodeBlock::Var(Var::new(t)));
                }
                _ => {
                    let expression = Expression::new(t, '\n', '}');
                    self.body.push(ValidInCodeBlock::Expression(expression));
                }

            }
            t.eat_all_spaces();

        })
    }
    fn preview_scan(t: &mut Tokenizer) {
        use crate::previewScannerUtils::looks_like_identifier;
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
                    print!("{:?}\n", expression);
                }
                ValidInCodeBlock::FunctionCall(function_call) => {
                    print!("{:?}\n", function_call);
                }
                ValidInCodeBlock::Var(var) => {
                    print!("{:?}\n", var);
                }
                ValidInCodeBlock::JumpIndex(_) => {
                    print!("{:?}\n", '}');
                }
                ValidInCodeBlock::WhileStartMarker => {
                    print!("while (\n");
                }
                ValidInCodeBlock::IfStartMarker => {
                    print!("if (\n");
                }
                ValidInCodeBlock::HeadEndAndBodyStartMarker => {
                    print!("){}\n", '{');
                }
            }
        }
        self.return_type.display();
        println!("}}");
    }
}

impl CodeBlock for Function {
    fn get_body(&self) -> &Vec<ValidInCodeBlock> {
        &self.body
    }
    fn body_ptr(&mut self) -> &mut Vec<ValidInCodeBlock> {
        &mut self.body
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn test_function_parser() {
        let mut t = Tokenizer {
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "
            
            function sub(int a, int b){}



            ",
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
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "function sub(int a = 9, int b = 2 + 3){}



            ",
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
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "function sub(int a = 9, int b = 2 + 3){
                const int a = 9
                let int b = 2
                a = b+9
            }



            ",
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
