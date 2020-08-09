use super::token::{Token, bin_op::BinOp, special_character::SpecialCharacter, types::Type, keywords::Keyword};

pub fn create_tokens(expression: String) -> Vec<Token> {
    let mut token: Vec<Token> = Vec::new();

    let mut i = 0;
    while i < expression.len() {
        let character = &expression[i..=i];
        if character.chars().next().expect("Fatal error. Should never happen!").is_numeric() {
            let x = parse_number(&expression, &mut i);
            token.push(x);
        } else if character.chars().next().expect("falat error. Should never happen!").is_alphabetic() {
            let s = parse_string(&expression, &mut i);
            token.push(s);
        } else if character == "=" {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtEqual));
        } else if character == "+" {
            token.push(Token::TtBinOp(BinOp::TtPlus));
        } else if character == "-" {
            token.push(Token::TtBinOp(BinOp::TtMinus));
        } else if character == "*" {
            token.push(Token::TtBinOp(BinOp::TtMul));
        } else if character == "/" {
            token.push(Token::TtBinOp(BinOp::TtDiv));
        // TODO: Implement to the power of.
        } else if character == "(" {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningBracket));
        } else if character == ")" {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtClosingBracket));
        } else if character == "{" {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtOpeningCurlyBracket));
        } else if character == "}" {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtClosingCurlyBracket));
        } else if character == ";" {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtSemicolon));
        } else if character == "," {
            token.push(Token::TtSpecialCharacter(SpecialCharacter::TtComma))
        } else if character == " " {
        } else {
            // TODO: Create proper Error handling for the whole project.
            panic!("Character Error");
        }

        i += 1;
    }

    token
}

fn parse_number(expression: &String, i: &mut usize) -> Token {
    let start = *i;
    *i += 1;
    while *i < expression.len() {
        let character = &expression[*i..=*i];
        if !character.chars().next().expect("Fatal error. Should never happen!").is_numeric() {
            break;
        }
        *i += 1;
    }
    if *i < expression.len() && &expression[*i..=*i] == "." {
        // FLOATING POINT
        *i += 1;
        while *i < expression.len() {
            let character = &expression[*i..=*i];
            if !character.chars().next().expect("Fatal error. Should never happen!").is_numeric() {
                break;
            }
            *i += 1;
        }
        if *i < expression.len() {
            match &expression[*i..=*i] {
                "d" | "D" => {
                    return Token::TtType(Type::TtF64(expression[start..*i].parse().expect("Fatal number parsing error.")));
                }
                _ => {
                    *i -= 1;
                    return Token::TtType(Type::TtF32(expression[start..*i+1].parse().expect("Fatal number parsing error.")));
                }
            }
        } else {
            return Token::TtType(Type::TtF32(expression[start..*i].parse().expect("Fatal number parsing error.")));
        }

    } else {
        // INTEGER
        if *i < expression.len() {
            match &expression[*i..=*i] {
                "u" | "U" => {
                    *i += 1;
                    if *i < expression.len() {
                        match &expression[*i..=*i] {
                            "l" | "L" => {
                                // U64
                                return Token::TtType(Type::TtU64(expression[start..*i-1].parse().expect("Fatal number parsing error.")));
                            }
                            _ => {
                                *i -= 1;
                                // U32
                                return Token::TtType(Type::TtU32(expression[start..*i].parse().expect("Fatal number parsing error.")));
                            }
                        }
                    } else {
                        // U32
                        return Token::TtType(Type::TtU32(expression[start..*i-1].parse().expect("Fatal number parsing error.")));
                    }
                }
                "l" | "L" => {
                    // I64
                    return Token::TtType(Type::TtI64(expression[start..*i].parse().expect("Fatal number parsing error.")));
                }
                _ => {
                    // I32
                    *i -= 1;
                    return Token::TtType(Type::TtI32(expression[start..*i+1].parse().expect("Fatal number parsing error.")));
                }
            }
        } else {
            // I32
            return Token::TtType(Type::TtI32(expression[start..*i].parse().expect("Fatal number parsing error.")));
        }
    }
}

fn parse_string<'a>(expression: &'a String, i: &mut usize) -> Token {
    let start = *i;
    *i += 1;

    while *i < expression.len() {
        let character = &expression[*i..=*i];
        if !character.chars().next().expect("Fatal error. Should never happen!").is_alphabetic() {
            break;
        }
        *i += 1;
    }

    if &expression[start..*i] == "fn" {
        *i -= 1;

        return Token::TtKeyword(Keyword::TtFn);
    }

    if &expression[start..*i] == "let" {
        *i -= 1;
        
        return Token::TtKeyword(Keyword::TtLet);
    }

    *i -= 1;

    Token::TtName(String::from(&expression[start..=*i]))
}