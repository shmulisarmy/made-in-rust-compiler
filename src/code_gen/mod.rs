use std::ffi::c_double;

use crate::parser::function_parser::Function;
use crate::parser::function_parser::Param;
use crate::parser::code_block::ValidInCodeBlock;
use crate::parser::expression::Expression;
use crate::parser::var_parser::Var;

type FunctionTokens = Function;
type ParamTokens = Param;
type VarTokens = Var;
type FunctionCallTokens = crate::parser::expression::FunctionCall;
type FileTokens = crate::file::File;
type ClassTokens = crate::parser::class_parser::Class;

impl FunctionTokens {
    pub fn function_header_generate_python_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let param_list = self.params.iter()
            .map(|param| param.generate_python_code(depth + 1))
            .collect::<Vec<String>>()
            .join(", ");
        format!("{}def {}({}):\n", indent, self.name, param_list)
    }

    pub fn function_body_generate_python_code(&self, depth: usize) -> String {
        self.generate_body_from_tokens(&self.body, depth)
    }

    fn generate_body_from_tokens(&self, tokens: &[ValidInCodeBlock], depth: usize) -> String {
        let mut output = String::new();
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                ValidInCodeBlock::WhileStartMarker => {
                    // Next should be condition, then body, then JumpIndex
                    if let Some(ValidInCodeBlock::Expression(cond)) = tokens.get(i + 1) {
                        output.push_str(&format!("{}while {}:\n", "    ".repeat(depth), self.expression_to_python(cond)));
                        // Find the body (from i+2 to JumpIndex)
                        let mut j = i + 2;
                        let mut body_tokens = Vec::new();
                        while j < tokens.len() {
                            if let ValidInCodeBlock::JumpIndex(_) = tokens[j] {
                                break;
                            }
                            body_tokens.push(tokens[j].clone());
                            j += 1;
                        }
                        output.push_str(&self.generate_body_from_tokens(&body_tokens, depth + 1));
                        i = j + 1; // Skip past JumpIndex
                        continue;
                    }
                }
                ValidInCodeBlock::IfStartMarker => {
                    // Next should be condition, then body, then JumpIndex
                    if let Some(ValidInCodeBlock::Expression(cond)) = tokens.get(i + 1) {
                        output.push_str(&format!("{}if {}:\n", "    ".repeat(depth), self.expression_to_python(cond)));
                        // Find the body (from i+2 to JumpIndex)
                        let mut j = i + 2;
                        let mut body_tokens = Vec::new();
                        while j < tokens.len() {
                            if let ValidInCodeBlock::JumpIndex(_) = tokens[j] {
                                break;
                            }
                            body_tokens.push(tokens[j].clone());
                            j += 1;
                        }
                        output.push_str(&self.generate_body_from_tokens(&body_tokens, depth + 1));
                        i = j + 1; // Skip past JumpIndex
                        continue;
                    }
                }
                ValidInCodeBlock::JumpIndex(_) => {
                    // Should be handled by the above logic, just skip
                    i += 1;
                    continue;
                }
                _ => {
                    output.push_str(&self.generate_statement_python(&tokens[i], depth));
                    i += 1;
                }
            }
        }
        output
    }
    
    fn generate_statement_python(&self, statement: &ValidInCodeBlock, mut depth: usize) -> String {
        let mut indent = "    ".repeat(depth); // we may extend this as we encounter a while marker
        match statement {
            ValidInCodeBlock::Expression(expr) => {
                format!("{}{}\n", indent, self.expression_to_python(expr))
            }
            ValidInCodeBlock::Var(var) => {
                format!("{}{}\n", indent, var.generate_python_code(depth))
            }
            ValidInCodeBlock::FunctionCall(func_call) => {
                format!("{}{}\n", indent, func_call.generate_python_code(depth))
            }
            ValidInCodeBlock::WhileStartMarker => {
                depth += 1;
                indent = "    ".repeat(depth);
                format!("{}while ", indent)
            }
            ValidInCodeBlock::IfStartMarker => {
                depth += 1;
                indent = "    ".repeat(depth);
                format!("{}if ", indent)
            }
            ValidInCodeBlock::JumpIndex(_) => {
                depth -= 1;
                indent = "    ".repeat(depth);
                String::new() // Handle jump logic if needed
            }
            ValidInCodeBlock::HeadEndAndBodyStartMarker => {
                format!("{}:\n", indent)
            }
        }
    }
    
    fn expression_to_python(&self, expr: &Expression) -> String {
        // Convert Expression to Python code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_python_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "None".to_string(),
        }
    }
}

