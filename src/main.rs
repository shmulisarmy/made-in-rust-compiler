mod If_parser;
mod previewScannerUtils;
mod utils;
mod while_parser;
use std::sync::{LazyLock, Mutex};


mod constants;
mod expression;
mod function_parser;
mod precedence_order;
mod project_basic_utils{
    pub mod token;
    pub mod tokenizer;

}

    mod libs{
    pub mod trie;
    pub mod mapTrie;
    pub mod linkedList;
    pub mod macros;
}


mod type_parser;

use project_basic_utils::token::*;
use project_basic_utils::tokenizer::*;

use expression::*;
use function_parser::*;

use crate::If_parser::If;
use crate::while_parser::While;
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

static Classes: LazyLock<Mutex<Vec<Class>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static Functions: LazyLock<Mutex<Vec<Function>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn main() {
    color_backtrace::install();
    let mut t = Tokenizer {
        file_name: file!(),
        start_line: line!() as usize,
        code: "class Person{
            int age = b + c * add(3*7)
            string name = 'John Doe'
            string email = \"hello world\"
        }

        function add(int a = 9, int b) Person<int?> {
            let int a = operation_map
            let function<(int, char), void>? callback = 0
        }


        while (a + b){
            a  = 9
            b = 2
        }

        if a+b{
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
            }
            "function" => {
                let _function = Function::new(&mut t);
                _function.display();
            }
            "while" => {
                let _while = While::new(&mut t);
                _while.display();
            }
            "if" => {
                let _if = If::new(&mut t);
                _if.display();
            }
            _ => {
                t.expect_char('\n');
            }
        }
        t.eat_all_spaces();
    }

    // expression::Expression::new(&mut t, ',', '\n');
}
