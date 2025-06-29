mod linkedList;
mod precedence_order;
mod expression;
mod constants;
mod macros;
mod mapTrie;
mod token;
mod tokenizer;
mod trie;

use std::string;

use mapTrie::MapTrie;
use token::*;
use tokenizer::*;
use trie::Trie;

use expression::*;
// enum SyntaxNode{
//     Class(Class),
//     Function(Function),
// }

// enum ExpressionPiece{
//     FunctionCall(FunctionCall),
//     Variable(String),
//     // StringLiteral(Literal),
//     // NumberLiteral(Literal),
// }










#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
struct Field {
    name: String,
    type_: String,
    // default_value: ,
}

impl Field {
    fn new(t: &mut Tokenizer) -> Self {
        let res = Self {
            type_: t.expect(TokenType::IDENTIFIER),
            name: t.expect(TokenType::IDENTIFIER),
        };
        t.eat_all_spaces();
        res
    }
}

struct Class {
    name: String,
    fields: Vec<Field>,
}

impl Class {
    fn new(t: &mut Tokenizer) -> Self {
        let name = t.expect(TokenType::IDENTIFIER);
        dbg!(name);
        t.expect_char('{');
        t.eat_all_spaces();
        let fields = comp![Field::new(t); until t.optionaly_expect_char('}')];
        Self { name, fields }
    }

    fn display(&self) {
        println!("Class {} {{", self.name);
        for field in &self.fields {
            println!("    {} {}", field.type_, field.name);
        }
        println!("}}");
    }
}



fn main() {
    color_backtrace::install();

    let mut t = Tokenizer {
        code: "
        class Person{
            int age = a = b + c * add(9, 3*7)
            string name
            string email
        }
        "
        .to_string(),
        parse_index: 0,
    };

    if t.expect(TokenType::KEYWORD) == "class" {
        let _class = Class::new(&mut t);
        _class.display();
    }

    // expression::Expression::new(&mut t, ',', '\n');
}


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












#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_parser() {
        let mut t = Tokenizer {
            code: "class Person{
            int age
            string name
            string email
        }"
            .to_string(),
            parse_index: 0,
        };
    
        if t.expect(TokenType::KEYWORD) == "class" {
            let _class = Class::new(&mut t);
            _class.display();
            assert_eq!(_class.name, "Person");
            assert_eq!(_class.fields.len(), 3);

            assert_eq!(_class.fields[0].type_, "int");
            assert_eq!(_class.fields[0].name, "age");

            assert_eq!(_class.fields[1].type_, "string");
            assert_eq!(_class.fields[1].name, "name");

            assert_eq!(_class.fields[2].type_, "string");
            assert_eq!(_class.fields[2].name, "email");
        }
    }
}