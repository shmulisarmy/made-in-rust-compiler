use crate::libs::linkedList;
use crate::precedence_order::absorb_neighbors;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;

use crate::comp;
use crate::until;

// Define FunctionCall here since it's used in this module
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct FunctionCall {
    pub name: String,
    pub params: Vec<Expression>,
}

pub fn OperatorToString(ep: &ExpressionPiece) -> String {
    if let ExpressionPiece::Operator(op) = ep {
        op.clone()
    } else {
        panic!("not an operator");
    }
}

impl FunctionCall {
    fn new(name: String, params: Vec<Expression>) -> Self {
        Self { name, params }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ExpressionPiece {
    FunctionCall(FunctionCall),
    Variable(String),
    StringLiteral(String),
    NumberLiteral(String),
    Operator(String),
    Placeholder(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
// ExpressionPiece is the what the parser uses internally, when you see ExpressionPiece getting passed around that means its not done making the syntax tree
pub struct Expression(pub ExpressionPiece);

impl Expression {
    pub fn new(t: &mut Tokenizer, separator: char, scope_ender: char) -> Self {
        //todo: remove the idea of the separator, where we eat up the token and just have tokens that when we wee we stop and dont take any action on. if we wanna eat that token wele do it form the call site
        println!("about to parse expression");
        use crate::libs::linkedList::*;
        let mut tokens = LinkedList::new();

        until!(
            t.optionaly_expect_char(separator) || t.current_char() == scope_ender; {
                tokens.append(parse_next_expression_piece(t));
            }
        );


        //display
        println!("about to display expression tokens");
        let mut cur = tokens.head;
        while let Some(node_index) = cur {
            match &tokens.storage[node_index].value {
                ExpressionPiece::FunctionCall(f) => println!("FunctionCall {}", f.name),
                ExpressionPiece::Variable(v) => println!("Variable {}", v),
                ExpressionPiece::StringLiteral(v) => println!("StringLiteral {}", v),
                ExpressionPiece::NumberLiteral(v) => println!("NumberLiteral {}", v),
                ExpressionPiece::Operator(v) => println!("Operator {}", v),
                ExpressionPiece::Placeholder(v) => println!("Placeholder {}", v),
            }
            cur = tokens.storage[node_index].next;
        }
        println!("done displaying expression tokens");
        


        // by scope_ender we make sure that when we do the check we don't eat up the char bc we want the parent syntaxNode to see and know to stop
        // println!("about to display expression tokens");
        // for token in tokens.iter() {
        //     dbg!(&token);
        // }

        let mut current = tokens.head;
        while let Some(node_index) = current {
            if let ExpressionPiece::Operator(op) = &tokens.storage[node_index].value {
                absorb_neighbors(&mut tokens, node_index);
            }
            current = tokens.storage[node_index].next;
        }

        if tokens.len() == 0 {
            return Self(ExpressionPiece::Placeholder(true));
        }

            
        dbg!(&tokens.storage[tokens.head.unwrap()].value);
        println!("done parsing expression");
        Self(tokens.storage[tokens.head.unwrap()].value.clone())
    }
}

fn parse_next_expression_piece(t: &mut Tokenizer) -> ExpressionPiece {
    let token = t.next();
    dbg!(&token);
    // dbg!(token);
    if token.type_ == TokenType::IDENTIFIER {
        if t.optionaly_expect_char('(') {
            return ExpressionPiece::FunctionCall(FunctionCall::new(
                token.value,
                comp![
                    Expression::new(t, ',', ')');
                    until t.optionaly_expect_char(')')
                ],
            ));
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
