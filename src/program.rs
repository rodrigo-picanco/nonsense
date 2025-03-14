use crate::parser::{Parser, Statement};

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Program {
    pub fn new(input: &str) -> Self {
        Parser::new(input).run()
    }
    pub fn run(&self) -> String {
        self.statements
            .iter()
            .map(|statement| statement.eval())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod test_run {
        use super::*;
        macro_rules! test_run {
            ($input: expr, $expected: expr) => {
                assert_eq!(Program::new($input).run(), $expected);
            };
        }
        #[test]
        fn class_selector() {
            test_run!(".users {}", "SELECT * FROM users;");
        }
        #[test]
        fn block_statement() {
            test_run!(".users { name, id }", "SELECT name, id FROM users;");
        }
        #[test]
        fn mul_statement() {
            test_run!(".users {}", "SELECT * FROM users;");
        }
    }
}
