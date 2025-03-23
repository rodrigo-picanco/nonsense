pub type Ruleset = Vec<Rule>;

pub trait Eval {
    fn eval(&mut self) -> String;
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
}
impl Eval for Rule {
    fn eval(&mut self) -> String {
        format!("SELECT {columns} FROM {tables};",
            columns =  match self.declarations.len() {
                0 => "*".to_string(),
                _ => self.declarations.iter_mut().map(|declaration| declaration.eval()).collect::<Vec<String>>().join(", ")
             },
            tables = self.selector.eval()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    pub property: String,
    // TODO: This should allow expressions to be evaluated
    // expression: String,
}
impl Eval for Declaration {
    fn eval(&mut self) -> String {
        self.property.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct Selector {
    pub literals: Vec<String>,
}
impl Eval for Selector {
    fn eval(&mut self) -> String {
        self.literals.first().unwrap().to_string()
    }
}
