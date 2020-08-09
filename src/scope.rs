use std::collections::HashMap;
use super::token::Token;

pub struct Scope {
    // should save parent scope to be able to access elements of higher scopes.
    instances: HashMap<usize, Token>,
    instances_name: HashMap<String, usize>,
    instance_next_id: usize,

    /*
    types: HashMap<usize, Token>,
    types_name: HashMap<String, usize>,
    types_next_id: usize,
    */
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            instances: HashMap::new(),
            instances_name: HashMap::new(),
            instance_next_id: 0,
            /*
            types: HashMap::new(),
            types_name: HashMap::new(),
            types_next_id: 0,
            */
        }
    }

    pub fn add_instance_value(&mut self, id: usize, token: Token) {
        match self.instances.insert(id, token) {
            None => (),
            // WHEN CHANGE IS IMPLEMENTED: Some(_) => panic!("Duplicated key!"),
            Some(_) => (),
        }
    }
    
    pub fn add_instance_name(&mut self, name: &str) -> usize {
        let s = String::from(name);
        match self.instances_name.insert(s, self.instance_next_id) {
            Some(_) => panic!("Variable name already exists."),
            None => self.instance_next_id += 1,
        }
        self.instance_next_id - 1
    }

    pub fn add_instance_with_value(&mut self, name: &str, token: Token) {
        let id = self.add_instance_name(name);
        self.add_instance_value(id, token)
    }

    pub fn get_instance_id(&self, name: &str) -> Option<&usize> {
        self.instances_name.get(name)
    }

    pub fn get_instance_value(&self, id: usize) -> Option<&Token> {
        self.instances.get(&id)
    }

    // change_variable_value

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