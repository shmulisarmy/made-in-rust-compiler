use core::panic;

use crate::parser::expression::Expression;
use crate::parser::expression::FunctionCall;
use crate::parser::function_parser::ValidInFunctionBody;
use crate::project_basic_utils::tokenizer::Tokenizer;
use crate::utils::yellow;
use crate::parser::var_parser::Var;
use crate::parser::If_parser::If;
use crate::parser::while_parser::While;
use crate::project_basic_utils::token::TokenType;


#[derive(Debug)]
pub enum ValidInCodeBlock{
    Expression(Expression),
    FunctionCall(FunctionCall),
    Var(Var),
    While(While),
    If(If),
    // ScopeJumpIndex(usize)
}


pub trait CodeBlock{
    fn body_ptr(&mut self) -> &mut Vec<ValidInCodeBlock>;
    fn get_body(&self) -> & Vec<ValidInCodeBlock>;
    fn parse_body(&mut self, t: &mut Tokenizer) {
        t.expect_char('{');
        let parse_index_of_code_block = t.parse_index-1; // the minus one is because we ate up the char when we called expect_char
        t.eat_all_spaces();
        dbg!(t.peek_next_word());
        while !t.optionaly_expect_char('}') {
            t.eat_all_spaces();
            dbg!(t.peek_next_word());
            if t.current_char() == '}' {
                   panic!("found token const or let"); 
            }
            if !t.in_range() {
                let (line, column) = t.find_line_and_column(parse_index_of_code_block);
                t.user_error(parse_index_of_code_block, parse_index_of_code_block + 1);
                if self.contains_nested_bracket_scope() {   
                    println!("{} perhaps you didn't forget to close this scope but rather a nested scope that the compiler mistook its closing bracket for the one that was meant to close this scope", yellow("hint:".to_string()));
                }
                panic!("you have an unclosed code block that was opened at {}:{}:{}", t.file_name, line+t.start_line, column);
            }
            match t.peek_next_word() {
                "const" => {
                    println!("found token const");
                    t.expect(TokenType::KEYWORD);
                    let var = Var::new(t);
                    self.body_ptr().push(ValidInCodeBlock::Var(var));
                }
                "let" => {
                    println!("found token let");
                    t.expect(TokenType::KEYWORD);
                    let var = Var::new(t);
                    self.body_ptr().push(ValidInCodeBlock::Var(var));
                }
                "while" => {
                    assert_eq!(t.expect(TokenType::KEYWORD).to_string(), "while");
                    let while_val = While::new(t);
                    while_val.display();
                    self.body_ptr().push(ValidInCodeBlock::While(while_val));
                }
                "if" => {
                    assert_eq!(t.expect(TokenType::KEYWORD).to_string(), "if");
                    let if_val = If::new(t);
                    if_val.display();
                    self.body_ptr().push(ValidInCodeBlock::If(if_val));
                }
                "}" => {
                    panic!("didnt find token const or let");
                    t.user_error(t.parse_index, t.parse_index + 1);
                    println!("didnt find token const or let");
                    let expression = Expression::new(t, '\n', '}');
                    self.body_ptr().push(ValidInCodeBlock::Expression(expression));
                }
                "class" | "function" => {
                    let next_word_size = t.peek_next_word().len();
                    t.user_error(t.parse_index, t.parse_index+next_word_size);
                    panic!("cannot declare function or class inside another function");
                }
                _ => {
                    dbg!(t.peek_next_word());
                    // t.user_error(t.parse_index, t.parse_index + 1);
                    // println!("didnt find token const or let");
                    let expression = Expression::new(t, '\n', '}');
                    self.body_ptr().push(ValidInCodeBlock::Expression(expression));
                }
            }
            t.eat_all_spaces();
        }
        t.eat_all_spaces();
    }


    fn contains_nested_bracket_scope(&self) -> bool {
        //bracket_scope is a scope defined by {}, (if, while, function, etc)
        for child_node in self.get_body() {
            match child_node {
                ValidInCodeBlock::While(_) => {
                    return  true;
                }
                ValidInCodeBlock::If(_) => {
                    return  true;
                },
                _ => {}
            }
        }
        return false;
    }

}








