use crate::nashira_vm::value::VALUE;

//#[derive(Debug, Clone, Copy)]
pub enum TOKEN {
    VAL(VALUE),
    LPAREN, RPAREN,
    LBRACE, RBRACE,
    COMMA, SEMICOLON,

    ADD, SUB, MUL, DIV,

    FUNC,
    IF,
    ELSE,
    WHILE,
    RETURN,

    EOF,
}