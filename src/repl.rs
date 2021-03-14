use crate::lexer::{Lexer, Token};
use std::io::{stdin, stdout, Write};

pub fn repl() {
    let mut s = String::new();

    loop {
        print!(">> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();

        let mut lexer = Lexer::new(&s);
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);

            if token == Token::EOF {
                break;
            }
        }
        s.clear();
    }
}
