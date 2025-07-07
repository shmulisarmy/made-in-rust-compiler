use core::panic;
use crate::file::*;
use crate::parser::code_block;
use crate::parser::expression::Expression;
use crate::parser::expression::FunctionCall;
use crate::parser::function_parser::ValidInFunctionBody;
use crate::parser::type_parser::Type_;
use crate::project_basic_utils::tokenizer::Tokenizer;
use crate::parser::var_parser::Var;
use crate::project_basic_utils::token::TokenType;



/**
 * 
 * before i did a refactor when a function had any of [if, while] it would make a an entire object for that 
 * point of refactor: reduce the number of allocations, by just using WhileMarker and IfMarker instead of a whole object, and to speed up the part where where byte code is generated, by looking for the var reference when using it in a function
 * 
 */


#[derive(Debug)]
pub enum ValidInCodeBlock{
    Expression(Expression),
    FunctionCall(FunctionCall),
    Var(Var),
    // ScopeJumpIndex(usize)
    //once i learned that the way to do it is in a stack like manner
    WhileStartMarker,
    IfStartMarker,
    HeadEndAndBodyStartMarker,
    JumpIndex(usize),
}


pub trait CodeBlock{
    fn body_ptr(&mut self) -> &mut Vec<ValidInCodeBlock>;
    fn get_body(&self) -> & Vec<ValidInCodeBlock>;


    fn contains_nested_bracket_scope(&self) -> bool {
        //bracket_scope is a scope defined by {}, (if, while, function, etc)
       todo!("not implemented")
    }
}






