# parser

This directory contains all modules related to parsing the language, including class, function, expression, and control flow (if/while) parsers. It is the core of the compiler's frontend, responsible for turning tokens into an abstract syntax tree (AST) and managing scope/context.

## Key Concepts

### Context Stack
- The parser uses a context stack (`Vec<SyntaxNode>`) to manage scope and enable context-aware parsing (see main README for details).
- Each new scope (class, function, while, if) pushes a node onto the stack; the node is popped after parsing the scope.

###valid type syntax examples
```rust
        (int, string)    -    //tuple of int's and strings
        []int            -    //list of int's
        []*int           -    //list of int pointer's
        [][]int          -    //matrix of int's
        [string]string   -    //string to string map
        Person?          -    //optional/nullable Person
        StorageUnit<Person>
```
