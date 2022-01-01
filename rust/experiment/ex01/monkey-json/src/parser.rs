use crate::{lexer::Token, Value};

#[derive(Debug, Clone)]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> ParserError {
        ParserError {
            msg: msg.to_string(),
        }
    }
}

pub struct Parser {
    /// `Lexer`で`tokenize`した`Token`一覧
    tokens: Vec<Token>,
    /// `tokens`の先頭
    index: usize,
}

impl Parser {
    /// `Token`の一覧を受け取り`Parser`を返す
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    /// `Array`のパースを行う
    /// [1, 2, 3, null, "string"]
    fn parse_array(&mut self) -> Result<Value, ParserError> {
        todo!()
    }

    /// `Object`のパースを行う
    /// {
    ///     "key1": 12345,
    ///     "key2": 6789,
    /// }
    fn parse_object(&mut self) -> Result<Value, ParserError> {
        todo!()
    }

    /// `Token`を評価して`Value`に変換する。この関数は再帰的に呼び出される
    fn parse(&mut self) -> Result<Value, ParserError> {
        todo!()
    }

    /// 先頭の`Token`を返す
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    /// 先頭の`Token`を返す。（先頭に`Token`があることを想定している）
    fn peek_expect(&self) -> Result<&Token, ParserError> {
        self.peek()
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }

    /// 先頭の`Token`を返して、1トークン進める
    fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.tokens.get(self.index - 1)
    }

    /// 先頭の`Token`を返して、1トークン進める。（先頭に`Token`があることを想定している）
    fn next_expect(&mut self) -> Result<&Token, ParserError> {
        self.next()
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{lexer::Lexer, Value};
    use std::collections::BTreeMap;

    #[test]
    fn test_parse_object() {}

    #[test]
    fn test_parse_array() {}

    #[test]
    fn test_parse() {}
}
