
use crate::parser::expression::Expression;
use crate::parser::expression::ExpressionPiece;
use crate::parser::function_parser::Function;
use crate::parser::type_parser::Type_;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;
use crate::until;
use crate::utils::red;

use crate::comp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Field {
    pub name: &'static str,
    pub type_: Type_,
    pub default_value: Expression,
}

impl Field {
    fn new(t: &mut Tokenizer) -> Self {
        let type_ = Type_::new(t);
        let name = t.expect(TokenType::IDENTIFIER);
        if t.optionaly_expect_char('=') {
            let default_value = Expression::new(t, '\n', '\n');
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value,
            };
        } else {
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value: Expression(ExpressionPiece::Placeholder(false)),
            };
        }
    }
}

pub struct Class {
    pub name: &'static str,
    pub fields: Vec<Field>,
    pub methods: Vec<Function>,
}

impl Class {
    pub fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let mut res = Self {
            name: t.expect(TokenType::IDENTIFIER),
            fields: vec![],
            methods: vec![],
        };
        t.expect_char('{');
        t.eat_all_spaces();
        until!(t.optionaly_expect_char('}'); {
            t.eat_all_spaces();
            if t.peek_next_word() == "function" {
                t.next(); //eat up the 'function' keyword
                res.methods.push(Function::new(t));
            } else {
                res.fields.push(Field::new(t));
            }
            t.eat_all_spaces(); 
        });
        res
    }

    pub fn display(&self) {
        println!("Class {} {{", self.name);
        for field in &self.fields {
            println!("    {} {}", field.type_.to_string(), field.name);
        }
        println!("}}");
    }

    fn preview_scan(t: &mut Tokenizer) {
        use crate::previewScannerUtils::*;
        if !looks_like_identifier(t) {
            let next_token = t.next();
            t.user_error(
                next_token.start_index,
                next_token.start_index + next_token.value.len(),
            );
            panic!("{}", red("expected identifier (class name)".to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn test_class_parser() {
        let mut t = Tokenizer {
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "class Person{
            int age
            string name
            string email
        }",
            parse_index: 0,
        };

        if t.expect(TokenType::KEYWORD) == "class" {
            let _class = Class::new(&mut t);
            _class.display();
            assert_eq!(_class.name, "Person");
            assert_eq!(_class.fields.len(), 3);

            assert_eq!(_class.fields[0].type_.name, "int");
            assert_eq!(_class.fields[0].name, "age");

            assert_eq!(_class.fields[1].type_.name, "string");
            assert_eq!(_class.fields[1].name, "name");

            assert_eq!(_class.fields[2].type_.name, "string");
            assert_eq!(_class.fields[2].name, "email");
        }
    }
}
