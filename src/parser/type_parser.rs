use crate::parser::expression::Expression;
use crate::project_basic_utils::token::TokenType;
use crate::project_basic_utils::tokenizer::Tokenizer;
use crate::until;
use crate::utils::{blue, green};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct Type_ {
    pub name: &'static str,
    pub sub_types: Vec<Type_>,
    pub is_optional: bool,
    pub is_pointer: bool,
}

impl Type_ {
    pub fn new(t: &mut Tokenizer) -> Self {
        let is_pointer = t.optionaly_expect_char('*');
        if t.optionaly_expect_char('[') {
            if t.optionaly_expect_char(']') {
                //@example: []int which is an array of ints
                return Self {
                    name: "array",
                    sub_types: vec![Type_::new(t)],
                    is_optional: t.optionaly_expect_char('?'),
                    is_pointer,
                };
            } else {
                //@example: [string]int which is a map of strings to ints
                let key_type = Type_::new(t);
                t.expect_char(']');
                let res = Self {
                    name: "map",
                    sub_types: vec![key_type, Type_::new(t)],
                    is_optional: t.optionaly_expect_char('?'),
                    is_pointer,
                };
                return res;
            }
        }

        if t.optionaly_expect_char('(') {
            let mut res = Self {
                name: "tuple",
                sub_types: vec![],
                is_optional: false,
                is_pointer,
            };
            until!(t.optionaly_expect_char(')');{
                res.sub_types.push(Type_::new(t));
                t.optionaly_expect_char(',');
            });
            if t.optionaly_expect_char('?') {
                res.is_optional = true;
            }
            return res;
        }

        let mut res = Self {
            name: t.expect(TokenType::IDENTIFIER),
            sub_types: vec![],
            is_optional: false,
            is_pointer,
        };
        if t.optionaly_expect_char('<') {
            until!(t.optionaly_expect_char('>'); {
                res.sub_types.push(Type_::new(t));
                t.optionaly_expect_char(',');
            });
        }
        if t.optionaly_expect_char('?') {
            res.is_optional = true;
        }
        res
    }
    pub fn display(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        match self.sub_types.len() {
            0 => return correct_coloring(&self.name),
            1 => {
                return format!(
                    "{}<{}>",
                    correct_coloring(&self.name),
                    self.sub_types[0].to_string()
                );
            }
            2 => {
                return format!(
                    "{}<{}, {}>",
                    correct_coloring(&self.name),
                    self.sub_types[0].to_string(),
                    self.sub_types[1].to_string()
                );
            }
            _ => {
                return format!(
                    "{}<{}, {}, {}> optional = {}",
                    correct_coloring(&self.name),
                    self.sub_types[0].to_string(),
                    self.sub_types[1].to_string(),
                    self.sub_types[2].to_string(),
                    self.is_optional
                );
            }
        }
    }
}

fn correct_coloring(s: &str) -> String {
    if BUILTINS.contains(&s) {
        return blue(&s.to_string());
    } else {
        return green(&s.to_string());
    }
}
static BUILTINS: [&str; 5] = ["int", "string", "char", "bool", "void"];

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn basic_test() {
        let mut t = Tokenizer {
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "
            Person<(int, char), [int]string<char>, []int>?
            ",
            parse_index: 0,
        };

        let _type = Type_::new(&mut t);
        _type.display();

        assert_eq!(_type.name, "Person");
        assert_eq!(_type.sub_types.len(), 3);

        assert_eq!(_type.sub_types[0].name, "tuple");
        assert_eq!(_type.sub_types[0].sub_types[0].name, "int");

        assert_eq!(_type.sub_types[1].name, "map");
        assert_eq!(_type.sub_types[1].sub_types[0].name, "int");
        assert_eq!(_type.sub_types[1].sub_types[1].name, "string");

        assert_eq!(_type.sub_types[2].name, "array");
        assert_eq!(_type.sub_types[2].sub_types[0].name, "int");
    }
}
