# compiler

# Custom Language Grammar Documentation

This document describes the grammar and syntax of the custom language used in `example.bob`.

## 1. File Structure
- Files can contain function definitions, class definitions, and global variable declarations.

## 2. Functions
```
function <name>(<param_list>) [<return_type>] {
    <body>
}
```
- `function` keyword starts a function definition.
- Parameters are declared as `<type> <name>`, optionally with a default value: `<type> <name> = <expr>`.
- Return type is optional and follows the parameter list.
- The function body is enclosed in `{}`.

**Example:**
```
function add(int a = 9, int b) Person {
    // ...
}
```

## 3. Classes
```
class <name> {
    <field_declarations>
    <method_definitions>
}
```
- `class` keyword starts a class definition.
- Fields are declared as `<type> <name>` or `<type> <name> = <expr>`.
- Methods are defined using the same syntax as functions, inside the class body.

**Example:**
```
class Engine {
    int id
    string repair_station
    function vroom() {
        // ...
    }
}
```

## 4. Variables
- Variables can be declared with `let` (mutable) or `const` (immutable):
```
let <type> <name> = <expr>
const <type> <name> = <expr>
```
- Type and initialization are required.

## 5. Control Flow
### While Loop
```
while (<condition>) {
    <body>
}
```
### If Statement
```
if <condition> {
    <body>
}
```
- No parentheses are required for `if` conditions, but are allowed.

## 6. Types
- Built-in types: `int`, `string`, `char`, etc.
- Nullable types: `Type?` (e.g., `int?`)
- Function types: `function<(<param_types>), <return_type>>`
- Generic types: `Type<OtherType>`

## 7. Expressions
- Standard arithmetic and logical expressions are supported.
- Function calls: `add(1, 2)`

## 8. Comments
- Comments are not explicitly shown in the example, but can be added using `//` for single-line comments.

---

**See `example.bob` for usage examples.** 



0.111 seconds total to lex/parse 5k lines of code