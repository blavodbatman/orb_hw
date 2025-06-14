#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;

struct Logger {
    cmd: RefCell<ParserMessage>,
}

struct ParserMessage {
    autocomplete: String,
}

struct Input<'a> {
    input: String,
    logger: &'a Logger,
}

impl Input<'_> {
    fn read(&mut self) -> String {
        let whitespace = self.input.find(' ').unwrap_or(self.input.len()); // позиция первого пробела
        let expression: String = self.input.drain(..whitespace).collect(); // часть до пробела
        if !self.input.is_empty() {
            self.input.drain(..1); // оставляем от input часть после пробела
        }
        if self.logger.cmd.borrow().autocomplete == "y" {
            self.logger.cmd.borrow_mut().autocomplete = "n".to_owned();
            if !expression.ends_with('}') {
                return expression + "}";
            }
        }
        expression
    }
}

struct Lexer<'a> {
    // не модифицируйте эту структуру
    input: Input<'a>,
}

impl Lexer<'_> {
    fn call(&mut self) -> String {
        let from_input = self.input.read();
        if from_input.starts_with('{') {
            return "block_start:".to_owned() + &from_input;
        }
        if from_input.is_empty() {
            return "end".to_owned();
        }
        from_input
    }
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    logger: &'a Logger,
}

impl Parser<'_> {
    fn parse(&mut self) -> String {
        let mut parsed = vec![];
        let mut value = self.lexer.call();

        while &value != "end" {
            let mut v = value;
            if v.starts_with("block_start:") {
                let fixed_v = v.strip_prefix("block_start:").unwrap();
                self.logger.cmd.borrow_mut().autocomplete = "y".to_owned();
                v = fixed_v.to_owned();
            }
            parsed.push(v);
            value = self.lexer.call();
        }

        parsed.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "{ab aba ba {bb bb} {ab aa".to_owned();
        let expected = "{ab aba} ba {bb bb} {ab aa}".to_owned();

        let logger = &Logger {
            cmd: RefCell::new(ParserMessage {
                autocomplete: "n".to_owned(),
            }),
        };
        let mut p = Parser {
            logger,
            lexer: Lexer {
                input: Input { input, logger },
            },
        };
        assert_eq!(p.parse(), expected);
    }
}
