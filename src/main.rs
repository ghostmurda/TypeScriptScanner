// SCANNER
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum TokenType {
    // Single-character tokens
    SEMICOLON,
    COLON,
    EQUAL,
    GREATER,
    LESS,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    // Keywords
    LET,
    CONST,
    VAR,
    FUNCTION,
    RETURN,
    IF,
    ELSE,
    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

#[derive(Debug)]
struct Scanner {
    tokens: Vec<Token>,
    cur_line: String,
    line_number: usize,
    word_buffer: String,
}

impl Scanner {
    fn push_line(&mut self, line: &String) {
        self.cur_line = line.clone();

        self.increase_line_number();
        self.scan_line();
    }

    fn increase_line_number(&mut self) {
        self.line_number += 1;
    }

    fn scan_word(&mut self) {
        if !self.word_buffer.is_empty() {
            let word = &self.word_buffer.clone();
            let mut is_aplha_exists = false;
            let mut is_digit_exists = false;

            for char in word.as_str().chars() {
                if char.is_alphabetic() || char == '_' || char == '$' {
                    is_aplha_exists = true;
                }
                if char.is_numeric() {
                    is_digit_exists = true;
                }
            }

            if is_aplha_exists && !is_digit_exists {
                match word.as_str() {
                    "let" => self.add_token(TokenType::LET, String::from(word)),
                    "const" => self.add_token(TokenType::CONST, String::from(word)),
                    "var" => self.add_token(TokenType::VAR, String::from(word)),
                    "function" => self.add_token(TokenType::FUNCTION, String::from(word)),
                    "return" => self.add_token(TokenType::RETURN, String::from(word)),
                    "if" => self.add_token(TokenType::IF, String::from(word)),
                    "else" => self.add_token(TokenType::ELSE, String::from(word)),
                    _ => self.add_token(TokenType::IDENTIFIER, String::from(word)),
                }
            }
            if is_aplha_exists && is_digit_exists {
                self.add_token(TokenType::IDENTIFIER, String::from(word))
            }
            if !is_aplha_exists && is_digit_exists {
                self.add_token(TokenType::NUMBER, String::from(word))
            }

            self.word_buffer.clear();
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        if !matches!(token_type, TokenType::LET)
            && !matches!(token_type, TokenType::CONST)
            && !matches!(token_type, TokenType::VAR)
            && !matches!(token_type, TokenType::IDENTIFIER)
            && !matches!(token_type, TokenType::STRING)
            && !matches!(token_type, TokenType::NUMBER)
            && !matches!(token_type, TokenType::FUNCTION)
            && !matches!(token_type, TokenType::RETURN)
            && !matches!(token_type, TokenType::IF)
            && !matches!(token_type, TokenType::ELSE)
        {
            self.scan_word();
        }

        let token = Token {
            token_type,
            lexeme,
            line: self.line_number,
        };

        self.tokens.push(token);
    }

    fn scan_line(&mut self) {
        let line = &self.cur_line.clone();
        let mut cur_position: usize = 0;

        for (i, char) in line.as_str().chars().enumerate() {
            if cur_position != i {
                continue;
            }

            match char {
                '=' => self.add_token(TokenType::EQUAL, String::from(char)),
                ';' => self.add_token(TokenType::SEMICOLON, String::from(char)),
                ':' => self.add_token(TokenType::COLON, String::from(char)),
                '(' => self.add_token(TokenType::LEFT_PAREN, String::from(char)),
                ')' => self.add_token(TokenType::RIGHT_PAREN, String::from(char)),
                '{' => self.add_token(TokenType::LEFT_BRACE, String::from(char)),
                '}' => self.add_token(TokenType::RIGHT_BRACE, String::from(char)),
                ',' => self.add_token(TokenType::COMMA, String::from(char)),
                '.' => self.add_token(TokenType::DOT, String::from(char)),
                '-' => self.add_token(TokenType::MINUS, String::from(char)),
                '+' => self.add_token(TokenType::PLUS, String::from(char)),
                '/' => self.add_token(TokenType::SLASH, String::from(char)),
                '*' => self.add_token(TokenType::STAR, String::from(char)),
                '>' => self.add_token(TokenType::GREATER, String::from(char)),
                '<' => self.add_token(TokenType::LESS, String::from(char)),
                '"' | '\'' => {
                    let mut string_buffer = String::new();
                    let line = String::from(&line[i + 1..]);

                    for (i, string_char) in line.as_str().chars().enumerate() {
                        if (char == '"' && string_char == '"')
                            || (char == '\'' && string_char == '\'')
                        {
                            cur_position += i + 1;
                            break;
                        }

                        string_buffer.push(string_char);
                    }

                    self.add_token(TokenType::STRING, string_buffer);
                }
                ' ' => self.scan_word(),
                '_' | '$' => self.word_buffer.push(char),
                _ if char.is_alphabetic() => self.word_buffer.push(char),
                _ if char.is_digit(10) => self.word_buffer.push(char),
                _ => (),
            };

            cur_position += 1;
        }
    }
}

fn main() {
    let mut scanner = Scanner {
        tokens: Vec::new(),
        cur_line: String::new(),
        line_number: 0,
        word_buffer: String::new(),
    };

    if let Ok(lines) = read_lines("./test.ts") {
        for line in lines {
            if let Ok(ip) = line {
                scanner.push_line(&ip);
            }
        }
    }

    println!("scanner: {:#?}", scanner);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
