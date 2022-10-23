#[derive(Debug)]
enum TokenType {
    // Punctuation
    Semicolon,
    OpenParen, // (
    CloseParen,
    OpenBracket, // [
    CloseBracket,
    OpenBrace, // {
    CloseBrace,
    Comma,
    Equals,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    PercentEquals,
    Question,
    Colon,
    Ampersand,
    AmpersandAmpersand,
    //And,
    EqualsEquals,
    ExclamationEquals,
    EqualsEqualsEquals,
    ExclamationEqualsEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    //Tilde,
    Exclamation,
    PlusPlus,
    MinusMinus,
    Dot,
    DotDotDot,
    EqualsGreaterThan,
	Or,
	OrOr,

    // Keywords
    Any,
    Bool,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
	Error,
    Declare,
    False,
    Finally,
    For,
    Function,
    Constructor,
    Get,
    If,
    Implements,
    Import,
    In,
    InstanceOf,
    Interface,
    Let,
    Module,
    New,
    Number,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Set,
    Static,
    String,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    TypeOf,
    Var,
    //Void,
    //With,
    While,
    //Yield,

    // Literals
    StringLiteral,
    //RegularExpressionLiteral,
    NumberLiteral,
    Identifier
}

#[derive(Debug)]
enum TokenKind {
    Punctuation,
    Keyword,
    Litetal,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    token_kind: TokenKind,
    value: String,
    line: usize,
	column: usize,
}

#[derive(Debug)]
pub struct Scanner {
    pub tokens: Vec<Token>,
    pub cur_line: Vec<char>,
    pub line_number: usize,
	pub word_buffer: String,
	pub word_buffer_column: usize,
}

impl Scanner {
	pub fn push_line(&mut self, line: &String) {
		let line_vec: Vec<char> = line.as_str().chars().collect();

		self.cur_line = line_vec;
		self.line_number += 1;

		self.scan_line();
	}