impl ParamTokens {
    pub fn generate_python_code(&self, _depth: usize) -> String {
        match &self.default_value.0 {
            crate::parser::expression::ExpressionPiece::Placeholder(false) => {
                self.name.to_string()
            }
            _ => {
                // For now, just return the parameter name since we need to implement
                // proper expression conversion for default values
                self.name.to_string()
            }
        }
    }
}

impl VarTokens {
    pub fn generate_python_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let type_name = self.type_.name;
        let value_str = self.expression_to_python(&self.default_value);
        if type_name != "" && type_name != "None" && type_name != "_" {
            format!("{}{}: {} = {}", indent, self.name, type_name, value_str)
        } else {
            format!("{}{} = {}", indent, self.name, value_str)
        }
    }
    
    fn expression_to_python(&self, expr: &Expression) -> String {
        // Convert Expression to Python code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_python_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "None".to_string(),
        }
    }
}

impl FunctionCallTokens {
    pub fn generate_python_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let operator_names = ["=", "+", "-", "*", "/", "%", ">", "<", ">=", "<=", "==", "!=", "&&", "||", "^", "|", "&"]; // Add more as needed
        if operator_names.contains(&self.name) && self.params.len() == 2 {
            // Infix notation for binary operators
            let left = self.expression_to_python(&self.params[0]);
            let right = self.expression_to_python(&self.params[1]);
            format!("{}({} {} {})", indent, left, self.name, right)
        } else {
            let params = self.params.iter()
                .map(|param| self.expression_to_python(param))
                .collect::<Vec<String>>()
                .join(", ");
            format!("{}{}({})", indent, self.name, params)
        }
    }
    
    fn expression_to_python(&self, expr: &Expression) -> String {
        // Convert Expression to Python code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_python_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "None".to_string(),
        }
    }
}

impl FileTokens {
    pub fn generate_python_code(&self, depth: usize) -> String {
        let mut output = String::new();
        
        // Generate classes
        for class in &self.classes {
            output.push_str(&class.generate_python_code(depth));
            output.push('\n');
        }
        
        // Generate functions
        for function in &self.functions {
            output.push_str(&function.function_header_generate_python_code(depth));
            output.push_str(&function.function_body_generate_python_code(depth + 1));
            output.push('\n');
        }
        
        // Generate global variables
        for var in &self.variables {
            output.push_str(&var.generate_python_code(depth));
            output.push('\n');
        }
        
        output
    }
}

impl ClassTokens {
    pub fn generate_python_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let mut output = format!("{}class {}:\n", indent, self.name);
        
        // Generate class fields
        for field in &self.fields {
            let field_indent = "    ".repeat(depth + 1);
            match &field.default_value.0 {
                crate::parser::expression::ExpressionPiece::Placeholder(false) => {
                    output.push_str(&format!("{}    self.{} = None\n", field_indent, field.name));
                }
                _ => {
                    output.push_str(&format!("{}    self.{} = {}\n", field_indent, field.name, self.expression_to_python(&field.default_value)));
                }
            }
        }
        
        output
    }
    
    fn expression_to_python(&self, expr: &Expression) -> String {
        // Convert Expression to Python code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_python_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "None".to_string(),
        }
    }
}