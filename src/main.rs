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
