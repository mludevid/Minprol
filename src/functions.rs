use super::token::Token;

#[derive(Debug, Clone)]
pub struct Function {
    expected_inputs: usize, // If language is strongly typed here needs to be a list of the exptected type.
    instructions: Vec<Token>,
}

impl Function {
    pub fn new(expected_inputs: usize, instructions: Vec<Token>) -> Self {
        Function {
            expected_inputs,
            instructions,
        }
    }

    pub fn call(&self, inputs: Vec<Token>) -> Token {
        if inputs.len() != self.expected_inputs {
            panic!("Expected {} input(s) and got {} input(s).", self.expected_inputs, inputs.len());
        }

        let mut scope = super::scope::Scope::new();

        // The idea is, that since in the function declaration the inputs will also be
        // declared first this is goint to map them correctly.
        // I'm sure there is a much more elegant solution. For now it seems to work.
        for input in inputs {
            scope.add_instance_with_value("", input);
        }

        println!("{:?}", scope.get_instance_value(0));

        super::syntactic::process_tokens(&mut scope, &self.instructions)
    }
}