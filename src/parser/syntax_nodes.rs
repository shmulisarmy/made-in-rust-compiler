use crate::parser::class_parser::Class;
use crate::parser::function_parser::{Function, Param};
use crate::parser::var_parser::Var;
use crate::parser::while_parser::While;
use crate::parser::If_parser::If;
use crate::reference_validation::DeclarationInfo;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxNode<'a> {
    Class(Rc<RefCell<Class<'a>>>),
    Function(Rc<RefCell<Function<'a>>>),
    Var(Rc<RefCell<Var<'a>>>),
    While(Rc<RefCell<While<'a>>>),
    If(Rc<RefCell<If<'a>>>),
    CodeBlock {
        statements: Vec<SyntaxNode<'a>>,
    },
    VariableDeclaration {
        var_name: crate::project_basic_utils::token::Token,
        var_type: crate::parser::type_parser::Type_,
        value: Box<SyntaxNode<'a>>,
    },
    ParameterDeclaration(Rc<RefCell<Param<'a>>>),
}
