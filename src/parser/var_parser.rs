use crate::parser::expression::Expression;
use crate::parser::expression::ExpressionPiece;
use crate::parser::type_parser::Type_;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;
use crate::utils::red;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Var {
    pub name: &'static str,
    pub type_: Type_,
    pub default_value: Expression,
}

impl Var {
    pub fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let type_ = Type_::new(t);
        let name = t.expect(TokenType::IDENTIFIER);
        if t.optionaly_expect_char('=') {
            let default_value = Expression::new(t, '\n', '}'); //} is bc for now this appears in a function body wich ends with }
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value,
            };
        } else {
            if t.current_char() != '\n' {
                t.expect_char('\n');
            }
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value: Expression(ExpressionPiece::Placeholder(false)),
            };
        }
    }
    fn preview_scan(t: &mut Tokenizer) {
        use crate::previewScannerUtils::*;
        if !looks_like_type(t) {
            let next_token = t.next();
            t.user_error(
                next_token.start_index,
                next_token.start_index + next_token.value.len(),
            );
            panic!("{}", red("expected type".to_string()));
        }
    }
    pub fn display(&self) {
        println!("Var {} {}", self.type_.to_string(), self.name);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::parser::expression::Expression;
    use crate::parser::expression::ExpressionPiece;
    use crate::parser::type_parser::Type_;
    use crate::project_basic_utils::token::TokenType;
    use crate::project_basic_utils::tokenizer::Tokenizer;

    #[test]
    fn test_var_with_default_value() {
        let mut t = Tokenizer {
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "int a = 42\n",
            parse_index: 0,
        };

        let var = Var::new(&mut t);
        assert_eq!(var.name, "a");
        assert_eq!(var.type_.name, "int");
        // The default_value should not be a placeholder
        match &var.default_value {
            Expression(ExpressionPiece::Placeholder(false)) => panic!("Should not be placeholder"),
            _ => {}
        }
    }

    #[test]
    fn test_var_without_default_value() {
        let mut t = Tokenizer {
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "int b\n",
            parse_index: 0,
        };

        let var = Var::new(&mut t);
        assert_eq!(var.name, "b");
        assert_eq!(var.type_.name, "int");
        // The default_value should be a placeholder
        match &var.default_value {
            Expression(ExpressionPiece::Placeholder(false)) => {}
            _ => panic!("Should be placeholder"),
        }
    }

    #[test]
    fn post_type_system_upgrade_var_test() {
        let mut t = Tokenizer {
            mutex: Mutex::new(()),
            file_name: file!(),
            start_line: line!() as usize,
            code: "
                const []int  a = 9


            ",
            parse_index: 0,
        };

        assert_eq!(t.expect(TokenType::KEYWORD), "const");
        let var = Var::new(&mut t);
        var.display();
        assert_eq!(var.name, "a");
    }
}
