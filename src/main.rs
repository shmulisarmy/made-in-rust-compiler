mod file;

mod previewScannerUtils;
mod utils;
use std::any::TypeId;
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

use crate::file::File;
use crate::parser::code_block::{self, ValidInCodeBlock};
use crate::parser::type_parser::Type_;








fn main() {
    color_backtrace::install();
    let mut t = Tokenizer {
        file_name: file!(),
        start_line: line!() as usize,
        code: "
        


        class Engine{
            int id
            string repair_station
        }

        class Car{
            Engine engine
            int age = b + c * add(3*7)
            string name = 'John Doe'
            string email = \"hello world\"
        }


        function do(){
            while (a + b){
                a  = 9
                b = 2
            }
        }

        
        class Person{
            Car car
            Cat age = b + c * add(3*7)
            string name = 'John Doe'
            string email = \"hello world\"
        }


        class Cat{
            string name
        }

        function add(int a = 9, int b) Person<int?> {
            let int a = operation_map
            let *Person? shmuli = add()
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



    let mut this_file = File::new();

    while t.in_range() {
        match t.expect(TokenType::KEYWORD) {
            "class" => {
                let _class = Class::new(&mut t);
                (&_class).display();
                this_file.classes.push(_class);
            }
            "function" => {
                let _function = Function::new(&mut t);
                (&_function).display();
                this_file.functions.push(_function);
            }
            "const" => {
                let _var = Var::new(&mut t);
                (&_var).display();
                this_file.variables.push(_var);
            }
            "let" => {
                let _var = Var::new(&mut t);
                (&_var).display();
                this_file.variables.push(_var);
            }

            _ => {
                t.expect_char('\n');
            }
        }
        t.eat_all_spaces();
    }


    this_file.typeCheck();

    // expression::Expression::new(&mut t, ',', '\n');
}
