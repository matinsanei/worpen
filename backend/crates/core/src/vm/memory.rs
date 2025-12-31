use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ExecutionMemory {
    data: Vec<Value>,
}

impl ExecutionMemory {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(64), // Pre-allocate with reasonable capacity
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: Value) {
        if index >= self.data.len() {
            self.data.resize(index + 1, Value::Null);
        }
        self.data[index] = value;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}