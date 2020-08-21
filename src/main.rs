mod lexical;
mod syntactic;
mod token;
mod scope;
mod functions;

// Key Idea: Everything is an expression
// ; "clears" the expression
// At the moment the language is not strongly typed and therefore it generates many runtime errors.
// Since compilation time and runtime is the "same" in this case it does not bother me a lot.
// But it clearly is a thing that can be improved!

// Main funtion.
// At the moment it is a loop of asking for expressions.
// Terminal prints the result of the expression
// Later it will accept files.
fn main() {
    let mut scope = scope::Scope::new();
    loop {
        println!("");
        let expression = input("minprol> ", 0, 0);

        let tokens = lexical::create_tokens(expression);

        // println!("{:?}", tokens);

        let token = syntactic::process_tokens(&mut scope, &tokens);

        println!("{:?}", token);
    }
}

fn input(message: &str, parenthesis_count: usize, curly_parenthesis_count: usize) -> String {
    use std::io::{stdin, stdout, Write};

    let mut s = String::new();
    print!("{}", message);

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let parenthesis_count_opening = s.matches("(").count() + parenthesis_count;
    let curly_parenthesis_count_opening = s.matches("{").count() + curly_parenthesis_count;

    let parenthesis_count_closing = s.matches(")").count();
    let curly_parenthesis_count_closing = s.matches("}").count();

    if parenthesis_count_opening < parenthesis_count_closing {
        panic!("Unexpected closing bracket");
    }

    if curly_parenthesis_count_opening < curly_parenthesis_count_closing {
        panic!("Unexpected closing curly bracket");
    }

    let parenthesis = parenthesis_count_opening - parenthesis_count_closing;
    let curly_parenthesis = curly_parenthesis_count_opening - curly_parenthesis_count_closing;

    if parenthesis > 0 || curly_parenthesis > 0 {
        s.extend(input(">>>> ", parenthesis, curly_parenthesis).chars());
    }

    s
}

#[cfg(test)]
mod functionality {
    use super::scope;
    use super::token;
    use super::lexical;
    use super::syntactic;

    #[test]
    fn math_expression() {
        let mut expressions_results: Vec<(String, token::Token)> = Vec::new();
        let mut scope = scope::Scope::new();

        //Adding test cases
        expressions_results.push((String::from("2+3+4"), token::Token::TtType(token::types::Type::TtI32(9))));
        expressions_results.push((String::from("12-100"), token::Token::TtType(token::types::Type::TtI32(-88))));
        expressions_results.push((String::from("111*-15"), token::Token::TtType(token::types::Type::TtI32(-1665))));
        expressions_results.push((String::from("1324/13"), token::Token::TtType(token::types::Type::TtI32(101))));

        expressions_results.push((String::from("2u+3u+4u"), token::Token::TtType(token::types::Type::TtU32(9))));
        expressions_results.push((String::from("123u-100u"), token::Token::TtType(token::types::Type::TtU32(23))));
        expressions_results.push((String::from("111u*15u"), token::Token::TtType(token::types::Type::TtU32(1665))));
        expressions_results.push((String::from("1324u/13u"), token::Token::TtType(token::types::Type::TtU32(101))));

        expressions_results.push((String::from("2.3+3.5+4.1"), token::Token::TtType(token::types::Type::TtF32(9.9))));
        expressions_results.push((String::from("12.5-100.8"), token::Token::TtType(token::types::Type::TtF32(-88.3))));
        expressions_results.push((String::from("111.1*15.1"), token::Token::TtType(token::types::Type::TtF32(1677.61))));
        expressions_results.push((String::from("1324.5/12."), token::Token::TtType(token::types::Type::TtF32(110.375))));

        expressions_results.push((String::from("2.3d+3.4d+4.1d"), token::Token::TtType(token::types::Type::TtF64(9.8))));
        expressions_results.push((String::from("12.5d-100.8d"), token::Token::TtType(token::types::Type::TtF64(-88.3))));
        expressions_results.push((String::from("111.1d*15.1d"), token::Token::TtType(token::types::Type::TtF64(1677.61))));
        expressions_results.push((String::from("1324.5d/12.d"), token::Token::TtType(token::types::Type::TtF64(110.375))));

        expressions_results.push((String::from("(2+3)*4"), token::Token::TtType(token::types::Type::TtI32(20))));
        expressions_results.push((String::from("(2-5)/3"), token::Token::TtType(token::types::Type::TtI32(-1))));

        //Running test cases
        for (expression, result) in expressions_results {
            let tokens = lexical::create_tokens(expression);
            assert_eq!(syntactic::process_tokens(&mut scope, &tokens), result);
        }
    }
}