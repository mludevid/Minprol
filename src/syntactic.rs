use super::token::*;
use super::token::bin_op::BinOp;
use super::token::special_character::SpecialCharacter;
use super::token::types::*;

pub fn process_tokens(tokens: Vec<Token>) -> Token {
    let mut cursor = TokenCursor::new(tokens);
    let token = process_math_sum(&mut cursor);
    token
}

pub fn process_math_sum(tokens: &mut TokenCursor) -> Token {
    // ADDITION
    let mut p = process_math_mul(tokens);
    
    match tokens.next() {
        Some(Token::TtBinOp(BinOp::TtPlus)) => {
            let p2 = process_math_sum(tokens);
            p = Token::TtType(add_types(p.to_type(), p2.to_type()));
        },
        Some(Token::TtBinOp(BinOp::TtMinus)) => {
            let p2 = process_math_sum(tokens);
            p = Token::TtType(substract_types(p.to_type(), p2.to_type()));
        },
        None => (),
        _ => {tokens.step_back();},
    }

    p
}

pub fn process_math_mul(tokens: &mut TokenCursor) -> Token {
    // MULTIPLICATION
    let mut p = process_math_value(tokens);

    match tokens.next() {
        Some(Token::TtBinOp(BinOp::TtMul)) => {
            let p2 = process_math_mul(tokens);
            p = Token::TtType(multiply_types(p.to_type(), p2.to_type()));
        },
        Some(Token::TtBinOp(BinOp::TtDiv)) => {
            let p2 = process_math_mul(tokens);
            p = Token::TtType(divide_types(p.to_type(), p2.to_type()));
        },
        None => (),
        _ => {tokens.step_back();},
    }

    p
}

pub fn process_math_value(tokens: &mut TokenCursor) -> Token {
    // VALUES OR (EXPRESSION) OR -VALUE
    match tokens.next() {
        Some(Token::TtType(Type::TtI32(i))) => Token::TtType(Type::TtI32(*i)),
        Some(Token::TtType(Type::TtI64(i))) => Token::TtType(Type::TtI64(*i)),
        Some(Token::TtType(Type::TtU32(i))) => Token::TtType(Type::TtU32(*i)),
        Some(Token::TtType(Type::TtU64(i))) => Token::TtType(Type::TtU64(*i)),
        Some(Token::TtType(Type::TtF32(i))) => Token::TtType(Type::TtF32(*i)),
        Some(Token::TtType(Type::TtF64(i))) => Token::TtType(Type::TtF64(*i)),
        Some(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket)) => {
            let value = process_math_sum(tokens);
            match tokens.next() {
                Some(Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket)) => (),
                _ => panic!("Missing closing bracket!")
            }
            value
        }
        Some(Token::TtBinOp(BinOp::TtMinus)) => {
            let t = process_math_value(tokens);
            match t {
                Token::TtType(i) => Token::TtType(negate_type(i)),
                _ => panic!("Fatal error. Should never happen!"),
            }
        }
        None => panic!("Unexpected end."),
        _ => panic!("Unexpected character.")
    }
}