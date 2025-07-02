# Reference Validation and Symbol Resolution

This document outlines the conceptual plan for implementing reference validation and symbol resolution within the compiler, focusing on how variables and types are linked to their declarations based on scope.

## Core Principles

1.  **Context Stack for Scoping:** The existing `parser_context` stack (`Vec<SyntaxNode>`) is central to managing scoping information. As the parser enters a new scope (e.g., class, function, while loop, if statement), the corresponding `SyntaxNode` is pushed onto the stack. When the scope is exited, the node is popped.

2.  **Real-time Local Scope Resolution ("Declare Before Use"):**
    *   For variables and types declared within a local scope (e.g., inside a function, a `while` loop, or an `if` block), the "declare before use" rule applies.
    *   When a variable or type *usage* is encountered, the parser will immediately attempt to resolve it by traversing the `parser_context` stack backwards from the current scope.
    *   Only declarations that have *already been processed and are present in the `local_symbols` map of nodes currently on the stack* will be considered valid.

3.  **Deferred Global Scope Resolution ("Declare Later" Allowed):**
    *   For variables and types that are *not* found in any local scope on the `parser_context` stack, the parser will assume they are global declarations that may appear later in the global scope.
    *   No error will be thrown at the parsing stage for such cases. Actual validation for these "assumed global" references will occur in a later compilation phase (e.g., a dedicated symbol resolution pass after the entire global scope is parsed, or during type checking), where all global declarations are known.

## Data Structure Modifications

To support symbol resolution, scope-defining `SyntaxNode`s will be augmented to store their local declarations.

### 1. Augmenting Scope-Defining `SyntaxNode`s (`src/parser/syntax_nodes.rs`)

Each `SyntaxNode` variant that represents a new scope (e.g., `FunctionNode`, `WhileNode`, `IfNode`, `ClassNode`) will conceptually include a `local_symbols` field.

*   **`local_symbols: HashMap<String, DeclarationInfo<'a>>`**: This map will store entries for every variable, parameter, or nested type/function that is declared directly within that specific scope.
    *   **`DeclarationInfo<'a>`**: A conceptual struct holding metadata about a declared item:
        *   `kind`: An enum indicating the type of declaration (e.g., `Variable`, `Function`, `TypeAlias`, `Class`).
        *   `declaration_node`: A reference (`&'a SyntaxNode` or `Rc<RefCell<SyntaxNode>>`) to the actual `SyntaxNode` where the item was declared. This is crucial for linking and will require careful management of Rust's ownership and lifetimes.
        *   `type_info`: The resolved type of the declared item (to be determined in a later phase).

### 2. Populating Symbol Tables During Parsing

As the parser processes declarations, it will populate the `local_symbols` map of the current scope's `SyntaxNode`.

*   **`src/parser/function_parser.rs`**: When parsing a function definition, this module will be responsible for adding function parameters and any variables/types declared directly within the function's body to the `local_symbols` map of the `FunctionNode` currently at the top of the `parser_context` stack.
*   **`src/parser/var_parser.rs`**: When parsing a variable declaration (`let` statement), this module will add the declared variable to the `local_symbols` map of the `SyntaxNode` representing the current scope.
*   **`src/parser/type_parser.rs`**: Similarly, for type aliases or other type declarations, this module would add them to the current scope's `local_symbols`.
*   **`src/parser/while_parser.rs` / `src/parser/if_parser.rs`**: If `while let` or `if let` constructs introduce new variables, these parsers would be responsible for adding those variables to their respective `WhileNode` or `IfNode`'s `local_symbols`.

### 3. Resolving References (Variable/Type Usage)

When the parser encounters a usage of a variable or type (e.g., in an expression, a type annotation):

*   **`src/parser/expression.rs`**: This module (and potentially others where usages occur) will be the primary place where resolution logic is invoked.
*   **Context Walk:** The parser will traverse the `parser_context` stack backwards, from the current scope up to the global scope.
*   **Lookup:** In each `SyntaxNode` on the stack, it will attempt to find the variable/type name in its `local_symbols` map.
*   **First Match Wins:** The first match found (in the nearest enclosing scope) is the correct definition.
*   **Store Reference:** The AST node representing the usage will store a **reference** to the `declaration_node` from the `DeclarationInfo` of the resolved symbol. This is where Rust's lifetimes (`&'a`) must be correctly applied, potentially requiring `Rc` or `Arc` for shared ownership if simple borrowing is insufficient.
*   **Not Found (Local):** If a name is not found in any `local_symbols` map on the stack, and it's not an assumed global, a red text message will be printed.

## Files Likely to be Modified/Interacted With

*   `src/parser/syntax_nodes.rs`: To define `DeclarationInfo` and add `local_symbols` to scope-defining `SyntaxNode` variants.
*   `src/parser/function_parser.rs`: To populate `FunctionNode`'s `local_symbols` with parameters and local declarations.
*   `src/parser/var_parser.rs`: To add variable declarations to the current scope's `local_symbols`.
*   `src/parser/type_parser.rs`: To handle type declarations and add them to `local_symbols`.
*   `src/parser/expression.rs`: Where variable and type *usages* are identified and where the resolution logic would be invoked to update the AST nodes with references.
*   `src/parser/while_parser.rs`: To handle scope-specific declarations within `while` loops.
*   `src/parser/if_parser.rs`: To handle scope-specific declarations within `if` statements.
*   `src/reference_validation/mod.rs`: This module will contain the core logic for symbol table management, reference resolution algorithms, and AST traversal for resolution.
