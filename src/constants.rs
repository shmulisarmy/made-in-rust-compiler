use crate::libs::mapTrie::MapTrie;
use crate::libs::trie::Trie;
use std::sync::LazyLock;
use std::sync::Mutex;

pub static SPACE_CHARS: LazyLock<Vec<char>> = LazyLock::new(|| vec![' ', '\t', '\n', '\r']);

pub static OPERATORS: LazyLock<Vec<char>> = LazyLock::new(|| {
    vec![
        '+', '-', '*', '/', '%', '=', '!', '&', '|', '<', '>', '^', '~',
    ]
});

pub static KEYWORDS_TRIE: LazyLock<Trie> = LazyLock::new(|| {
    let mut trie = Trie::new();
    let keywords = vec![
        "fn",
        "let",
        "mut",
        "if",
        "else",
        "while",
        "for",
        "in",
        "return",
        "break",
        "continue",
        "struct",
        "enum",
        "impl",
        "use",
        "mod",
        "pub",
        "const",
        "static",
        "match",
        "as",
        "trait",
        "type",
        "where",
        "self",
        "super",
        "crate",
        "extern",
        "ref",
        "move",
        "unsafe",
        "dyn",
        "async",
        "await",
        "box",
        "macro_rules",
        "class",
        "function",
    ];

    for keyword in keywords {
        trie.insert(&keyword.to_string());
    }
    trie
});

pub static OPERATORS_TRIE: LazyLock<MapTrie> = LazyLock::new(|| {
    let mut trie = MapTrie::new();
    let operators = vec![
        "+", "-", "*", "/", "%", "=", "!", "&", "|", "<", ">", "^", "~", "?", ".", "==", "!=",
        "<=", ">=", "&&", "||", "++", "--", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<",
        ">>", "<<=", ">>=", "->", "=>", "::", "..", "...",
    ];

    for operator in operators {
        trie.insert(&operator.to_string());
    }
    trie
});

pub static PUNCTUATION_TRIE: LazyLock<MapTrie> = LazyLock::new(|| {
    let mut trie = MapTrie::new();
    let punctuation = vec![",", ";", ":", "@", "#", "$", "`", "!"];

    for punct in punctuation {
        trie.insert(&punct.to_string());
    }
    trie
});
