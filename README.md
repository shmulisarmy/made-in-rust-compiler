# made-in-rust-compiler

## Parser Context Management and Scoping



### Difference between type-checking in the local and global scope
- [] a var, function or type that you use that your getting from the within the local scope everything is in order of time (you cant use something before you declare it)
- [] you can use a global type, var or function and then declare later in the global scope
therefore if you see you a type, var or function that isn't declared yet (which gets checked by walking up the function_body stack) once it cant find it there it will look in the global scope


0.111 seconds total to lex/parse 5k lines of code