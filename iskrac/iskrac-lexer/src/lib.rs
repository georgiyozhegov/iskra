use std::char;

pub use token::Token;

#[cfg(test)]
mod test;
mod token;

pub struct Lexer<'s> {
    iter: std::iter::Peekable<std::str::Chars<'s>>,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            iter: source.chars().peekable(),
        }
    }

    fn token(&mut self) -> Option<Token> {
        let token = match self.iter.peek()? {
            c if char_kind::is_alphabetic(c) => self.begins_with_alphabetic(),
            c if char_kind::is_numeric(c) => self.begins_with_numeric(),
            c if char_kind::is_invisible(c) => self.begins_with_invisible(),
            _ => self.begins_with_other(),
        };
        Some(token)
    }

    fn begins_with_alphabetic(&mut self) -> Token {
        let buffer = self.take_while(|c| {
            char_kind::is_alphabetic(c) || char_kind::is_numeric(c)
        });
        self.keyword_or_identifier(buffer)
    }

    fn keyword_or_identifier(&mut self, buffer: String) -> Token {
        match buffer.as_str() {
            "let" => Token::Let,
            "function" => Token::Function,
            "do" => Token::Do,
            "end" => Token::End,
            "mutable" => Token::Mutable,
            "while" => Token::While,
            _ => Token::Identifier(buffer),
        }
    }

    fn begins_with_numeric(&mut self) -> Token {
        let buffer = self.take_while(|c| char_kind::is_numeric(c));
        let value = buffer.parse().expect("todo: error manager");
        Token::Integer(value)
    }

    fn begins_with_invisible(&mut self) -> Token {
        self.take_while(|c| char_kind::is_invisible(c));
        Token::Invisible
    }

    fn begins_with_other(&mut self) -> Token {
        match self.iter.next().unwrap() {
            '=' => Token::Equal,
            '-' => {
                if self.followed_by('=') {
                    Token::MinusEqual
                } else {
                    Token::Minus
                }
            }
            '+' => Token::Equal,
            '*' => Token::Asterisk,
            '>' => Token::Greater,
            '(' => Token::OpenRound,
            ')' => Token::CloseRound,
            '{' => Token::OpenCurly,
            '}' => Token::CloseCurly,
            _ => panic!("todo: error manager"),
        }
    }

    fn take_while(&mut self, predicate: impl Fn(&char) -> bool) -> String {
        let mut buffer = String::new();
        while let Some(c) = self.iter.peek() {
            if !predicate(c) {
                break;
            }
            buffer.push(self.iter.next().unwrap());
        }
        buffer
    }

    fn followed_by(&mut self, c: char) -> bool {
        if self.iter.peek() == Some(&c) {
            self.iter.next();
            true
        } else {
            false
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}

mod char_kind {
    pub fn is_alphabetic(c: &char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    pub fn is_numeric(c: &char) -> bool {
        matches!(c, '0'..='9')
    }

    pub fn is_invisible(c: &char) -> bool {
        matches!(c, ' ' | '\t' | '\n')
    }
}
