use super::token::*;
use super::token::bin_op::BinOp;
use super::token::special_character::SpecialCharacter;
use super::token::keywords::Keyword;
use super::token::types::*;
use super::scope::Scope;
use super::functions::Function;

pub fn process_tokens(scope: &mut Scope, tokens: &Vec<Token>) -> Token {
    let mut cursor = TokenCursor::new(tokens);
    let mut token = process_line(scope, &mut cursor);
    while !cursor.ended() {
        match token {
            Token::TtEmpty => (),
            _ => panic!("Expected ';'"),
        }
        token = process_line(scope, &mut cursor);
    }
    token
}

pub fn process_line(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    let mut token = process_expression(scope, tokens);
    match tokens.next() {
        Some(Token::TtSpecialCharacter(SpecialCharacter::TtSemicolon)) => token = Token::TtEmpty,
        _ => tokens.step_back(), // panic!("Line should have ended."),
    }
    token
}

fn process_expression(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    match tokens.next() {
        Some(Token::TtIdentifier(id)) => {
            let i = *id;
            match tokens.next() {
                Some(Token::TtSpecialCharacter(SpecialCharacter::TtEqual)) => {
                    // assign value to identifier
                    let value = process_expression(scope, tokens);
                    scope.add_instance_value(i, value.clone());
                    value
                },
                _ => {
                    tokens.step_back();
                    tokens.step_back();
                    process_math_sum(scope, tokens)
                },
            }
        },
        Some(Token::TtKeyword(Keyword::TtFn)) => {
            process_function(scope, tokens)
        },
        _ => {
            tokens.step_back();
            process_math_sum(scope, tokens)
        },
    }
}

fn process_function(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    // --- Get ID ---
    let function_id = *tokens.next().expect("Unexpected end.").to_identifier();
    let mut inputs: usize = 0;

    // --- Create Function Struct ---
    let mut instructions: Vec<Token> = Vec::new();

    match tokens.next().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket) => (),
        _ => panic!("Expected opening bracket."),
    }
    // match inputs:
    match tokens.next().expect("Unexpected end.") {
        Token::TtType(Type::TtI32(i)) => {
            inputs = *i as usize;
            match tokens.next().expect("Unexpected end.") {
                Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => (),
                _ => panic!("Expected closing bracket."),
            }
        },
        Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => (),
        _ => panic!("Expected inputs or closing bracket."),
    }

    match tokens.next().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtOpeningCurlyBracket) => (),
        _ => panic!("Expected opening curly bracket."),
    }

    loop {
        match tokens.next().expect("Unexpected end.") {
            Token::TtSpecialCharacter(SpecialCharacter::TtClosingCurlyBracket) => break,
            t => instructions.push(t.clone()),
        }
    }

    // --- Asign Function Struct to ID ---
    let function = Function::new(inputs, instructions);
    scope.add_instance_value(function_id, Token::TtFunction(function));

    Token::TtEmpty
}

fn process_math_sum(scope: &Scope, tokens: &mut TokenCursor) -> Token {
    // ADDITION
    let mut p = process_math_mul(scope, tokens);
    
    match tokens.next() {
        Some(Token::TtBinOp(BinOp::TtPlus)) => {
            let p2 = process_math_sum(scope, tokens);
            p = Token::TtType(add_types(p.to_type(), p2.to_type()));
        },
        Some(Token::TtBinOp(BinOp::TtMinus)) => {
            let p2 = process_math_sum(scope, tokens);
            p = Token::TtType(substract_types(p.to_type(), p2.to_type()));
        },
        None => (),
        _ => {tokens.step_back();},
    }

    p
}

fn process_math_mul(scope: &Scope, tokens: &mut TokenCursor) -> Token {
    // MULTIPLICATION
    let mut p = process_math_value(scope, tokens);

    match tokens.next() {
        Some(Token::TtBinOp(BinOp::TtMul)) => {
            let p2 = process_math_mul(scope, tokens);
            p = Token::TtType(multiply_types(p.to_type(), p2.to_type()));
        },
        Some(Token::TtBinOp(BinOp::TtDiv)) => {
            let p2 = process_math_mul(scope, tokens);
            p = Token::TtType(divide_types(p.to_type(), p2.to_type()));
        },
        None => (),
        _ => {tokens.step_back();},
    }

    p
}

fn process_math_value(scope: &Scope, tokens: &mut TokenCursor) -> Token {
    // VALUES OR (EXPRESSION) OR -VALUE
    match tokens.next() {
        Some(Token::TtType(t)) => Token::TtType(t.clone()),
        Some(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket)) => {
            let value = process_math_sum(scope, tokens);
            match tokens.next() {
                Some(Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket)) => (),
                _ => panic!("Missing closing bracket!")
            }
            value
        }
        Some(Token::TtBinOp(BinOp::TtMinus)) => {
            let t = process_math_value(scope, tokens);
            match t {
                Token::TtType(i) => Token::TtType(negate_type(i)),
                _ => panic!("Fatal error. Should never happen!"),
            }
        }
        Some(Token::TtIdentifier(id)) => {
            match scope.get_instance_value(*id).expect("Fatal error. Should not happen!") {
                Token::TtType(t) => Token::TtType(t.clone()),
                Token::TtFunction(f) => call_function(scope, tokens, &f),
                _ => panic!("Identifier is not a type"),
            }
        }
        None => panic!("Unexpected end."),
        Some(x) => panic!("Unexpected character: {:?}", x)
    }
}

fn call_function(scope: &Scope, tokens: &mut TokenCursor, f: &Function) -> Token {
    match tokens.next().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket) => (),
        _ => panic!("Expected opening bracket."),
    }

    let mut inputs: Vec<Token> = Vec::new();

    loop {
        match tokens.next().expect("Unexpected end.") {
            Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => break,
            _ => {
                tokens.step_back();
                let processed_input = process_math_sum(scope, tokens);
                inputs.push(processed_input);
            },
        }
    }

    f.call(inputs)
}