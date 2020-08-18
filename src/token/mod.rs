pub mod bin_op;
pub mod special_character;
pub mod types;
pub mod keywords;

use bin_op::BinOp;
use special_character::SpecialCharacter;
use types::Type;
use keywords::Keyword;
use super::functions::Function;

// TODO: Improve the overall structure of tokens. Especially the access to the values in the tokens.

#[derive(Debug, Clone)]
pub enum Token {
    TtEmpty,
    // TtEOL,
    TtName(String),
    TtKeyword(Keyword),
    TtFunction(Function),
    TtType(Type),
    TtBinOp(BinOp),
    TtSpecialCharacter(SpecialCharacter),
}

impl Token {
    pub fn to_type(&self) -> &Type {
        match &self {
            Token::TtType(i) => i,
            _ => panic!("Could not convert to Type."),
        }
    }

    pub fn to_name(&self) -> String {
        match &self {
            Token::TtName(s) => s.clone(),
            _ => panic!("Could not convert to Identifier."),
        }
    }
}

pub struct TokenCursor<'a> {
    tokens: &'a Vec<Token>,
    i: usize
}

impl<'a> TokenCursor<'a> {
    pub fn new(tokens: &Vec<Token>) -> TokenCursor {
        TokenCursor{tokens, i: 0}
    }

    pub fn next(&mut self) -> Option<&Token> {
        let ret = self.tokens.get(self.i);
        self.i += 1;
        ret
    }

    pub fn step(&mut self) {
        self.i += 1;
    }

    pub fn peak(&self) -> Option<&Token> {
        self.tokens.get(self.i)
    }

    pub fn peak_next(&self) -> Option<&Token> {
        self.tokens.get(self.i + 1)
    }

    pub fn ended(&self) -> bool {
        self.i >= self.tokens.len()
    }
}