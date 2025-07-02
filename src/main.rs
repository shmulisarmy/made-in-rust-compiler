
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

use crate::parser::code_block;
use crate::parser::type_parser::Type_;






struct File {
    functions: Vec<Function>,       
    classes: Vec<Class>,
    variables: Vec<Var>,
    builtins: Vec<Type_>,
}


impl File{
    fn new ()-> Self {
        Self {
            functions: Vec::new(),
            classes: Vec::new(),
            variables: Vec::new(),
            builtins: vec![
                Type_ {
                    name: "int".to_string(),
                    sub_types: Vec::new(),
                    is_optional: false,
                },
                Type_ {
                    name: "string".to_string(),
                    sub_types: Vec::new(),
                    is_optional: false,
                },
            ],
        }
    }




    fn typeCheckVars(&self){
        for var in &self.variables{
            if !self.is_allowed_type(&var.type_){
                panic!("Variable {} of type {} is not allowed", var.name, var.type_.to_string());
            }
        }
    }

    fn typeCheckClasses(&self){
        for _class in &self.classes{
            for field in &_class.fields{
                if !self.is_allowed_type(&field.type_){
                    panic!("type {} (used as field {} of class {}) is unknown for the compiler", field.name, field.type_.to_string(), _class.name);
                }
            }
        }
    }

    fn typeCheckFunctions(&self){
        for function in &self.functions{
            for param in &function.params{
                if !self.is_allowed_type(&param.type_){
                    panic!("type {} (used as param {} of function {}) is unknown for the compiler", param.name, param.type_.to_string(), function.name);
                }
            }
            if function.return_type.name != "void" && !self.is_allowed_type(&function.return_type){
                panic!("type {} (used as return type of function {}) is unknown for the compiler", function.return_type.name, function.name);
            }



            


        }
    }


    fn typeCheck(&self){
        self.typeCheckVars();
        self.typeCheckClasses();
        self.typeCheckFunctions();
    }



    fn is_allowed_type(&self, type_: &Type_)-> bool {
        for class in &self.classes{
            if class.name == type_.name{
                return true;
            }
        }
        if self.builtins.contains(type_){
            return true;
        }
        return false;
    }
}


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

        
        class Person{
            Car car
            int age = b + c * add(3*7)
            string name = 'John Doe'
            string email = \"hello world\"
        }

        function add(int a = 9, int b) Person<int?> {
            let int a = operation_map
            let Person shmuli = add()
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
