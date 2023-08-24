use std::sync::Arc;

use crate::{func, error::Error};


#[derive(Debug, Clone)]
pub struct CacheVariable {
    pub name: String,
    pub value: ExpType,
}

#[derive(Clone)]
pub struct CacheFunction {
    pub name: String,
    pub function: Box<Arc<dyn Fn(Vec<ExpType>) -> Result<ExpType, Error>>>,
}

#[derive(Clone)]
pub struct Cache {
    pub variables: Vec<CacheVariable>,
    pub functions: Vec<CacheFunction>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ExpType {
    INT(i32),
    FLOAT(f32),
    STRING(String),
    BOOL(bool),
}

impl ExpType {
    pub fn to_string(&self) -> String {
        match self {
            ExpType::INT(i) => i.to_string(),
            ExpType::FLOAT(f) => f.to_string(),
            ExpType::STRING(s) => s.to_string(),
            ExpType::BOOL(b) => b.to_string(),
        }
    }
}

impl CacheVariable {
    pub fn new(name: String, value: ExpType) -> Self {
        Self {
            name,
            value,
        }
    }
}

impl CacheFunction {
    pub fn new(name: String, function: Box<Arc<dyn Fn(Vec<ExpType>) -> Result<ExpType, Error>>>) -> Self {
        Self {
            name,
            function,
        }
    }

}
    

impl Cache {
    pub fn new(size: usize) -> Self {
        Self {
            variables: Vec::with_capacity(size),
            functions: Self::init_functions(),
        }
    }

    fn init_functions() -> Vec<CacheFunction> {
        vec![
            CacheFunction::new("hostname".to_string(), Box::new(Arc::new(func::hostname))),
            CacheFunction::new("random_str".to_string(), Box::new(Arc::new(func::random_str))),
            CacheFunction::new("random_num".to_string(), Box::new(Arc::new(func::random_num))),
            CacheFunction::new("random_bool".to_string(), Box::new(Arc::new(func::random_bool))),
            CacheFunction::new("current_time".to_string(), Box::new(Arc::new(func::current_time))),
        ]
    }

    pub fn functions(&self) -> &Vec<CacheFunction> {
        &self.functions
    }

    pub fn variables(&self) -> &Vec<CacheVariable> {
        &self.variables
    }

    pub fn add_variable(&mut self, variable: CacheVariable) {
        self.variables.push(variable);
    }

    pub fn add_function<F>(&mut self, name: &str, function: F) 
    where
        F: Fn(Vec<ExpType>) -> Result<ExpType, Error> + 'static,
    {
        self.functions.push(CacheFunction::new(name.to_string(), Box::new(Arc::new(function))));
    }

    pub fn call_function(&mut self, name: &str, params: Vec<ExpType>) -> Result<ExpType, Error>{
        (*(self.functions.iter().find(|f| f.name == name).unwrap().function).clone())(params)
    }

    pub fn get_variable(&self, name: &str) -> Option<&CacheVariable> {
        self.variables.iter().find(|v| v.name == name)
    }

    pub fn remove_variable(&mut self, name: &str) {
        self.variables.retain(|v| v.name != name);
    }

    pub fn set_variable(&mut self, name: &str, value: ExpType) {
        if let Some(variable) = self.get_variable(name) {
            let mut variable = variable.clone();
            variable.value = value;
            self.remove_variable(name);
            self.add_variable(variable);
        }
    }
}