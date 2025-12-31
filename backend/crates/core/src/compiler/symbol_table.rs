use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    name_to_index: HashMap<String, usize>,
    index_to_name: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            name_to_index: HashMap::new(),
            index_to_name: Vec::new(),
        }
    }

    /// Register a new variable and return its unique index.
    /// If the variable is already registered, return the existing index.
    pub fn register(&mut self, name: String) -> usize {
        if let Some(&index) = self.name_to_index.get(&name) {
            index
        } else {
            let index = self.index_to_name.len();
            self.name_to_index.insert(name.clone(), index);
            self.index_to_name.push(name);
            index
        }
    }

    /// Get the index for a variable name.
    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.name_to_index.get(name).copied()
    }

    /// Get the name for a variable index.
    pub fn get_name(&self, index: usize) -> Option<&str> {
        self.index_to_name.get(index).map(|s| s.as_str())
    }

    /// Get the number of registered variables.
    pub fn len(&self) -> usize {
        self.index_to_name.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.index_to_name.is_empty()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}