use crate::lexer::Token;

// Traits
pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement<N: Node> {
    fn statement_node();
}

trait Expression<N: Node> {
    fn expression_node();
}

// Program
pub struct Program<T>
where
    T: Node,
    T: Statement<T>,
{
    statements: Vec<T>,
}

impl<T> Node for Program<T>
where
    T: Node,
    T: Statement<T>,
{
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
    T: Node,
    T: Expression<T>,
{
    token: Token,
    name: &'a Identifier,
    value: T,
}

impl<'a, T> Node for LetStatement<'a, T>
where
    T: Node,
    T: Expression<T>,
{
    fn token_literal(&self) -> String {
        format!("{}", self.token)
    }
}

impl<'a, T> Statement<T> for LetStatement<'a, T>
where
    T: Node,
    T: Expression<T>,
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

impl<T: Node> Expression<T> for Identifier {
    fn expression_node() {}
}
