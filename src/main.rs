mod file;


mod code_gen;
mod previewScannerUtils;
mod utils;
use std::any::TypeId;
use std::os;
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

use parser::class_parser::Class;
use parser::expression::Expression;

use crate::file::File;
use crate::parser::code_block::{self, ValidInCodeBlock};
use crate::parser::type_parser::Type_;

fn main() {
    color_backtrace::install();
    let mut t = Tokenizer {
        file_name: file!(),
        start_line: line!() as usize,
        code: "
        
        function add(int a = 9, int b) Person<int?> {
            let int a = operation_map
            let *Person? shmuli = add(1, 7)
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

        


        function do_some_stuff(){
            while (a + b){
                a  = 9
                b = 2
            }
        }


        class Engine{
            int id
            string repair_station

            function vroom(){
                }
            function fix(){
            }

           
        }

        class Car{
            Engine engine
            int age = b + c * add(3*7)
            string name = 'John Doe'
            string email = \"hello world\"

            function drive(){
                vroom()
            }
        }
        class Person{
            Engine engine
            int age = b + c * add(3*7)
            string name = 'John Doe'
            string email = \"hello world\"
        }

        ",
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

    this_file.type_check();
    let js_code = this_file.generate_javascript_code(0);
    let cpp_header_file = this_file.generate_cpp_header_file();
    let cpp_code = this_file.generate_cpp_code(0);

    

    use std::fs::File as StdFile;
    use std::fs;
    use std::io::Write;

    // Create output directory if it doesn't exist
    fs::create_dir_all("output").expect("Unable to create output directory");

    let mut javascript_output_file = StdFile::create("output/file.js").expect("Unable to create file");
    let mut cpp_header_output_file = StdFile::create("output/file.hpp").expect("Unable to create file");
    let mut cpp_code_output_file = StdFile::create("output/file.cpp").expect("Unable to create file");
    javascript_output_file.write_all(js_code.as_bytes()).expect("Unable to write data");
    cpp_header_output_file.write_all(cpp_header_file.as_bytes()).expect("Unable to write data");
    cpp_code_output_file.write_all(cpp_code.as_bytes()).expect("Unable to write data");


}
