use super::token::Token;

#[derive(Debug, Clone)]
pub struct Function {
    expected_inputs: Vec<Token>, // If language is strongly typed here needs to be a list of the exptected type.
    instructions: Vec<Token>,
}

impl Function {
    pub fn new(expected_inputs: Vec<Token>, instructions: Vec<Token>) -> Self {
        Function {
            expected_inputs,
            instructions,
        }
    }

    pub fn call(&self, inputs: Vec<Token>) -> Token {
        if inputs.len() != self.expected_inputs.len() {
            panic!("Expected {} input(s) and got {} input(s).", self.expected_inputs.len(), inputs.len());
        }

        let mut scope = super::scope::Scope::new();

        for i in 0..self.expected_inputs.len() {
            scope.add_variable(self.expected_inputs.get(i).expect("Unexpected error. Should not happen!").to_name(),
                                inputs.get(i).expect("Unexpected error. Should not happen!").clone());
        }

        super::syntactic::process_tokens(&mut scope, &self.instructions)
    }
}