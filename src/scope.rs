use std::collections::HashMap;
use super::token::Token;

pub struct Scope {
    variables: HashMap<String, Token>,
    inner_scope: Option<Box<Scope>>

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
            inner_scope: None,

            /*
            types: HashMap::new(),
            types_name: HashMap::new(),
            types_next_id: 0,
            */
        }
    }

    pub fn add_variable(&mut self, name: String, token: Token) {
        match &mut self.inner_scope {
            Some(s) => s.add_variable(name, token),
            None => {
                if self.variables.contains_key(&name) {
                    panic!("Tried to create variable twice.");
                }

                self.variables.insert(name, token);
            }
        }
    }

    pub fn change_variable(&mut self, name: String, token: Token) -> Result<(), String> {
        match &mut self.inner_scope {
            Some(s) => {
                match s.change_variable(name, token.clone()) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(s) => {
                        if !self.variables.contains_key(&s) {return Err(s)}
                        self.variables.insert(s, token);
                        Ok(())
                    },
                }
            },
            None => {
                if !self.variables.contains_key(&name) {return Err(name)};
                self.variables.insert(name, token);
                Ok(())
            }
        }
    }

    pub fn get_variable(&self, name: &String) -> Result<&Token, ()> {
        match &self.inner_scope {
            Some(s) => {
                match s.get_variable(name) {
                    Ok(t) => Ok(t),
                    Err(t) => {
                        match self.variables.get(name) {
                            None => Err(t),
                            Some(s) => Ok(s),
                        }
                    }
                }
            },
            None => {
                match self.variables.get(name) {
                    None => Err(()),
                    Some(s) => Ok(s),
                }
            }
        }
    }

    pub fn create_new_inner_scope(&mut self) {
        match &mut self.inner_scope {
            None => {
                self.inner_scope = Some(Box::new(Scope::new()));
            }
            Some(c) => c.create_new_inner_scope(),
        }
    }

    pub fn destroy_inner_scope(&mut self) {
        self.inner_scope = None;
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