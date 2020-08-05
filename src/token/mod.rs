pub mod bin_op;
pub mod special_character;
pub mod types;

use bin_op::BinOp;
use special_character::SpecialCharacter;
use types::Type;

// TODO: Improve the overall structure of tokens. Especially the access to the values in the tokens.

#[derive(Debug)]
pub enum Token {
    TtType(types::Type),
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
}

pub struct TokenCursor {
    tokens: Vec<Token>,
    i: usize
}

impl TokenCursor {
    pub fn new(tokens: Vec<Token>) -> TokenCursor {
        TokenCursor{tokens, i: 0}
    }

    pub fn next(&mut self) -> Option<&Token> {
        let ret = self.tokens.get(self.i);
        self.i += 1;
        ret
    }

    pub fn step_back(&mut self) {
        self.i -= 1;
    }
}