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

struct Token{
    kind: TokenKind,
    literal: String
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
}
