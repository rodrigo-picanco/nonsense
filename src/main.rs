// The example of the language we want to write
//
//
// .users {
// } 
//
// results in: 
//
// SELECT * FROM users;

fn main() {
}

type TokenKind = &'static str;
const DOT: TokenKind = "DOT";
const RBRACK: TokenKind= "RBRACK";
const LBRACK: TokenKind = "LBRACK";
const IDENT: TokenKind = "IDENT";
const EOF: TokenKind = "EOF";

struct Node {
    token: Token,
    children: Vec<Node>
}

trait Statement {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
    fn node(&self) -> Node;
    fn eval(&self) -> String;
}

struct Program {
    statements: Vec<Box<dyn Statement>>
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

struct Identifier {
    token: Token,
    literal: String
}

struct DotStatement {
    token: Token,
    ident: Identifier
}
impl Statement for DotStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!(".{}", self.ident.literal)
    }

    fn eval(&self) -> String {
        format!("SELECT * FROM {};", self.ident.literal)
    }

    fn node(&self) -> Node {
        Node { token: self.token.clone(), children: vec![] }
    }
}

struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>
}
impl Statement for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn eval(&self) -> String {
        "".to_string()
    }

    fn string(&self) -> String {
        "NOT IMPLEMENTED".to_string()
    }

    fn node(&self) -> Node {
        Node { token: self.token.clone(), children: vec![] }
    }
}


struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token
}
impl Parser {
    fn new(input: &str ) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self { lexer: lexer, current_token, peek_token }
    }

    fn run(&mut self) -> Program {
        let mut program = Program { statements: vec![] };
        while self.current_token.kind != "EOF" {
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
            "DOT" => self.parse_dot(),
            "LBRACK" => self.parse_block_statement(),
            _ => panic!("Unknown token {}", self.current_token.kind)
        }
    }

    fn parse_block_statement(&mut self) -> Box<dyn Statement> {
        self.next_token();
        self.next_token();
        Box::new(BlockStatement { token: self.current_token.clone(), statements: vec![] })
    }

    fn parse_dot(&mut self) -> Box<dyn Statement> {
        let token = self.current_token.clone();
        self.next_token();
        let ident = self.parse_identifier();
        let statement = DotStatement { token, ident };
        Box::new(statement)
    }

    fn parse_identifier(&mut self) -> Identifier {
        Identifier { token: self.current_token.clone(), literal: self.current_token.literal.clone() }
    }
}

struct Token{
    kind: TokenKind,
    literal: String
}
impl Clone for Token {
    fn clone(&self) -> Self {
        Self { kind: self.kind, literal: self.literal.clone() }
    }
}
struct Lexer {
    input: String,
    position: u64,
    read_position: u64,
    character: char
}
impl Lexer {
    fn new(input: &str) -> Self {
        Self { input: input.to_string(), position: 0, read_position: 1, character: input.chars().nth(0).unwrap() }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.character {
            '.' => Token { kind: DOT, literal: ".".to_string() },
            '{' => Token { kind: LBRACK, literal: "{".to_string() },
            '}' => Token { kind: RBRACK, literal: "}".to_string() },
            '0' => Token { kind: EOF, literal: "".to_string() },
            _ => {
                if self.character.is_alphabetic() {
                    return Token { kind: IDENT, literal: self.read_identifier() }
                } else {
                    panic!("Unknown token {}", self.character);
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
        self.input.chars().skip(position as usize).take((self.position - position) as usize).collect::<String>().clone()
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

    #[test]
    fn test_lexer() {
        let expected = vec![
            Token { kind: DOT, literal: ".".to_string() }, 
            Token { kind: IDENT, literal: "users".to_string() },
            Token { kind: LBRACK, literal: "{".to_string() },
            Token { kind: RBRACK, literal: "}".to_string() }
        ];
        let mut result = Lexer::new(".users {}");

        expected.iter().for_each(|expected_token| {
            let token = result.next_token();
            assert_eq!(token.literal, expected_token.literal);
        })
    }

    #[test]
    fn test_parser() {
        let input = ".users {}";
        let mut parser = Parser::new(input); 
        let expected_tree = Program { statements: vec![
            Box::new(DotStatement { token: Token { kind: DOT, literal: ".".to_string() }, ident: Identifier { token: Token { kind: IDENT, literal: "users".to_string() }, literal: "users".to_string() } })
        ]};
        let result = parser.run();
        expected_tree.statements.iter().zip(result.statements.iter()).for_each(|(expected, result)| {
            assert_eq!(expected.token_literal(), result.token_literal());
        });
    }

    #[test]
    fn test_run() {
        assert_eq!(Program::new(".users {}").run(), "SELECT * FROM users; ");
    }
}
