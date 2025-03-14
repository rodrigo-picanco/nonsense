type TokenKind = &'static str;

pub const DOT: TokenKind = "DOT";
const RBRACK: TokenKind = "RBRACK";
pub const LBRACK: TokenKind = "LBRACK";
const IDENT: TokenKind = "IDENT";
pub const EOF: TokenKind = "EOF";

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}
impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            literal: self.literal.clone(),
        }
    }
}

pub struct Lexer {
    input: String,
    position: u64,
    read_position: u64,
    character: char,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            read_position: 1,
            character: input.chars().nth(0).unwrap(),
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.character {
            '.' => Token {
                kind: DOT,
                literal: ".".to_string(),
            },
            '{' => Token {
                kind: LBRACK,
                literal: "{".to_string(),
            },
            '}' => Token {
                kind: RBRACK,
                literal: "}".to_string(),
            },
            '0' => Token {
                kind: EOF,
                literal: "".to_string(),
            },
            _ => {
                if self.character.is_alphabetic() {
                    Token {
                        kind: IDENT,
                        literal: self.read_identifier(),
                    }
                } else {
                    panic!("LEX ERROR: Unknown token {}", self.character);
                }
            }
        };
        self.read_char();
        token
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.character.is_alphabetic() {
            self.read_char();
        }
        self.input
            .chars()
            .skip(position as usize)
            .take((self.position - position) as usize)
            .collect::<String>()
            .clone()
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u64 {
            self.character = '0';
        } else {
            self.character = self.input.chars().nth(self.read_position as usize).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self.character.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod test_lexer {
        use super::*;

        macro_rules! test_lexer {
            ($input:expr, $expected:expr) => {
                let mut lexer = Lexer::new($input);
                let mut tokens = vec![];
                loop {
                    let token = lexer.next_token();
                    tokens.push(token.clone());
                    if token.kind == EOF {
                        break;
                    }
                }
                $expected
                    .iter()
                    .zip(tokens.iter())
                    .for_each(|(expected, result)| {
                        assert_eq!(expected.kind, result.kind);
                        assert_eq!(expected.literal, result.literal);
                    });
            };
        }

        #[test]
        fn class_selector() {
            test_lexer!(
                ".users {}",
                vec![
                    Token {
                        kind: DOT,
                        literal: ".".to_string(),
                    },
                    Token {
                        kind: IDENT,
                        literal: "users".to_string(),
                    },
                    Token {
                        kind: LBRACK,
                        literal: "{".to_string(),
                    },
                    Token {
                        kind: RBRACK,
                        literal: "}".to_string(),
                    },
                ]
            );
        }

        #[test]
        fn block_statement() {
            test_lexer!(
                ".users { name, id }",
                vec![
                    Token {
                        kind: DOT,
                        literal: ".".to_string(),
                    },
                    Token {
                        kind: IDENT,
                        literal: "users".to_string(),
                    },
                    Token {
                        kind: LBRACK,
                        literal: "{".to_string(),
                    },
                    Token {
                        kind: IDENT,
                        literal: "name".to_string(),
                    },
                    Token {
                        kind: IDENT,
                        literal: "id".to_string(),
                    },
                    Token {
                        kind: RBRACK,
                        literal: "}".to_string(),
                    },
                ]
            );
        }

        #[test]
        fn joint_class_selector() {
            test_lexer!(
                ".users .posts {}",
                vec![
                    Token {
                        kind: DOT,
                        literal: ".".to_string(),
                    },
                    Token {
                        kind: IDENT,
                        literal: "users".to_string(),
                    },
                    Token {
                        kind: DOT,
                        literal: ".".to_string(),
                    },
                    Token {
                        kind: IDENT,
                        literal: "posts".to_string(),
                    },
                    Token {
                        kind: LBRACK,
                        literal: "{".to_string(),
                    },
                    Token {
                        kind: RBRACK,
                        literal: "}".to_string(),
                    },
                ]
            );
        }
    }
}
