use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = std::fs::read_to_string(filename).expect("EXEC ERROR: Failed to read file");
    let program = Program::new(&input);
    println!("{}", program.run());
}

type TokenKind = &'static str;
const DOT: TokenKind = "DOT";
const RBRACK: TokenKind = "RBRACK";
const LBRACK: TokenKind = "LBRACK";
const IDENT: TokenKind = "IDENT";
const EOF: TokenKind = "EOF";

trait Statement {
    fn eval(&self) -> String;
}

struct Program {
    statements: Vec<Box<dyn Statement>>,
}
impl Program {
    fn new(input: &str) -> Self {
        Parser::new(input).run()
    }
    fn run(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.eval())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

struct IdentifierStatement {
    literal: String,
}
impl Statement for IdentifierStatement {
    fn eval(&self) -> String {
        self.literal.clone()
    }
}

struct DotStatement {
    ident: IdentifierStatement,
    block: BlockStatement,
}
impl Statement for DotStatement {
    fn eval(&self) -> String {
        let columns = self.block.eval();
        if columns.is_empty() {
            return format!("SELECT * FROM {};", self.ident.literal);
        }
        let table = &self.ident.literal;
        format!("SELECT {} FROM {};", columns, table)
    }
}

struct BlockStatement {
    properties: Vec<IdentifierStatement>,
}
impl Statement for BlockStatement {
    fn eval(&self) -> String {
        self.properties
            .iter()
            .map(|statement| statement.eval())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}
impl Parser {
    fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }
    fn run(&mut self) -> Program {
        let mut program = Program { statements: vec![] };
        loop {
            if self.current_token.kind == EOF {
                break;
            }
            let statement = self.parse_statement();
            program.statements.push(statement);
            self.next_token();
        }
        program
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    fn parse_statement(&mut self) -> Box<dyn Statement> {
        match self.current_token.kind {
            "DOT" => Box::new(self.parse_dot()),
            "LBRACK" => Box::new(self.parse_block_statement()),
            _ => panic!("PARSE ERROR: unknown token {}", self.current_token.kind),
        }
    }
    fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();
        self.next_token();
        let mut properties = vec![];
        loop {
            if self.current_token.kind == "RBRACK" {
                break;
            }
            properties.push(self.parse_identifier());
            self.next_token();
        }
        BlockStatement { properties }
    }
    fn parse_dot(&mut self) -> DotStatement {
        self.next_token();
        DotStatement {
            ident: self.parse_identifier(),
            block: self.parse_block_statement(),
        }
    }
    fn parse_identifier(&mut self) -> IdentifierStatement {
        IdentifierStatement {
            literal: self.current_token.literal.clone(),
        }
    }
}

struct Token {
    kind: TokenKind,
    literal: String,
}
impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            literal: self.literal.clone(),
        }
    }
}
struct Lexer {
    input: String,
    position: u64,
    read_position: u64,
    character: char,
}
impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            read_position: 1,
            character: input.chars().nth(0).unwrap(),
        }
    }
    fn next_token(&mut self) -> Token {
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
                $expected.iter().zip(tokens.iter()).for_each(|(expected, result)| {
                    assert_eq!(expected.kind, result.kind);
                    assert_eq!(expected.literal, result.literal);
                });
            };
        }

        #[test]
        fn dot_statement() {
            test_lexer!(".users {}", vec![
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
            ]);
        }

        #[test]
        fn block_statement() {
            test_lexer!(".users { name, id }", vec![
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
            ]);
        }

        #[test]
        fn joint_dot_statement() {
            test_lexer!(".users .posts {}", vec![
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
            ]);
        }
    }

    #[test]
    fn test_parse() {
        let input = ".users {
            name,
            id
        }";
        let mut parser = Parser::new(input);
        let expected_tree = Program {
            statements: vec![Box::new(DotStatement {
                ident: IdentifierStatement {
                    literal: "users".to_string(),
                },
                block: BlockStatement {
                    properties: vec![
                        IdentifierStatement {
                            literal: "name".to_string(),
                        },
                        IdentifierStatement {
                            literal: "id".to_string(),
                        },
                    ],
                },
            })],
        };
        let result = parser.run();
        expected_tree
            .statements
            .iter()
            .zip(result.statements.iter())
            .for_each(|(expected, result)| {
                assert_eq!(expected.eval(), result.eval());
            });
    }

    #[test]
    fn test_run() {
        assert_eq!(Program::new(".users {}").run(), "SELECT * FROM users;");
        assert_eq!(
            Program::new(
                ".users {
            name,
            id
        }"
            )
            .run(),
            "SELECT name, id FROM users;"
        );
    }
}
