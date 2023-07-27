mod lexer;
mod repl;

use std::io;

fn main() {
    println!("Hello This is the Monkey programming language!");
    println!("Feel free to type in commands!");

    if repl::start(io::stdin(), io::stdout()).is_err() {
        panic!("Something went wrong!")
    }
}
