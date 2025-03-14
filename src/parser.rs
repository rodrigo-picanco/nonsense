use crate::{
    lexer::{Lexer, Token, DOT, EOF, LBRACK},
    program::Program,
};

pub trait Statement {
    fn eval(&self) -> String;
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}
impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }
    pub fn run(&mut self) -> Program {
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
        let ident = self.parse_identifier();

        if self.peek_token.kind == LBRACK {
             return DotStatement {
                ident,
                block: Some(self.parse_block_statement()),
            };
        }

        if self.peek_token.kind == DOT

        panic!()
    }
    fn parse_identifier(&mut self) -> IdentifierStatement {
        IdentifierStatement {
            literal: self.current_token.literal.clone(),
        }
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
impl Clone for IdentifierStatement {
    fn clone(&self) -> IdentifierStatement {
        IdentifierStatement {
            literal: self.literal.clone(),
        }
    }
}

struct DotStatement {
    ident: IdentifierStatement,
    block: Option<BlockStatement>,
}
impl Statement for DotStatement {
    fn eval(&self) -> String {
        let columns = self.block.clone().unwrap().eval();
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
impl Clone for BlockStatement {
    fn clone(&self) -> BlockStatement {
        BlockStatement {
            properties: self.properties.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod test_parser {
        use super::*;
        macro_rules! test_parser {
            ($input: expr, $exp: expr) => {
                let mut parser = Parser::new($input);
                let result = parser.run();
                $exp.statements
                    .iter()
                    .zip(result.statements.iter())
                    .for_each(|(expected, result)| {
                        assert_eq!(expected.eval(), result.eval());
                    });
            };
        }
        #[test]
        fn class_selector() {
            test_parser!(
                ".users {}",
                Program {
                    statements: vec![Box::new(DotStatement {
                        ident: IdentifierStatement {
                            literal: "users".to_string(),
                        },
                        block: Some(BlockStatement { properties: vec![] }),
                    })],
                }
            );
        }

        #[test]
        fn block_statement() {
            test_parser!(
                ".users { name, id }",
                Program {
                    statements: vec![Box::new(DotStatement {
                        ident: IdentifierStatement {
                            literal: "users".to_string(),
                        },
                        block: Some(BlockStatement {
                            properties: vec![
                                IdentifierStatement {
                                    literal: "name".to_string(),
                                },
                                IdentifierStatement {
                                    literal: "id".to_string(),
                                },
                            ],
                        }),
                    })],
                }
            );
        }

        #[test]
        fn joint_class_selector() {
            test_parser!(
                ".users .posts {}",
                Program {
                    statements: vec![Box::new(DotStatement {
                        ident: IdentifierStatement {
                            literal: "users".to_string(),
                        },
                        block: Some(BlockStatement { properties: vec![] }),
                    })],
                }
            );
        }
    }
}
