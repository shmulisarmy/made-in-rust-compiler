# project_basic_utils

This directory contains foundational utilities used throughout the compiler's parsing pipeline. These modules provide basic building blocks for tokenization, token types, and other low-level helpers that are reused by higher-level parser components.

## Usage
- on the tokenizer, methods like expect are faster because they know what their looking for where as methods like .next are slightly more magical in the sense that the parser will just send it out and it will give back the next useable token with info together with it
- when you know your looking for a specific char things can be really fast, weather that char is optional or not
- expect_char_with_backups will take in the main char that your looking for and when you see it you'l eat it up (move forward in the tokenizer), if doesn't find that char it will throw an "expected , but but got int" error for the example: 

    ```rust
        function add(int a int b)
    ```
where the user forgot a ','.
for the backups in expect_char_with_backups that would be something like [')', '='] where if we see that its valid but we wont consume that char because a different part of the parser is gonna use that as information (')': the function_header will know that its done parsing, '=', the part right after will know that the user is adding a default value for that param)

