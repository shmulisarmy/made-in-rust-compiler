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
    pub fn function_header_generate_cpp_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let param_list = self.params.iter()
            .map(|param| param.generate_cpp_code(depth + 1))
            .collect::<Vec<String>>()
            .join(", ");
        let return_type = self.cpp_type_name(&self.return_type);
        format!("{}{} {}({}) {{\n", indent, return_type, self.name, param_list)
    }

    pub fn function_body_generate_cpp_code(&self, depth: usize) -> String {
        self.cpp_generate_body_from_tokens(&self.body, depth)
    }

    fn cpp_generate_body_from_tokens(&self, tokens: &[ValidInCodeBlock], depth: usize) -> String {
        let mut output = String::new();
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                ValidInCodeBlock::WhileStartMarker => {
                    // Next should be condition, then body, then JumpIndex
                    if let Some(ValidInCodeBlock::Expression(cond)) = tokens.get(i + 1) {
                        output.push_str(&format!("{}while ({}) {{\n", "    ".repeat(depth), self.expression_to_cpp(cond)));
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
                        output.push_str(&self.cpp_generate_body_from_tokens(&body_tokens, depth + 1));
                        output.push_str(&format!("{}}}\n", "    ".repeat(depth)));
                        i = j + 1; // Skip past JumpIndex
                        continue;
                    }
                }
                ValidInCodeBlock::IfStartMarker => {
                    // Next should be condition, then body, then JumpIndex
                    if let Some(ValidInCodeBlock::Expression(cond)) = tokens.get(i + 1) {
                        let condition_str = self.expression_to_cpp(cond);
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
                        output.push_str(&self.cpp_generate_body_from_tokens(&body_tokens, depth + 1));
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
                    output.push_str(&self.generate_statement_cpp(&tokens[i], depth));
                    i += 1;
                }
            }
        }
        output
    }
    
    fn generate_statement_cpp(&self, statement: &ValidInCodeBlock, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        match statement {
            ValidInCodeBlock::Expression(expr) => {
                format!("{}{};\n", indent, self.expression_to_cpp(expr))
            }
            ValidInCodeBlock::Var(var) => {
                format!("{}{};\n", indent, var.generate_cpp_code(depth))
            }
            ValidInCodeBlock::FunctionCall(func_call) => {
                format!("{}{};\n", indent, func_call.generate_cpp_code(depth))
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
    
    fn expression_to_cpp(&self, expr: &Expression) -> String {
        // Convert Expression to C++ code
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
                        self.expression_to_cpp(&func_call.params[0])
                    } else {
                        func_call.generate_cpp_code(0)
                    }
                } else {
                    func_call.generate_cpp_code(0)
                }
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "nullptr".to_string(),
        }
    }

    fn cpp_type_name(&self, type_: &crate::parser::type_parser::Type_) -> String {
        match type_.name {
            "int" => "int".to_string(),
            "string" => "std::string".to_string(),
            "char" => "char".to_string(),
            "bool" => "bool".to_string(),
            "void" => "void".to_string(),
            "float" => "float".to_string(),
            "double" => "double".to_string(),
            _ => type_.name.to_string(),
        }
    }
}

impl ParamTokens {
    pub fn generate_cpp_code(&self, _depth: usize) -> String {
        let type_name = self.cpp_type_name(&self.type_);
        match &self.default_value.0 {
            crate::parser::expression::ExpressionPiece::Placeholder(false) => {
                format!("{} {}", type_name, self.name)
            }
            _ => {
                // For now, just return the parameter name since we need to implement
                // proper expression conversion for default values
                format!("{} {}", type_name, self.name)
            }
        }
    }

    fn cpp_type_name(&self, type_: &crate::parser::type_parser::Type_) -> String {
        match type_.name {
            "int" => "int".to_string(),
            "string" => "std::string".to_string(),
            "char" => "char".to_string(),
            "bool" => "bool".to_string(),
            "void" => "void".to_string(),
            "float" => "float".to_string(),
            "double" => "double".to_string(),
            _ => type_.name.to_string(),
        }
    }
}

