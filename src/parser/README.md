# parser

This directory contains all modules related to parsing the language, including class, function, expression, and control flow (if/while) parsers. It is the core of the compiler's frontend, responsible for turning tokens into an abstract syntax tree (AST) and managing scope/context.

## Key Concepts

### Context Stack
- The parser uses a context stack (`Vec<SyntaxNode>`) to manage scope and enable context-aware parsing (see main README for details).
- Each new scope (class, function, while, if) pushes a node onto the stack; the node is popped after parsing the scope.

### Modular Structure
- **class_parser.rs**: Parses class definitions and their fields.
- **function_parser.rs**: Parses function signatures, parameters, and bodies.
- **expression.rs**: Parses expressions, literals, and operators.
- **code_block.rs**: Shared logic for parsing code blocks and their valid contents.
- **If_parser.rs**: Parses if statements and their bodies.
    using parens around the if's condition is optional (should be used when you plan to use parens or brackets inside of the condition, in that case you'll need it to let the compiler know when your done parsing the condition and are moving onto the action block)
- **while_parser.rs**: Parses while loops and their bodies.
- **type_parser.rs**: Parses type annotations and type expressions.
    valid syntax examples: ```rust
                StorageUnit<Person>
                (int, string)    -    tuple of int's and strings
                []int            -    list of int's
                [][]int          -    matrix of int's
                [string]string   -    string to string map
                Person?          -    optional/nullable Person
            ```

- **var_parser.rs**: Parses variable declarations.
- **mod.rs**: Module glue for re-exporting the submodules.

### Extensibility
- New syntax nodes can be added by creating a new parser module and adding a variant to `SyntaxNode`.
- The context stack pattern makes it easy to add features like type checking, symbol resolution, and more.

## Design Notes
- Each parser is responsible for its own scope management and context stack interaction.
- The code is organized for clarity, modularity, and ease of extension.

--- 