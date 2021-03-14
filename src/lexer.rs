#[derive(PartialEq, Debug)]
enum Token<'a> {
    // Basic Syntax Blocks
    Assign,    // Assignment "="
    Comma,     // Separator ","
    Semicolon, // End of statement ";"
    LParen,    // Left Parenthesis "("
    RParen,    // Right Parenthesis ")"
    LBrace,    // Left Brace "{"
    RBrace,    // Right Brace "}"

    // Math
    Plus,     // Plus "+"
    Minus,    // Minus "-"
    Asterisk, // Multiplication "*"
    Slash,    // Division "/"

    // Simple Comparators
    LT,   // Lower Than "<"
    GT,   // Grether Than ">"
    Bang, // Negation "!"

    // Multi Character Comparators
    LTE,   // Less Than or Equal "<="
    GTE,   // Greater Than or Equal ">="
    Eq,    // Equal "=="
    NotEq, // Not Equal "!="

    // Multi character types
    Ident(&'a str), // An identity
    Int(i64),       // An integer

    // Keywords - Declation
    Function, // Function statement "fn"
    Let,      // Declaration "let"

    // Keywords - Boolean
    True,
    False,

    // Keywords - Program Flow
    If,
    Else,
    Return,

    // Miscellaneous
    Illegal, // Illegal token
    EOF,     // End of File
}

#[derive(Debug)]
struct Lexer<'a> {
    input: &'a str,       // Data to be lexed
    position: usize,      // Current position being lexed
    read_position: usize, // next position to be lexed
    ch: char,             // current char being lexed
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        // Skipping any whitespace
        while self.ch.is_whitespace() {
            self.read_char();
        }

        let token = match self.ch {
            // Basic Syntax
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,

            // Math
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,

            // Comparison and Assignment
            '>' => match self.peek_next() {
                '=' => {
                    self.read_char();
                    Token::GTE
                }
                _ => Token::GT,
            },
            '<' => match self.peek_next() {
                '=' => {
                    self.read_char();
                    Token::LTE
                }
                _ => Token::LT,
            },
            '!' => match self.peek_next() {
                '=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            '=' => match self.peek_next() {
                '=' => {
                    self.read_char();
                    Token::Eq
                }
                _ => Token::Assign,
            },

            // Keywords
            ch if ch.is_alphabetic() => return self.read_word(),

            // Numbers
            ch if ch.is_digit(10) => return self.read_int(),

            // Miscelaneous
            '\0' => Token::EOF,
            _ => Token::Illegal,
        };

        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        match self.read_position >= self.input.len() {
            true => self.ch = '\0',
            false => self.ch = self.input.chars().nth(self.read_position).unwrap_or('\0'),
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_next(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }
        self.input.chars().nth(self.read_position).unwrap()
    }
    fn read_word(&mut self) -> Token {
        // Start of char sequence
        let idx = self.position;

        // Identifying size of char sequence
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }

        // Checking if the sequence is a reserved keyword
        return match &self.input[idx..self.position] {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "if" => Token::If,
            "else" => Token::Else,
            "false" => Token::False,
            "return" => Token::Return,
            identifier => Token::Ident(identifier),
        };
    }

    fn read_int(&mut self) -> Token {
        // Start of int sequence
        let idx = self.position;

        // Identifying size of int sequence
        while self.ch.is_digit(10) {
            self.read_char();
        }

        // Parsing sequence as an i64 and returning
        return Token::Int(self.input[idx..self.position].parse::<i64>().unwrap());
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };
            
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            
            10 != 9;
            10 == 10;
            ";

        let parsed_correctly = vec![
            Token::Let,
            Token::Ident("five"),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten"),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add"),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x"),
            Token::Comma,
            Token::Ident("y"),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x"),
            Token::Plus,
            Token::Ident("y"),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result"),
            Token::Assign,
            Token::Ident("add"),
            Token::LParen,
            Token::Ident("five"),
            Token::Comma,
            Token::Ident("ten"),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::GT,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in parsed_correctly.into_iter() {
            let token = lexer.next_token();
            println!("{:?} é igual a {:?}", token, expected_token);
            assert_eq!(token, expected_token)
        }
    }
}
