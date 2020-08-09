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
        let expression = input("minprol> ");    

        let tokens = lexical::create_tokens(&mut scope, expression);

        // println!("{:?}", tokens);

        let token = syntactic::process_tokens(&mut scope, &tokens);

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
