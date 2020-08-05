mod lexical;
mod syntactic;
mod token;

// Key Idea: Everything is an expression
// ; "clears" the expression

// Main funtion.
// At the moment it is a loop of asking for expressions.
// Terminal prints the result of the expression
// Later it will accept files.
fn main() {
    loop {
        println!("");
        let expression = input("minprol> ");    

        let tokens = lexical::create_tokens(expression);

        let token = syntactic::process_tokens(tokens);

        println!("{:?}", token);
    }
}

fn input(message: &str) -> String {
    use std::io::{stdin, stdout, Write};

    let mut s = String::new();
    print!("{}", message);

    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    s
}
