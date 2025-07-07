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
    pub fn function_header_generate_javascript_code(&self, depth: usize, is_class_method: bool) -> String {
        let indent = "    ".repeat(depth);
        let param_list = self.params.iter()
            .map(|param| param.generate_javascript_code(depth + 1))
            .collect::<Vec<String>>()
            .join(", ");
        if is_class_method {
            format!("{}{}({}) {{\n", indent, self.name, param_list)
        }  else {
            format!("{}function {}({}) {{\n", indent, self.name, param_list)
        }
    }

    pub fn function_body_generate_javascript_code(&self, depth: usize) -> String {
        self.js_generate_body_from_tokens(&self.body, depth)
    }

    fn js_generate_body_from_tokens(&self, tokens: &[ValidInCodeBlock], depth: usize) -> String {
        let mut output = String::new();
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                ValidInCodeBlock::WhileStartMarker => {
                    // Next should be condition, then body, then JumpIndex
                    if let Some(ValidInCodeBlock::Expression(cond)) = tokens.get(i + 1) {
                        output.push_str(&format!("{}while ({}) {{\n", "    ".repeat(depth), self.expression_to_javascript(cond)));
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
                        output.push_str(&self.js_generate_body_from_tokens(&body_tokens, depth + 1));
                        output.push_str(&format!("{}}}\n", "    ".repeat(depth)));
                        i = j + 1; // Skip past JumpIndex
                        continue;
                    }
                }
                ValidInCodeBlock::IfStartMarker => {
                    // Next should be condition, then body, then JumpIndex
                    if let Some(ValidInCodeBlock::Expression(cond)) = tokens.get(i + 1) {
                        let condition_str = self.expression_to_javascript(cond);
                        // Check if the condition is just a variable named "if" (parser error)
                        let condition = if condition_str == "if" { "true" } else { &condition_str };
                        output.push_str(&format!("{}if ({}) {{\n", "    ".repeat(depth), condition));
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
                        output.push_str(&self.js_generate_body_from_tokens(&body_tokens, depth + 1));
                        output.push_str(&format!("{}}}\n", "    ".repeat(depth)));
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
                    output.push_str(&self.generate_statement_javascript(&tokens[i], depth));
                    i += 1;
                }
            }
        }
        output
    }
    
    fn generate_statement_javascript(&self, statement: &ValidInCodeBlock, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        match statement {
            ValidInCodeBlock::Expression(expr) => {
                format!("{}{};\n", indent, self.expression_to_javascript(expr))
            }
            ValidInCodeBlock::Var(var) => {
                format!("{}{};\n", indent, var.generate_javascript_code(depth))
            }
            ValidInCodeBlock::FunctionCall(func_call) => {
                format!("{}{};\n", indent, func_call.generate_javascript_code(depth))
            }
            ValidInCodeBlock::WhileStartMarker => {
                format!("{}while ", indent)
            }
            ValidInCodeBlock::IfStartMarker => {
                format!("{}if ", indent)
            }
            ValidInCodeBlock::JumpIndex(_) => {
                String::new() // Handle jump logic if needed
            }
            ValidInCodeBlock::HeadEndAndBodyStartMarker => {
                format!("{} {{\n", indent)
            }
        }
    }
    
    fn expression_to_javascript(&self, expr: &Expression) -> String {
        // Convert Expression to JavaScript code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                // Check if this is a control flow function call that should be handled specially
                if func_call.name == "while" || func_call.name == "if" {
                    // For control flow, just return the condition without the function name
                    if func_call.params.len() == 1 {
                        self.expression_to_javascript(&func_call.params[0])
                    } else {
                        func_call.generate_javascript_code(0)
                    }
                } else {
                    func_call.generate_javascript_code(0)
                }
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "null".to_string(),
        }
    }
}

impl ParamTokens {
    pub fn generate_javascript_code(&self, _depth: usize) -> String {
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
    pub fn generate_javascript_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let type_name = self.type_.name;
        let value_str = self.expression_to_javascript(&self.default_value);
        if type_name != "" && type_name != "None" && type_name != "_" {
            format!("{}let {} = {}; // type: {}", indent, self.name, value_str, type_name)
        } else {
            format!("{}let {} = {}", indent, self.name, value_str)
        }
    }
    
    fn expression_to_javascript(&self, expr: &Expression) -> String {
        // Convert Expression to JavaScript code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_javascript_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "null".to_string(),
        }
    }
}

impl FunctionCallTokens {
    pub fn generate_javascript_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let operator_names = ["=", "+", "-", "*", "/", "%", ">", "<", ">=", "<=", "==", "!=", "&&", "||", "^", "|", "&"]; // Add more as needed
        if operator_names.contains(&self.name) && self.params.len() == 2 {
            // Infix notation for binary operators
            let left = self.expression_to_javascript(&self.params[0]);
            let right = self.expression_to_javascript(&self.params[1]);
            format!("{}{} {} {}", indent, left, self.name, right)
        } else {
            let params = self.params.iter()
                .map(|param| self.expression_to_javascript(param))
                .collect::<Vec<String>>()
                .join(", ");
            format!("{}{}({})", indent, self.name, params)
        }
    }
    
    fn expression_to_javascript(&self, expr: &Expression) -> String {
        // Convert Expression to JavaScript code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_javascript_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "null".to_string(),
        }
    }
}

impl FileTokens {
    pub fn generate_javascript_code(&self, depth: usize) -> String {
        let mut output = String::new();
        
        // Generate classes
        for class in &self.classes {
            output.push_str(&class.generate_javascript_code(depth));
            output.push('\n');
        }
        
        // Generate functions
        for function in &self.functions {
            output.push_str(&function.function_header_generate_javascript_code(depth, false));
            output.push_str(&function.function_body_generate_javascript_code(depth + 1));
            output.push_str(&format!("{}}}\n", "    ".repeat(depth)));
            output.push('\n');
        }
        
        // Generate global variables
        for var in &self.variables {
            output.push_str(&var.generate_javascript_code(depth));
            output.push('\n');
        }
        
        output
    }
}

impl ClassTokens {
    pub fn generate_javascript_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let mut output = format!("{}class {} {{\n", indent, self.name);
        
        // Add constructor method
        let constructor_indent = "    ".repeat(depth + 1);
        output.push_str(&format!("{}constructor() {{\n", constructor_indent));
        
        // Generate class fields in constructor
        for field in &self.fields {
            let field_indent = "    ".repeat(depth + 2);
            match &field.default_value.0 {
                crate::parser::expression::ExpressionPiece::Placeholder(false) => {
                    output.push_str(&format!("{}this.{} = null;\n", field_indent, field.name));
                }
                _ => {
                    output.push_str(&format!("{}this.{} = {};\n", field_indent, field.name, self.expression_to_javascript(&field.default_value)));
                }
            }
        }
        
        output.push_str(&format!("{}}}\n", constructor_indent));
        
        // Generate class methods
        for method in &self.methods {
            let method_indent = "    ".repeat(depth + 1);
            output.push_str(&method.function_header_generate_javascript_code(depth + 1, true));
            output.push_str(&method.function_body_generate_javascript_code(depth + 2));
            output.push_str(&format!("{}}}\n", "    ".repeat(depth + 1)));
        }
        
        output.push_str(&format!("{}}}\n", indent));
        
        output
    }
    
    fn expression_to_javascript(&self, expr: &Expression) -> String {
        // Convert Expression to JavaScript code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_javascript_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "null".to_string(),
        }
    }
}