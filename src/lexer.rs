use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::str::Chars;

pub fn tokenizer(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }

    Ok(tokens)
}

#[derive(Debug)]
pub struct TokenError {
    err: String,
}

impl Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Tokenizer error: {}", self.err)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Float(f64),
    Integer(i64),
    Symbol(String),
    LeftParenthesis,
    RightParenthesis,
    String(String),
    BinaryOp(String),
    // UnaryOp(String),
    Keyword(String),
}

pub struct Tokenizer<'a> {
    input: Chars<'a>,
    keywords: HashSet<&'a str>,
    current_character: Option<char>,
    binary_operators: HashSet<char>,
    // unary_operators: HashSet<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_character = chars.next();
        let keywords = ["define", "if"].into_iter().collect();
        let binary_operators = ['+', '-', '*', '/'].into_iter().collect();

        Self {
            input: chars,
            current_character,
            keywords,
            binary_operators,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();

        match self.current_character? {
            '(' => {
                self.advance();
                Some(Token::LeftParenthesis)
            }
            ')' => {
                self.advance();
                Some(Token::RightParenthesis)
            }
            '"' => Some(Token::String(self.read_string())),
            c if c.is_numeric() => {
                let val = self.read_number();
                if val.contains('.') {
                    Some(Token::Float(val.parse::<f64>().unwrap()))
                } else {
                    Some(Token::Integer(val.parse::<i64>().unwrap()))
                }
            }
            c if c.is_alphabetic() || self.binary_operators.contains(&c) => {
                let sym = self.read_symbol();

                if self.keywords.contains(sym.as_str()) {
                    Some(Token::Keyword(sym))
                } else if self.binary_operators.contains(&sym.chars().next().unwrap()) {
                    Some(Token::BinaryOp(sym))
                } else {
                    Some(Token::Symbol(sym))
                }
            }
            _ => None,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current_character = self.input.next();

        self.current_character
    }

    fn eat_whitespace(&mut self) {
        while let Some(c) = self.current_character {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn read_symbol(&mut self) -> String {
        let mut symbol = String::new();
        while let Some(c) = self.current_character {
            if c.is_whitespace() || c == '(' || c == ')' {
                break;
            }

            symbol.push(c);
            self.advance();
        }

        symbol
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(c) = self.current_character {
            if !c.is_numeric() && c != '.' {
                break;
            }

            number.push(c);
            self.advance();
        }

        number
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance();

        while let Some(c) = self.current_character {
            if c == '"' {
                self.advance();
                break;
            }

            string.push(c);
            self.advance();
        }

        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operators() {
        let operators = vec!['+', '-', '*', '/'];

        for operator in operators {
            let expected_tokens = vec![
                Token::LeftParenthesis,
                Token::BinaryOp(operator.to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RightParenthesis,
            ];

            let lisp_program = format!("({} 1 2)", operator);
            let tokens = tokenizer(lisp_program.as_str()).unwrap_or_default();

            assert_eq!(expected_tokens, tokens);
        }
    }

    #[test]
    fn test_area_of_circle() {
        let lisp_program = "(
            (define r 10)
            (define pi 3.14)
            (* pi (* r r))
        )";

        let tokens = tokenizer(lisp_program).unwrap_or_default();

        assert_eq!(
            tokens,
            vec![
                Token::LeftParenthesis,
                Token::LeftParenthesis,
                Token::Keyword(String::from("define")),
                Token::Symbol(String::from("r")),
                Token::Integer(10),
                Token::RightParenthesis,
                Token::LeftParenthesis,
                Token::Keyword(String::from("define")),
                Token::Symbol(String::from("pi")),
                #[allow(clippy::approx_constant)]
                Token::Float(3.14),
                Token::RightParenthesis,
                Token::LeftParenthesis,
                Token::BinaryOp("*".to_string()),
                Token::Symbol(String::from("pi")),
                Token::LeftParenthesis,
                Token::BinaryOp("*".to_string()),
                Token::Symbol(String::from("r")),
                Token::Symbol(String::from("r")),
                Token::RightParenthesis,
                Token::RightParenthesis,
                Token::RightParenthesis,
            ]
        )
    }
}
