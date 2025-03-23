use crate::parser::Parser;
use crate::ast::{Eval, Ruleset};

pub struct Program {
    pub ruleset: Ruleset,
}
impl Program {
    pub fn new(input: &str) -> Self {
        Self {
            ruleset: Parser::new(input).run()
        }
    }
    pub fn run(&mut self) -> String {
        self.ruleset
            .iter_mut()
            .map(|rule| rule.eval())
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
            test_run!("users {}", "SELECT * FROM users;");
        }
        #[test]
        fn block_statement() {
            test_run!("users { name, id }", "SELECT name, id FROM users;");
        }
        #[test]
        fn mul_statement() {
            test_run!("users {}", "SELECT * FROM users;");
        }
    }
}
