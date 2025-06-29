use crate::token::{TokenType};
use crate::tokenizer::{Tokenizer};


macro_rules! comp {
    [$value:expr; until $cond:expr] => {
        {
            let mut res = Vec::new();
            while !$cond {
                res.push($value);
            }
            res
        }
    };
    }



// Define FunctionCall here since it's used in this module
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
struct FunctionCall {
    name: String,
    params: Vec<Expression>,
}

impl FunctionCall {
    fn new(name: String, params: Vec<Expression>) -> Self {
        Self { name, params }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum  ExpressionPiece {
    FunctionCall(FunctionCall),
    Variable(String),
    StringLiteral(String),
    NumberLiteral(String),
    Operator(String),
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Expression{
    tree_root:  ExpressionPiece,
}

impl Expression {
    pub fn new(t: &mut Tokenizer, separator: char, scope_ender: char) -> Self {
        println!("about to parse expression");
        let tokens = comp![
            parse_next_expression_piece(t);
            until ( t.optionaly_expect_char(separator) || t.current_char() == scope_ender)
            ];
            // by scope_ender we make sure that when we do the check we don't eat up the char bc we want the parent syntaxNode to see and know to stop
            // println!("about to display expression tokens");
            // for token in tokens.iter() {
                //     dbg!(&token);
                // }
            for piece in tokens.iter() {
                dbg!(&piece);
            }
                dbg!(&tokens.len());
                println!("done parsing expression");
        Self { tree_root: tokens[0].clone() }
    }
}



fn parse_next_expression_piece(t: &mut Tokenizer) -> ExpressionPiece {
    let token = t.next();
    dbg!(&token);
    // dbg!(token);
    if token.type_ == TokenType::IDENTIFIER {
        if t.optionaly_expect_char('(') {
            return ExpressionPiece::FunctionCall(FunctionCall::new(token.value, comp![
                Expression::new(t, ',', ')');
                until t.optionaly_expect_char(')')
            ]));
        } else {
            return ExpressionPiece::Variable(token.value);
        }
    } 
    if token.type_ == TokenType::STRING {
        return ExpressionPiece::StringLiteral(token.value);
    }
    if token.type_ == TokenType::NUMBER {
        return ExpressionPiece::NumberLiteral(token.value);
    }
    if token.type_ == TokenType::OPERATOR {
        return ExpressionPiece::Operator(token.value);
    }    
    println!("about to show token");
    dbg!(&token);
    todo!()
}



















