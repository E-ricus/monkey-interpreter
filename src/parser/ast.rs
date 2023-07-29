use crate::lexer::Token;

// Traits
pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node()
    where
        Self: Sized;
}

trait Expression: Node {
    fn expression_node();
}

// Program
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(statement) = self.statements.get(0) {
            return statement.token_literal();
        }
        String::from("")
    }
}

// LetStatement
struct LetStatement<'a, T>
where
    T: Expression,
{
    token: Token,
    name: &'a Identifier,
    value: T,
}

impl<'a, T> Node for LetStatement<'a, T>
where
    T: Expression,
{
    fn token_literal(&self) -> String {
        format!("{}", self.token)
    }
}

impl<'a, T> Statement for LetStatement<'a, T>
where
    T: Expression,
{
    fn statement_node() {}
}

// Identifier
struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        format!("{}", self.token)
    }
}

impl Expression for Identifier {
    fn expression_node() {}
}
