use crate::{
    ast::{Declaration, Rule, Ruleset, Selector},
    lexer::{Lexer, Token, EOF, IDENT},
};

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
    pub fn run(&mut self) -> Ruleset {
        let mut rules = vec![];
        loop {
            if self.current_token.kind == EOF {
                break;
            }

            rules.push(self.parse_rule());
        }
        rules
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selector: self.parse_selector(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selector(&mut self) -> Selector {
        let mut selector = Selector { literals: vec![] };
        while self.current_token.kind == IDENT {
            selector.literals.push(self.current_token.literal.clone());
            self.next_token();
        }
        selector
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = vec![];
        self.next_token();
        while self.current_token.kind == IDENT {
            declarations.push(Declaration {
                property: self.current_token.literal.clone(),
            });
            self.next_token(); 
        }
        self.next_token(); // SKIP BRACKET
        declarations
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
                assert_eq!(Parser::new($input).run(), $exp);
            };
        }

        #[test]
        fn class_selector() {
            test_parser!(
                "users {}",
                vec![Rule {
                    selector: Selector {
                        literals: ["users".to_string()].to_vec()
                    },
                    declarations: [].to_vec()
                }]
            );
        }

        #[test]
        fn block_statement() {
            test_parser!(
                "users { name, id }",
                vec![Rule {
                    selector: Selector {
                        literals: ["users".to_string()].to_vec()
                    },
                    declarations: [
                        Declaration {
                            property: "name".to_string()
                        },
                        Declaration {
                            property: "id".to_string()
                        }
                    ]
                    .to_vec()
                }]
            );
        }

        //#[test]
        //fn joint_class_selector() {
        //    test_parser!(
        //        ".users .posts {}",
        //        Program {
        //            statements: vec![Box::new(DotStatement {
        //                ident: IdentifierStatement {
        //                    literal: "users".to_string(),
        //                },
        //                block: Some(BlockStatement { properties: vec![] }),
        //            })],
        //        }
        //    );
        //}
    }
}
