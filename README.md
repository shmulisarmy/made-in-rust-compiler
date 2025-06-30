# made-in-rust-compiler

## Parser Context Management and Scoping

### Overview
This project uses a **context stack** (a `Vec<SyntaxNode>`) to manage scoping information during parsing. As the parser encounters new scopes (such as classes, functions, while loops, and if statements), it pushes a corresponding `SyntaxNode` onto the context stack. When the parser finishes parsing the scope, it pops the node off the stack. This enables future features like variable/type lookup with proper scoping and shadowing.


### Difference between type-checking in the local and global scope
- [] a var, function or type that you use that your getting from the within the local scope everything is in order of time (you cant use something before you declare it)
- [] you can use a global type, var or function and then declare later in the global scope
therefore if you see you a type, var or function that isn't declared yet (which gets checked by walking up the parser_context stack) the assumption is that your gonna declare it later the global scope and we can therefore continue parsing and not throw an error, and later when we parse more items in the global scope wel'e have enough info to make sure everything is properly checked

### How It Works
- **During Parsing:**
  - When a new scope (class, function, while, if) is encountered, a `SyntaxNode` is pushed onto the context stack.
  - The body of the scope is parsed while the node is on the stack.
  - After parsing the body, the node is popped off.
- **In Main Parsing (main.rs):**
  - The context stack is wrapped in a `Mutex` for thread safety, but this is only necessary for the main parsing entry point.
- **In Tests:**
  - Tests use a plain `Vec<SyntaxNode>` for the context stack, with no Mutex required. This keeps tests simple and fast.

### Future: Context-Walking for Lookup
The context stack enables future features such as:
- **Variable/Type Lookup:**
  - When resolving a variable or type, the parser can iterate backwards through the context stack, checking each scope for the definition.
  - The nearest enclosing scope wins, supporting shadowing and proper lexical scoping.
- **Extensibility:**
  - This pattern makes it easy to add new scope-aware features, such as type checking, symbol resolution, and more.


