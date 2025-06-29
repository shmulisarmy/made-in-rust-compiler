use crate::token::TokenType;
use crate::tokenizer::Tokenizer;

use crate::expression::Expression;
use crate::until;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct Type_ {
    pub name: String,
    pub sub_types: Vec<Type_>,
}

impl Type_ {
    pub fn new(t: &mut Tokenizer) -> Self {
        if t.optionaly_expect_char('[') {
            if t.optionaly_expect_char(']') {
                //@example: []int which is an array of ints
                return Self {
                    name: "array".to_string(),
                    sub_types: vec![Type_::new(t)],
                };
            } else {
                //@example: [string]int which is a map of strings to ints
                let key_type = Type_::new(t);
                t.expect_char(']');
                let res = Self {
                    name: "map".to_string(),
                    sub_types: vec![key_type, Type_::new(t)],
                };
                return res;
            }
        }

        if t.optionaly_expect_char('(') {
            let mut res=  Self {
                name: "tuple".to_string(),
                sub_types: vec![],
            };
            until!(t.optionaly_expect_char(')');{
                res.sub_types.push(Type_::new(t));
                t.optionaly_expect_char(',');
            });
            return  res;
        }

        let mut res = Self {
            name: t.expect(TokenType::IDENTIFIER).to_string(),
            sub_types: vec![],
        };
        if t.optionaly_expect_char('<') {
            until!(t.optionaly_expect_char('>'); {
                res.sub_types.push(Type_::new(t));
                t.optionaly_expect_char(',');
            });
        }
        res
    }
    pub fn display(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        match self.sub_types.len() {
            0 => return self.name.clone(),
            1 => return format!("{}<type: {}>", self.name, self.sub_types[0].to_string()),
            2 => {
                return format!(
                    "{}<type: {}, type: {}>",
                    self.name,
                    self.sub_types[0].to_string(),
                    self.sub_types[1].to_string()
                );
            }
            _ => {
                return format!(
                    "{}<type: {}, type: {}, type: {}>",
                    self.name,
                    self.sub_types[0].to_string(),
                    self.sub_types[1].to_string(),
                    self.sub_types[2].to_string()
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut t = Tokenizer {
            code: "    
            Person<(int), [int]string<char>, []int>
            "
            .to_string(),
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
