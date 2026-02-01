use crate::nashira_vm::instruction::TOKEN;
use crate::nashira_vm::value::VALUE;

pub struct Lexer;
impl Lexer {
    pub fn tokenize(code: &str) -> Vec<TOKEN> {
        let mut return_vec: Vec<TOKEN> = Vec::new();
        let string_tokens: Vec<&str> = code.split_whitespace().collect();

        for token_string in string_tokens {
            if let Some(val) = Self::try_to_value(token_string) {
                return_vec.push(TOKEN::VAL(val));
            } else if let Some(val) = Self::try_to_operator(token_string) {
                return_vec.push(val);
            }
        }

        return_vec
    }

    fn try_to_operator(string: &str) -> Option<TOKEN> {
        match string {
            "+" => Some(TOKEN::ADD),
            "-" => Some(TOKEN::SUB),
            "*" => Some(TOKEN::MUL),
            "/" => Some(TOKEN::DIV),
            &_ => None
        }
    }

    fn try_to_value(string: &str) -> Option<VALUE> {
        let result = string.parse::<i32>();
        if result.is_ok() {
            return Some(VALUE::NUM(result.unwrap()))
        }

        None
    }
}