mod If_parser;
mod while_parser;
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
mod function_parser;

use token::*;
use tokenizer::*;

use expression::*;
use function_parser::*;

use crate::while_parser::While;
use crate::If_parser::If;
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


static Functions: LazyLock<Mutex<Vec<Function>>> = LazyLock::new(|| {
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

        function sub(int a = 9, int b = 2) {
            let int a
            let int b
        }


        while (a + b){
            a  = 9
            b = 2
        }

        if (a + b){
            a  = 9
            b = 2
        }

        "
        .to_string(),
        parse_index: 0,
    };
    // function add(int a = 9, int b = sub(3*7))


    while t.in_range() {
        
        match t.expect(TokenType::KEYWORD) {
            "class" => {
                
                let _class = Class::new(&mut t);
                _class.display();

            },
            "function" => {
                let _function = Function::new(&mut t);
                _function.display();
            },
            "while" => {
                let _while = While::new(&mut t);
                _while.display();
            },
            "if" => {
                let _if = If::new(&mut t);
                _if.display();
            },
            _ => {
                t.expect_char('\n');
            }
        }
        t.eat_all_spaces();
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












