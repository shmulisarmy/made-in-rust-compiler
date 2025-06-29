use std::sync::{LazyLock, Mutex};


mod precedence_order;
mod expression;
mod constants;
mod macros;
mod token;
mod tokenizer;
mod trie;
mod mapTrie;
mod linkedList;

use token::*;
use tokenizer::*;

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





use crate::expression::Expression;
mod class_parser;
use class_parser::Class;







static Classes: LazyLock<Mutex<Vec<Class>>> = LazyLock::new(|| {
    Mutex::new(Vec::new())
});



fn main() {
    color_backtrace::install();

    let mut t = Tokenizer {
        code: "
        class Person{
            int age = b + c * add(3*7)
            string name
            string email
        }

        function add(int a, int b){
            return a + b
        }



        "
        .to_string(),
        parse_index: 0,
    };

    if t.expect(TokenType::KEYWORD) == "class" {
        let _class = Class::new(&mut t);
        _class.display();
    }

    // if t.expect(TokenType::KEYWORD) == "function" {
    //     let _class = Class::new(&mut t);
    //     _class.display();
    // }

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