	fn match_next(&self, pos: usize, char: char) -> bool {
		if pos + 1 <= self.cur_line.len() - 1 {
			if self.cur_line[pos + 1] == char {
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	fn add_token(&mut self, token_type: TokenType, token_kind: TokenKind, value: String, column: usize) {
		if matches!(token_kind, TokenKind::Punctuation) {
            self.check_word_buffer();
        }

        let token = Token {
            token_type,
            token_kind,
            value,
			column: column + 1,
            line: self.line_number,
        };

        self.tokens.push(token);
    }

	fn check_word_buffer(&mut self) {
        if !self.word_buffer.is_empty() {
            let word = &self.word_buffer.clone();
            let mut is_aplha_exists = false;
            let mut is_digit_exists = false;

            for ch in word.as_str().chars() {
                if ch.is_alphabetic() || ch == '_' || ch == '$' {
                    is_aplha_exists = true;
                }
                if ch.is_numeric() {
                    is_digit_exists = true;
                }
            }

            if is_aplha_exists && !is_digit_exists {
                match word.as_str() {
                    "any" => self.add_token(TokenType::Any, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "boolean" => self.add_token(TokenType::Bool, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "break" => self.add_token(TokenType::Break, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "case" => self.add_token(TokenType::Case, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "catch" => self.add_token(TokenType::Catch, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "class" => self.add_token(TokenType::Class, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "const" => self.add_token(TokenType::Const, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "continue" => self.add_token(TokenType::Continue, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "debugger" => self.add_token(TokenType::Debugger, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "default" => self.add_token(TokenType::Default, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "delete" => self.add_token(TokenType::Delete, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "do" => self.add_token(TokenType::Do, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "else" => self.add_token(TokenType::Else, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "enum" => self.add_token(TokenType::Enum, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "export" => self.add_token(TokenType::Export, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "extends" => self.add_token(TokenType::Extends, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "error" => self.add_token(TokenType::Error, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "declare" => self.add_token(TokenType::Declare, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "false" => self.add_token(TokenType::False, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "finally" => self.add_token(TokenType::Finally, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "for" => self.add_token(TokenType::For, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "function" => self.add_token(TokenType::Function, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "constructor" => self.add_token(TokenType::Constructor, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "get" => self.add_token(TokenType::Get, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "if" => self.add_token(TokenType::If, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "implements" => self.add_token(TokenType::Implements, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "import" => self.add_token(TokenType::Import, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "in" => self.add_token(TokenType::In, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "instanceof" => self.add_token(TokenType::InstanceOf, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "interface" => self.add_token(TokenType::Interface, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "let" => self.add_token(TokenType::Let, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "module" => self.add_token(TokenType::Module, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "new" => self.add_token(TokenType::New, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "number" => self.add_token(TokenType::Number, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "null" => self.add_token(TokenType::Null, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "package" => self.add_token(TokenType::Package, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "private" => self.add_token(TokenType::Private, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "protected" => self.add_token(TokenType::Protected, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "public" => self.add_token(TokenType::Public, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "return" => self.add_token(TokenType::Return, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "set" => self.add_token(TokenType::Set, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "static" => self.add_token(TokenType::Static, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "string" => self.add_token(TokenType::String, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "super" => self.add_token(TokenType::Super, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "switch" => self.add_token(TokenType::Switch, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "this" => self.add_token(TokenType::This, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "throw" => self.add_token(TokenType::Throw, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "true" => self.add_token(TokenType::True, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "try" => self.add_token(TokenType::Try, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "typeof" => self.add_token(TokenType::TypeOf, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "var" => self.add_token(TokenType::Var, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    "while" => self.add_token(TokenType::While, TokenKind::Keyword, String::from(word), self.word_buffer_column),
                    _ => self.add_token(TokenType::Identifier, TokenKind::Litetal, String::from(word), self.word_buffer_column),
                }
            }
            if is_aplha_exists && is_digit_exists {
                self.add_token(TokenType::Identifier, TokenKind::Litetal, String::from(word), self.word_buffer_column);
            }
            if !is_aplha_exists && is_digit_exists {
                self.add_token(TokenType::NumberLiteral, TokenKind::Litetal, String::from(word), self.word_buffer_column);
            }

            self.word_buffer.clear();
			self.word_buffer_column = 1;
        }
    }

	fn scan_line(&mut self) {
		let line = &self.cur_line.clone();
		let mut cur_pos: usize = 0;

		for (pos, ch) in line.iter().enumerate() {
			if pos != cur_pos {
				continue;
			}

			match ch {
				';' => self.add_token(TokenType::Semicolon, TokenKind::Punctuation, String::from(";"), pos),
                '(' => self.add_token(TokenType::OpenParen, TokenKind::Punctuation, String::from("("), pos),
                ')' => self.add_token(TokenType::CloseParen, TokenKind::Punctuation, String::from(")"), pos),
                '[' => self.add_token(TokenType::OpenBracket, TokenKind::Punctuation, String::from("["), pos),
                ']' => self.add_token(TokenType::CloseBracket, TokenKind::Punctuation, String::from("]"), pos),
                '{' => self.add_token(TokenType::OpenBrace, TokenKind::Punctuation, String::from("{"), pos),
                '}' => self.add_token(TokenType::CloseBrace, TokenKind::Punctuation, String::from("}"), pos),
                ',' => self.add_token(TokenType::Comma, TokenKind::Punctuation, String::from(","), pos),
				'?' => self.add_token(TokenType::Question, TokenKind::Punctuation, String::from("?"), pos),
                ':' => self.add_token(TokenType::Colon, TokenKind::Punctuation, String::from(":"), pos),
				'=' => {
					if self.match_next(pos, '=') {
						if self.match_next(pos + 1, '=') {
							self.add_token(TokenType::EqualsEqualsEquals, TokenKind::Punctuation, String::from("==="), pos);
							cur_pos += 2;
						} else {
							self.add_token(TokenType::EqualsEquals, TokenKind::Punctuation, String::from("=="), pos);
							cur_pos += 1;
						}
					} else if self.match_next(pos, '>') {
						self.add_token(TokenType::EqualsGreaterThan, TokenKind::Punctuation, String::from("=>"), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Equals, TokenKind::Punctuation, String::from("="), pos);
					}
				}
				'&' => {
					if self.match_next(pos, '&') {
						self.add_token(TokenType::AmpersandAmpersand, TokenKind::Punctuation, String::from("&&"), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Ampersand, TokenKind::Punctuation, String::from("&"), pos);
					}
				}
				'<' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::LessThanEquals, TokenKind::Punctuation, String::from("<="), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::LessThan, TokenKind::Punctuation, String::from("<"), pos);
					}
				}
				'>' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::GreaterThanEquals, TokenKind::Punctuation, String::from(">="), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::GreaterThan, TokenKind::Punctuation, String::from(">"), pos);
					}
				}
				'+' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::PlusEquals, TokenKind::Punctuation, String::from("+="), pos);
						cur_pos += 1;
					} else if self.match_next(pos, '+') {
						self.add_token(TokenType::PlusPlus, TokenKind::Punctuation, String::from("++"), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Plus, TokenKind::Punctuation, String::from("+"), pos);
					}
				}
				'-' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::MinusEquals, TokenKind::Punctuation, String::from("-="), pos);
						cur_pos += 1;
					} else if self.match_next(pos, '-') {
						self.add_token(TokenType::MinusMinus, TokenKind::Punctuation, String::from("--"), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Minus, TokenKind::Punctuation, String::from("-"), pos);
					}
				}
				'*' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::StarEquals, TokenKind::Punctuation, String::from("*="), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Star, TokenKind::Punctuation, String::from("*"), pos);
					}
				}
				'/' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::SlashEquals, TokenKind::Punctuation, String::from("/="), pos);
						cur_pos += 1;
					} else if self.match_next(pos, '/') {
						break;
					} else {
						self.add_token(TokenType::Slash, TokenKind::Punctuation, String::from("/"), pos);
					}
				}
				'%' => {
					if self.match_next(pos, '=') {
						self.add_token(TokenType::PercentEquals, TokenKind::Punctuation, String::from("%="), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Percent, TokenKind::Punctuation, String::from("%"), pos);
					}
				}
				'!' => {
					if self.match_next(pos, '=') {
						if self.match_next(pos + 1, '=') {
							self.add_token(TokenType::ExclamationEqualsEquals, TokenKind::Punctuation, String::from("!=="), pos);
							cur_pos += 2;
						} else {
							self.add_token(TokenType::ExclamationEquals, TokenKind::Punctuation, String::from("!="), pos);
							cur_pos += 1;
						}
					} else {
						self.add_token(TokenType::Exclamation, TokenKind::Punctuation, String::from("!"), pos);
					}
				}
				'.' => {
					if self.match_next(pos, '.') && self.match_next(pos + 1, '.') {
						self.add_token(TokenType::DotDotDot, TokenKind::Punctuation, String::from("..."), pos);
						cur_pos += 2;
					} else {
						self.add_token(TokenType::Dot, TokenKind::Punctuation, String::from("."), pos);
					}
				}
				'|' => {
					if self.match_next(pos, '|') {
						self.add_token(TokenType::OrOr, TokenKind::Punctuation, String::from("||"), pos);
						cur_pos += 1;
					} else {
						self.add_token(TokenType::Or, TokenKind::Punctuation, String::from("|"), pos);
					}
				}
				'"' | '\'' => {
                    let mut string_buffer = String::new();
					let mut string_pos = pos + 1;

					for string_ch in line[pos + 1..].iter() {
						if (ch == &'"' && string_ch == &'"') || (ch == &'\'' && string_ch == &'\'') {
							cur_pos = string_pos;
                    		break;
						}

						string_buffer.push(*string_ch);
						string_pos += 1;
					}

					self.add_token(TokenType::StringLiteral, TokenKind::Litetal, string_buffer, pos);
                }
				' ' | '\r' | '\t' => self.check_word_buffer(),
                _ if ch.is_digit(10) || ch.is_alphabetic() || ch == &'_' || ch == &'$' => {
					if self.word_buffer.is_empty() {
						self.word_buffer_column = pos;
					}

					self.word_buffer.push(*ch)
				}
				_ => ()
			}

			cur_pos += 1;
		}

		self.check_word_buffer();
	}
}