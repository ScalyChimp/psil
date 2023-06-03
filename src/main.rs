#![feature(iterator_try_collect)]
use ::rustyline::error::ReadlineError;
use ast::env::{self, Env};
pub use chumsky::{prelude::*, Parser};
use clap::Parser as ArgParser;
pub use std::{
    error::Error,
    io::{self, stdout, Write},
};
use std::{fs, path::PathBuf};

mod ast;
mod rustyline;

#[derive(ArgParser)]
struct Args {
    /// File path to evaluate like a script.
    /// If ommitted, starts a repl.
    #[arg(short, long, value_name = "SCRIPT")]
    script: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut env = env::Env::default();
    match args.script {
        Some(script) => eval_script(script, &mut env),
        None => repl(&mut env),
    }
}

fn eval_script(script: PathBuf, env: &mut Env) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(script)?;
    let result = ast::eval_script(&input, env)?;
    println!("Result: {}", result);
    Ok(())
}

fn repl(env: &mut Env) -> Result<(), Box<dyn Error>> {
    let mut rl = rustyline::config()?;

    println!("fe-lisp repl v0.0.1");
    let mut input = rl.readline("λ ");
    loop {
        match input {
            Ok(ref line) => {
                if line.is_empty() {
                    input = rl.readline("λ ");
                    continue;
                }

                rl.add_history_entry(line.as_str())?;
                rl.save_history("fe-lisp.history")?;

                let result = match ast::eval_expr(&input?, env) {
                    Ok(result) => result.to_string(),
                    Err(err) => format!("Error - {err}"),
                };
                input = rl.readline(&format!("{}\nλ ", result));
            }
            Err(ReadlineError::Eof | ReadlineError::Interrupted) => {
                break Ok(());
            }
            Err(ref err) => print!("Err: {}", err),
        }
    }
}