impl VarTokens {
    pub fn generate_cpp_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let type_name = self.cpp_type_name(&self.type_);
        let value_str = self.expression_to_cpp(&self.default_value);
        format!("{}{} {} = {}", indent, type_name, self.name, value_str)
    }
    
    fn expression_to_cpp(&self, expr: &Expression) -> String {
        // Convert Expression to C++ code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_cpp_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "nullptr".to_string(),
        }
    }

    fn cpp_type_name(&self, type_: &crate::parser::type_parser::Type_) -> String {
        match type_.name {
            "int" => "int".to_string(),
            "string" => "std::string".to_string(),
            "char" => "char".to_string(),
            "bool" => "bool".to_string(),
            "void" => "void".to_string(),
            "float" => "float".to_string(),
            "double" => "double".to_string(),
            _ => type_.name.to_string(),
        }
    }
}

impl FunctionCallTokens {
    pub fn generate_cpp_code(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let operator_names = ["=", "+", "-", "*", "/", "%", ">", "<", ">=", "<=", "==", "!=", "&&", "||", "^", "|", "&"]; // Add more as needed
        if operator_names.contains(&self.name) && self.params.len() == 2 {
            // Infix notation for binary operators
            let left = self.expression_to_cpp(&self.params[0]);
            let right = self.expression_to_cpp(&self.params[1]);
            format!("{}{} {} {}", indent, left, self.name, right)
        } else {
            let params = self.params.iter()
                .map(|param| self.expression_to_cpp(param))
                .collect::<Vec<String>>()
                .join(", ");
            format!("{}{}({})", indent, self.name, params)
        }
    }
    
    fn expression_to_cpp(&self, expr: &Expression) -> String {
        // Convert Expression to C++ code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_cpp_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "nullptr".to_string(),
        }
    }
}

impl FileTokens {
    pub fn generate_cpp_header_file(&self) -> String {
        let mut output = String::new();
        
        // Add header guard
        output.push_str("#ifndef GENERATED_CODE_H\n");
        output.push_str("#define GENERATED_CODE_H\n\n");
        
        // Add includes
        output.push_str("#include <iostream>\n");
        output.push_str("#include <string>\n");
        output.push_str("#include <vector>\n");
        output.push_str("#include <map>\n");
        output.push_str("\n");
        
        // Generate class declarations
        for class in &self.classes {
            output.push_str(&class.generate_cpp_header(0));
            output.push('\n');
        }
        
        // Generate function declarations
        for function in &self.functions {
            let param_list = function.params.iter()
                .map(|param| param.generate_cpp_code(0))
                .collect::<Vec<String>>()
                .join(", ");
            let return_type = function.cpp_type_name(&function.return_type);
            output.push_str(&format!("{} {}({});\n", return_type, function.name, param_list));
        }
        
        // Generate global variable declarations
        for var in &self.variables {
            let type_name = var.cpp_type_name(&var.type_);
            output.push_str(&format!("extern {} {};\n", type_name, var.name));
        }
        
        output.push_str("\n#endif // GENERATED_CODE_H\n");
        
        output
    }

    pub fn generate_cpp_code(&self, depth: usize) -> String {
        let mut output = String::new();

        
        
        // Include the header file
        output.push_str(format!("#include \"{}.hpp\"\n\n",self.get_base_file_name()).as_str());
        
        // Generate global variable definitions
        for var in &self.variables {
            output.push_str(&var.generate_cpp_code(depth));
            output.push('\n');
        }
        
        // Generate standalone functions
        for function in &self.functions {
            output.push_str(&function.function_header_generate_cpp_code(depth));
            output.push_str(&function.function_body_generate_cpp_code(depth + 1));
            output.push_str(&format!("{}}}\n", "    ".repeat(depth)));
            output.push('\n');
        }
        
        // Generate class method implementations
        for class in &self.classes {
            output.push_str(&class.generate_cpp_implementations(depth));
        }
        
        output
    }
}

