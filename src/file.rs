use std::{fmt::format, vec};

use crate::{
    parser::{
        class_parser::Class,
        code_block::CodeBlock,
        function_parser::Function,
        type_parser::Type_,
        var_parser::Var,
    }, project_basic_utils::{token::TokenType, tokenizer::Tokenizer}, utils::green
};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum CompilationStage {
    Start = 0,
    Parsing = 1,
    TypeChecking = 2,
    CodeGeneration = 3,
    Done = 4,
}

pub struct File {
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
    pub variables: Vec<Var>,
    pub builtins: Vec<Type_>,
    pub tokenizer: Tokenizer,
    pub stage: CompilationStage,
}

impl File {
    pub fn new(t: Tokenizer) -> Self {
        Self {
            tokenizer: t,
            functions: Vec::new(),
            classes: Vec::new(),
            variables: Vec::new(),
            stage: CompilationStage::Start,
            builtins: vec![
                Type_ {
                    name: "int",
                    sub_types: Vec::new(),
                    is_optional: false,
                    is_pointer: false,
                },
                Type_ {
                    name: "Container", //just making sure that non built in generics work
                    sub_types: vec![Type_ {
                        name: "Tag",
                        sub_types: Vec::new(),
                        is_optional: false,
                        is_pointer: false,
                    }],
                    is_optional: false,
                    is_pointer: false,
                },
                Type_ {
                    name: "string",
                    sub_types: Vec::new(),
                    is_optional: false,
                    is_pointer: false,
                },
                Type_ {
                    name: "array",
                    sub_types: vec![Type_ {
                        name: "int",
                        sub_types: Vec::new(),
                        is_optional: false,
                        is_pointer: false,
                    }],
                    is_optional: false,
                    is_pointer: false,
                },
                Type_ {
                    name: "array",
                    sub_types: vec![Type_ {
                        name: "string",
                        sub_types: Vec::new(),
                        is_optional: false,
                        is_pointer: false,
                    }],
                    is_optional: false,
                    is_pointer: false,
                },
            ],
        }
    }

    fn type_check_vars(&self) {
        for var in &self.variables {
            if !self.is_allowed_type(&var.type_) {
                panic!(
                    "Variable {} of type {} is not allowed",
                    var.name,
                    var.type_.to_string()
                );
            }
        }
    }

    fn type_check_classes(&self) {
        for _class in &self.classes {
            for field in &_class.fields {
                if !self.is_allowed_type(&field.type_) {
                    panic!(
                        "type {} (used as field {} of class {}) is unknown to the compiler",
                        field.type_.to_string(),
                        field.name,
                        green(&_class.name.to_string())
                    );
                }
            }
        }
    }

    fn type_check_functions(&self) {
        for function in &self.functions {
            for param in &function.params {
                if !self.is_allowed_type(&param.type_) {
                    panic!(
                        "type {} (used as param {} of function {}) is unknown to the compiler",
                        param.name,
                        param.type_.to_string(),
                        function.name
                    );
                }
            }
            if function.return_type.name != "void" && !self.is_allowed_type(&function.return_type) {
                panic!(
                    "type {} (used as return type of function {}) is unknown for the compiler",
                    function.return_type.name, function.name
                );
            }

            let code_block_context: Vec<&dyn CodeBlock> = vec![function];
        }
    }

    pub fn type_check(&self) {
        self.type_check_vars();
        self.type_check_classes();
        self.type_check_functions();
    }

    fn is_allowed_type(&self, type_: &Type_) -> bool {
        for class in &self.classes {
            if class.name == type_.name {
                return true;
            }
        }
        if self.builtins.contains(type_) {
            return true;
        }
        return false;
    }






    pub fn generate_syntax_tree_from_source_code(&mut self) {

        while self.tokenizer.in_range() {
            match self.tokenizer.expect(TokenType::KEYWORD) {
                "class" => {
                    let _class = Class::new(&mut self.tokenizer);
                    // (&_class).display(); //for debug like info
                    self.classes.push(_class);
                }
                "function" => {
                    let _function = Function::new(&mut self.tokenizer);
                    // (&_function).display(); //for debug like info
                    self.functions.push(_function);
                }
                "const" => {
                    let _var = Var::new(&mut self.tokenizer);
                    // (&_var).display(); //for debug like info
                    self.variables.push(_var);
                }
                "let" => {
                    let _var = Var::new(&mut self.tokenizer);
                    // (&_var).display(); //for debug like info
                    self.variables.push(_var);
                }
                token_string => {
                    self.tokenizer.expect_char('\n');
                    println!("unknown token: {}", token_string);
                }
            }
            self.tokenizer.eat_all_spaces();
        }

        
    }

    pub fn output_code_from_syntax_tree(&self) {
        
        let file_base_name = self.tokenizer.file_name.split(".").next().unwrap();


        let js_code = self.generate_javascript_code(0);
        let cpp_header_file = self.generate_cpp_header_file();
        let cpp_code = self.generate_cpp_code(0);
    
        
    
        use std::fs::File as StdFile;
        use std::fs;
        use std::io::Write;
    
        // Create output directory if it doesn't exist
        fs::create_dir_all("output").expect("Unable to create output directory");
    
        let mut javascript_output_file = StdFile::create(format!("output/{}.js", file_base_name)).expect("Unable to create file");
        let mut cpp_header_output_file = StdFile::create(format!("output/{}.hpp", file_base_name)).expect("Unable to create file");
        let mut cpp_code_output_file = StdFile::create(format!("output/{}.cpp", file_base_name)).expect("Unable to create file");
        javascript_output_file.write_all(js_code.as_bytes()).expect("Unable to write data");
        cpp_header_output_file.write_all(cpp_header_file.as_bytes()).expect("Unable to write data");
        cpp_code_output_file.write_all(cpp_code.as_bytes()).expect("Unable to write data");
    }
}
