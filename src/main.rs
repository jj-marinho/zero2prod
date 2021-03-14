mod lexer;
mod repl;

use crate::repl::repl;
fn main() {
    println!("Welcome to Duklin alpha");
    println!("Starting REPL...");
    repl();
}
