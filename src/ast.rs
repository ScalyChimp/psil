use chumsky::Parser;
use std::error::Error;
use std::fmt::Display;

pub mod env;
mod expr;
mod parsing;

use crate::Env;
use expr::{Expr, Type};

pub fn eval(input: &str, env: &mut Env) -> Result<Expr, LispError> {
    let ast = parsing::parser().parse(input).unwrap();
    // dbg!(&ast);
    ast.eval(env)
}

#[derive(Debug)]
pub enum LispError {
    /// TypeMismatch (ExpectedType, ActualType)
    TypeMismatch(Type, Expr),

    /// Symbol which couldn't be found in the environment.
    SymbolNotFound(String),

    /// List which couldn't be evaulated.
    MalformedList(Vec<Expr>),

    /// Wrong number of arguments
    LambdaArity,
}

impl Error for LispError {}
impl Display for LispError {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LispError::TypeMismatch(expected, acquired) => write!(
                &mut f,
                "Type Mismatch: expected = {:?}, got = {:?}",
                expected, acquired
            ),
            LispError::SymbolNotFound(symbol) => {
                write!(
                    &mut f,
                    "Could not find symbol '{:?}' in environment",
                    symbol
                )
            }
            LispError::MalformedList(list) => {
                write!(&mut f, "Could not eval list '{:?}' in environment", list)
            }
            LispError::LambdaArity => {
                write!(&mut f, "Wrong number of forms expected for lambda form")
            }
        }
    }
}