impl ClassTokens {
    pub fn generate_cpp_header(&self, depth: usize) -> String {
        let indent = "    ".repeat(depth);
        let mut output = format!("{}class {} {{\n", indent, self.name);
        
        // Add public section
        let public_indent = "    ".repeat(depth + 1);
        output.push_str(&format!("{}public:\n", public_indent));
        
        // Generate class fields (just declarations, no default values)
        for field in &self.fields {
            let field_indent = "    ".repeat(depth + 2);
            let type_name = self.cpp_type_name(&field.type_);
            output.push_str(&format!("{}    {} {};\n", field_indent, type_name, field.name));
        }
        
        // Add constructor declaration
        let constructor_indent = "    ".repeat(depth + 2);
        output.push_str(&format!("{}    {}();\n", constructor_indent, self.name));
        
        // Add method declarations (just signatures, no bodies)
        for method in &self.methods {
            let method_indent = "    ".repeat(depth + 2);
            let param_list = method.params.iter()
                .map(|param| param.generate_cpp_code(depth + 1))
                .collect::<Vec<String>>()
                .join(", ");
            let return_type = method.cpp_type_name(&method.return_type);
            output.push_str(&format!("{}    {} {}({});\n", method_indent, return_type, method.name, param_list));
        }
        
        output.push_str(&format!("{}}};\n", indent));
        
        output
    }

    pub fn generate_cpp_implementations(&self, depth: usize) -> String {
        let mut output = String::new();
        
        // Add constructor implementation
        let indent = "    ".repeat(depth);
        output.push_str(&format!("{}{}::{}() {{\n", indent, self.name, self.name));
        
        // Initialize fields in constructor
        for field in &self.fields {
            let init_indent = "    ".repeat(depth + 1);
            match &field.default_value.0 {
                crate::parser::expression::ExpressionPiece::Placeholder(false) => {
                    // Don't initialize if no default value
                }
                _ => {
                    output.push_str(&format!("{}    {} = {};\n", init_indent, field.name, self.expression_to_cpp(&field.default_value)));
                }
            }
        }
        
        output.push_str(&format!("{}}}\n", indent));
        
        // Add method implementations in the style ReturnType ClassName::methodName()
        for method in &self.methods {
            let return_type = method.cpp_type_name(&method.return_type);
            let param_list = method.params.iter()
                .map(|param| param.generate_cpp_code(depth + 1))
                .collect::<Vec<String>>()
                .join(", ");
            output.push_str(&format!("{}{} {}::{}({}) {{\n", indent, return_type, self.name, method.name, param_list));
            output.push_str(&method.function_body_generate_cpp_code(depth + 1));
            output.push_str(&format!("{}}}\n", indent));
        }
        
        output.push('\n');
        output
    }
    
    fn expression_to_cpp(&self, expr: &Expression) -> String {
        // Convert Expression to C++ code
        match &expr.0 {
            crate::parser::expression::ExpressionPiece::Variable(name) => name.to_string(),
            crate::parser::expression::ExpressionPiece::StringLiteral(value) => format!("\"{}\"", value),
            crate::parser::expression::ExpressionPiece::NumberLiteral(value) => value.to_string(),
            crate::parser::expression::ExpressionPiece::Operator(op) => op.to_string(),
            crate::parser::expression::ExpressionPiece::FunctionCall(func_call) => {
                func_call.generate_cpp_code(0)
            }
            crate::parser::expression::ExpressionPiece::Placeholder(_) => "nullptr".to_string(),
        }
    }

    fn cpp_type_name(&self, type_: &crate::parser::type_parser::Type_) -> String {
        match type_.name {
            "int" => "int".to_string(),
            "string" => "std::string".to_string(),
            "char" => "char".to_string(),
            "bool" => "bool".to_string(),
            "void" => "void".to_string(),
            "float" => "float".to_string(),
            "double" => "double".to_string(),
            _ => type_.name.to_string(),
        }
    }
}
