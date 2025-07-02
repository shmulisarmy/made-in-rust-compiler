use crate::SyntaxNode;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DeclarationKind {
    Variable,
    Function,
    TypeAlias,
    Class,
    Parameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationInfo<'a> {
    pub kind: DeclarationKind,
    // This reference will point to the SyntaxNode where the item was declared.
    // Using Rc for shared ownership to manage lifetimes within the AST.
    pub declaration_node: Rc<SyntaxNode<'a>>,
}

// This will be the main entry point for reference validation logic.
// For now, it's just a placeholder.
pub fn validate_references<'a>(_ast: &'a SyntaxNode<'a>, _parser_context: &mut Vec<SyntaxNode<'a>>) {
    // TODO: Implement the logic to traverse the AST and resolve references
    // using the parser_context and the local_symbols within each SyntaxNode.
    // This will involve:
    // 1. Walking the AST.
    // 2. For each variable/type usage, look up its definition in the current scope.
    // 3. If found, link the usage to the declaration.
    // 4. If not found in local scope, assume global for now.
    // 5. If not found locally and not a global assumption, print red error.
}

// Helper function to print red text for errors
pub fn print_red_error(message: &str) {
    eprintln!("\x1b[31m{}\x1b[0m", message);
}

// Placeholder for find_definition
pub fn find_definition<'a>(
    context_stack: &Vec<SyntaxNode<'a>>,
    name: &str,
) -> Option<Rc<SyntaxNode<'a>>> {
    // Iterate backwards through the context stack to find the definition
    for node in context_stack.iter().rev() {
        match node {
            SyntaxNode::Function(func_rc) => {
                let func = func_rc.borrow();
                if let Some(decl_info) = func.local_symbols.get(name) {
                    return Some(Rc::clone(&decl_info.declaration_node));
                }
            }
            SyntaxNode::Class(class_rc) => {
                let class = class_rc.borrow();
                if let Some(decl_info) = class.local_symbols.get(name) {
                    return Some(Rc::clone(&decl_info.declaration_node));
                }
            }
            SyntaxNode::While(while_rc) => {
                let while_node = while_rc.borrow();
                if let Some(decl_info) = while_node.local_symbols.get(name) {
                    return Some(Rc::clone(&decl_info.declaration_node));
                }
            }
            SyntaxNode::If(if_rc) => {
                let if_node = if_rc.borrow();
                if let Some(decl_info) = if_node.local_symbols.get(name) {
                    return Some(Rc::clone(&decl_info.declaration_node));
                }
            }
            _ => {
                // Other SyntaxNode variants might not have local_symbols
            }
        }
    }
    None // Definition not found in any active scope
}