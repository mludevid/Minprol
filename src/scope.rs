use std::collections::HashMap;
use super::token::Token;

pub struct Scope {
    // should save parent scope to be able to access elements of higher scopes.
    variables: HashMap<String, Token>,

    /*
    types: HashMap<usize, Token>,
    types_name: HashMap<String, usize>,
    types_next_id: usize,
    */
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            /*
            types: HashMap::new(),
            types_name: HashMap::new(),
            types_next_id: 0,
            */
        }
    }

    pub fn add_variable(&mut self, name: String, token: Token) {
        if self.variables.contains_key(&name) {
            panic!("Tried to create variable twice.");
        }

        self.variables.insert(name, token);
    }

    pub fn change_variable(&mut self, name: String, token: Token) {
        if !self.variables.contains_key(&name) {
            panic!("Variable does not exist in this scope.");
        }

        self.variables.insert(name, token);
    }

    pub fn get_variable(&self, name: &String) -> &Token {
        match self.variables.get(name) {
            None => panic!("Variable does not exist yet"),
            Some(t) => t,
        }
    }

    /*
    pub fn add_type_value(&mut self, id: usize, token: Token) {
        match self.types.insert(id, token) {
            None => (),
            // WHEN CHANGE IS IMPLEMENTED: Some(_) => panic!("Duplicated key!"),
            Some(_) => (),
        }
    }
    
    pub fn add_type_name(&mut self, name: &str) -> usize {
        let s = String::from(name);
        match self.types_name.insert(s, self.types_next_id) {
            Some(_) => panic!("Type name already exists."),
            None => self.types_next_id += 1,
        }
        self.types_next_id - 1
    }

    pub fn get_type_id(&self, name: &str) -> Option<&usize> {
        self.types_name.get(name)
    }

    pub fn get_type_value(&self, id: usize) -> Option<&Token> {
        self.types.get(&id)
    }
    */
}