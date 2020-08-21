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
    match tokens.peak() {
        Some(Token::TtSpecialCharacter(SpecialCharacter::TtSemicolon)) => {
            token = Token::TtEmpty;
            tokens.step();
        },
        _ => (),
    }
    token
}

fn process_expression(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    match tokens.peak().expect("Unexpected end.") {
        Token::TtName(s) => {
            let name = s.clone();
            match tokens.peak_next() {
                Some(Token::TtSpecialCharacter(SpecialCharacter::TtEqual)) => {
                    tokens.step();
                    tokens.step();
                    let t = process_math_sum(scope, tokens);
                    match scope.change_variable(name, t) {
                        Ok(_) => (),
                        Err(s) => panic!("Variable {} is not declared yet.", s)
                    };
                    Token::TtEmpty
                },
                _ => {
                    process_math_sum(scope, tokens)
                }
            }
        }
        _ => {
            process_math_sum(scope, tokens)
        },
    }
}

fn declare_variable(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    match tokens.next().expect("Unexpected end.") {
        Token::TtName(s) => {
            let name = s.clone();
            match tokens.next().expect("Unexpected end.") {
                Token::TtSpecialCharacter(SpecialCharacter::TtEqual) => {
                    let t = process_math_sum(scope, tokens);
                    scope.add_variable(name, t.clone());
                    // Token::TtEmpty (I can imagine scenarios where this decision whould be smarter)
                    t
                },
                _ => panic!("You need to asign a value to the created variable"),
            }
        },
        _ => panic!("Expected Identifier."),
    }
}

fn declare_function(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    // --- Get ID ---
    let function_name = tokens.next().expect("Unexpected end.").to_name();
    let mut inputs: Vec<Token> = Vec::new();

    // --- Create Function Struct ---
    let mut instructions: Vec<Token> = Vec::new();

    match tokens.next().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket) => (),
        _ => panic!("Expected opening bracket."),
    }
    // match inputs:
    match tokens.peak().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => tokens.step(),
        _ => {
            loop {
                inputs.push(tokens.next().expect("Unexpected end.").clone());
                match tokens.next().expect("Unexpected end.") {
                    Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => break,
                    Token::TtSpecialCharacter(SpecialCharacter::TtComma) => (),
                    _ => panic!("Unexpected character."),
                }
            }
        }
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
    scope.add_variable(function_name, Token::TtFunction(function));

    Token::TtEmpty
}

fn process_math_sum(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    // ADDITION
    let mut p = process_math_mul(scope, tokens);
    
    match tokens.peak() {
        Some(Token::TtBinOp(BinOp::TtPlus)) => {
            tokens.step();
            let p2 = process_math_sum(scope, tokens);
            p = Token::TtType(add_types(p.to_type(), p2.to_type()));
        },
        Some(Token::TtBinOp(BinOp::TtMinus)) => {
            tokens.step();
            let p2 = process_math_sum(scope, tokens);
            p = Token::TtType(substract_types(p.to_type(), p2.to_type()));
        },
        _ => (),
    }

    p
}

fn process_math_mul(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    // MULTIPLICATION
    let mut p = process_math_value(scope, tokens);

    match tokens.peak() {
        Some(Token::TtBinOp(BinOp::TtMul)) => {
            tokens.step();
            let p2 = process_math_mul(scope, tokens);
            p = Token::TtType(multiply_types(p.to_type(), p2.to_type()));
        }
        Some(Token::TtBinOp(BinOp::TtDiv)) => {
            tokens.step();
            let p2 = process_math_mul(scope, tokens);
            p = Token::TtType(divide_types(p.to_type(), p2.to_type()));
        },
        _ => (),
    }

    p
}

fn process_math_value(scope: &mut Scope, tokens: &mut TokenCursor) -> Token {
    // VALUES OR (EXPRESSION) OR -VALUE
    match tokens.next() {
        Some(Token::TtType(t)) => Token::TtType(t.clone()),
        Some(Token::TtKeyword(Keyword::TtLet)) => declare_variable(scope, tokens),
        Some(Token::TtKeyword(Keyword::TtFn)) => declare_function(scope, tokens),
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
                _ => panic!("You can only add types!"),
            }
        }
        Some(Token::TtName(name)) => {
            let t = match scope.get_variable(name) {
                Ok(tok) => tok.clone(),
                Err(_) => panic!("Could not find variable."),
            };
            match t {
                Token::TtType(t) => Token::TtType(t),
                Token::TtFunction(f) => call_function(scope, tokens, &f),
                _ => panic!("Identifier is not a type"),
            }
        }
        Some(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningCurlyBracket)) => {
            scope.create_new_inner_scope();
            let mut counter = 1;
            let mut instructions: Vec<Token> = Vec::new();
            while counter > 0 {
                match tokens.next() {
                    None => panic!("Missing closing curly brackets"),
                    Some(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningCurlyBracket)) => {
                        instructions.push(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningCurlyBracket));
                        counter += 1
                    },
                    Some(Token::TtSpecialCharacter(SpecialCharacter::TtClosingCurlyBracket)) => {
                        instructions.push(Token::TtSpecialCharacter(SpecialCharacter::TtClosingCurlyBracket));
                        counter -= 1
                    },
                    Some(t) => instructions.push(t.clone()),
                }
            }
            instructions.pop(); // returns Token::TtSpecialCharacter(SpecialCharacter::TtClosingCurlyBracket)
            let t = process_tokens(scope, &instructions);
            scope.destroy_inner_scope();
            t
        },
        None => panic!("Unexpected end."),
        Some(x) => panic!("Unexpected character: {:?}", x)
    }
}

fn call_function(scope: &mut Scope, tokens: &mut TokenCursor, f: &Function) -> Token {
    match tokens.next().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket) => (),
        _ => panic!("Expected opening bracket."),
    }

    let mut inputs: Vec<Token> = Vec::new();

    match tokens.peak().expect("Unexpected end.") {
        Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => tokens.step(),
        _ => {
            loop {
                inputs.push(process_math_sum(scope, tokens));
                match tokens.next().expect("Unexpected end.") {
                    Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket) => break,
                    Token::TtSpecialCharacter(SpecialCharacter::TtComma) => (),
                    _ => panic!("Unexpected character."),
                }
            }
        }
    }

    f.call(inputs)
}