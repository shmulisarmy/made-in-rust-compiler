mod previewScannerUtils;
mod utils;
use std::sync::{LazyLock, Mutex};


mod constants;


mod parser;
mod precedence_order;
mod project_basic_utils;

mod libs;



use project_basic_utils::token::*;
use project_basic_utils::tokenizer::*;


use parser::function_parser::*;
use parser::var_parser::Var;

// eparser::num SyntaxNode{
//     Class(Class),
//     Function(Function),
// }

// enum ExpressionPiece{
//     FunctionCall(FunctionCall),
//     Variable(String),
//     // StringLiteral(Literal),
//     // NumberLiteral(Literal),
// }

use parser::expression::Expression;
use parser::class_parser::Class;

use crate::parser::while_parser::While;
use crate::parser::If_parser::If;

static Classes: LazyLock<Mutex<Vec<Class>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static Functions: LazyLock<Mutex<Vec<Function>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static Vars: LazyLock<Mutex<Vec<Var>>> = LazyLock::new(|| Mutex::new(Vec::new()));



enum SyntaxNode{
    Class(Class),
    Function(Function),
    Var(Var),
    While(While),
    If(If),
}


pub static PARSE_CONTEXT: LazyLock<Mutex<Vec< SyntaxNode>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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

        const int a = 9

        function add(int a = 9, int b) Person<int?> {
            let int a = operation_map
            let function<(int, char), void>? callback = 0

            while (a + b){
                a  = 9
                b = 2

                if a+b{
                    a  = 9
                    b = 2

                    while (a + b){
                        a  = 9
                        b = 2
                    }
                }
                
            }

            if a+b{
                a  = 9
                b = 2
            }
        }

        function main(int a){
            do_stuff()
        }
        function do_stuff(){
        
        }



        

        "
        .to_string(),
        parse_index: 0,
    };
    // function add(int a = 9, int b = sub(3*7))


    let mut parse_context: std::sync::MutexGuard<'_, Vec<SyntaxNode>> = PARSE_CONTEXT.lock().unwrap();

    while t.in_range() {
        match t.expect(TokenType::KEYWORD) {
            "class" => {
                let class_val = Class::new(&mut t, &mut parse_context);
                class_val.display();
                Classes.lock().unwrap().push(class_val);
            }
            "function" => {
                let function_val = Function::new(&mut t, &mut parse_context);
                function_val.display();
                Functions.lock().unwrap().push(function_val);
            }
            "const" => {
                let _var = Var::new(&mut t);
                _var.display();
                Vars.lock().unwrap().push(_var);

            }
            "let" => {
                let _var = Var::new(&mut t);
                _var.display();
                Vars.lock().unwrap().push(_var);
            }

            _ => {
                t.expect_char('\n');
            }
        }
        t.eat_all_spaces();
    }

    // expression::Expression::new(&mut t, ',', '\n');
}
