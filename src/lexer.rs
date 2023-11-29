use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum TokenType {
    Let,
    Print,
    Fn,
    Identifier,
    Number,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Plus,
    Equal,
    Minus,
    Star,
    By,
    Percent,
}

pub struct Token {
    m_value: Vec<u8>,
    m_type: TokenType,
}

fn print_token(tok: &Token) {
    let value_str = String::from_utf8_lossy(&tok.m_value);
    println!("Type: {:?}, Value: {}", tok.m_type, value_str);
}

pub fn tokenize(m_code: &str) -> Vec<Token> {
    let regex_patterns: Vec<(TokenType, Regex)> = vec![
        (TokenType::Let, Regex::new(r"let").unwrap()),
        (TokenType::Print, Regex::new(r"print").unwrap()),
        (TokenType::Fn, Regex::new(r"fn").unwrap()),
        (TokenType::Identifier, Regex::new(r"[a-zA-Z_]\w*").unwrap()),
        (TokenType::Number, Regex::new(r"\d+").unwrap()),
        (TokenType::OpenParen, Regex::new(r"\(").unwrap()),
        (TokenType::CloseParen, Regex::new(r"\)").unwrap()),
        (TokenType::OpenBrace, Regex::new(r"\{").unwrap()),
        (TokenType::CloseBrace, Regex::new(r"\}").unwrap()),
        (TokenType::Comma, Regex::new(r",").unwrap()),
        (TokenType::Plus, Regex::new(r"\+").unwrap()),
        (TokenType::Minus, Regex::new(r"\-").unwrap()),
        (TokenType::Star, Regex::new(r"\*").unwrap()),
        (TokenType::By, Regex::new(r"\/").unwrap()),
        (TokenType::Percent, Regex::new(r"\%").unwrap()),
        (TokenType::Equal, Regex::new(r"=").unwrap()),
    ];

    let mut tokens: Vec<Token> = Vec::new();
    let mut current_word = String::new();

    for ch in m_code.chars() {
        if ch.is_whitespace() || ch == ';' || ch == '(' || ch == ')' || ch == '+' || ch == '{' || ch == '}' || ch == ',' || ch == '=' {
            // Process the current word if it's not empty
            if !current_word.is_empty() {
                let mut token_type = TokenType::Identifier;

                for (t, regex) in &regex_patterns {
                    if regex.is_match(&current_word) {
                        token_type = *t;
                        break;
                    }
                }

                let token = Token {
                    m_value: current_word.as_bytes().to_vec(),
                    m_type: token_type,
                };

                print_token(&token);
                tokens.push(token);

                // Reset the current word
                current_word.clear();
            }

            // Process the special character as a separate token
            if ch == ';' || ch == '(' || ch == ')' || ch == '+' || ch == '{' || ch == '}' || ch == ',' || ch == '=' {
                let special_token_type = match ch {
                    ';' => TokenType::Identifier, // Placeholder for semicolon handling
                    '(' => TokenType::OpenParen,
                    ')' => TokenType::CloseParen,
                    '+' => TokenType::Plus,
                    '{' => TokenType::OpenBrace,
                    '}' => TokenType::CloseBrace,
                    ',' => TokenType::Comma,
                    '=' => TokenType::Equal,
                    _ => TokenType::Identifier,
                };

                let special_token = Token {
                    m_value: vec![ch as u8],
                    m_type: special_token_type,
                };

                print_token(&special_token);
                tokens.push(special_token);
            }
        } else {
            current_word.push(ch);
        }
    }

    // Process the last word (if any)
    if !current_word.is_empty() {
        let mut token_type = TokenType::Identifier;
        for (t, regex) in &regex_patterns {
            if regex.is_match(&current_word) {
                token_type = *t;
                break;
            }
        }

        let token = Token {
            m_value: current_word.as_bytes().to_vec(),
            m_type: token_type,
        };

        print_token(&token);
        tokens.push(token);
    }

    tokens
}
