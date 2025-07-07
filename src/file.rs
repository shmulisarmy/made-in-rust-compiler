use crate::{parser::{class_parser::Class, code_block::{CodeBlock, ValidInCodeBlock}, function_parser::Function, type_parser::Type_, var_parser::Var}, utils::green};

pub struct File {
    pub functions: Vec<Function>,       
    pub classes: Vec<Class>,
    pub variables: Vec<Var>,
    pub builtins: Vec<Type_>,
}


impl File{
    pub fn new ()-> Self {
        Self {
            functions: Vec::new(),
            classes: Vec::new(),
            variables: Vec::new(),
            builtins: vec![
                Type_ {
                    name: "int".to_string(),
                    sub_types: Vec::new(),
                    is_optional: false,
    is_pointer: false,

                },
                Type_ {
                    name: "string".to_string(),
                    sub_types: Vec::new(),
                    is_optional: false,
    is_pointer: false,

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
                    panic!("type {} (used as field {} of class {}) is unknown to the compiler", field.type_.to_string(), field.name,green(&_class.name));
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



            let code_block_context: Vec<&dyn CodeBlock> = vec![function];
           
        }



    }


    pub fn typeCheck(&self){
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
