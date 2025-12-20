//! Loop Enhancements - Phase 3 Day 15
//!
//! Advanced loop constructs:
//! - While loops with condition
//! - Until loops (inverse of while)
//! - Nested loops
//! - Break/Continue statements
//! - Loop counters and metadata

use serde_json::Value;/// Loop control flow result
#[derive(Debug, Clone, PartialEq)]
pub enum LoopControl {
    /// Continue normal execution
    Continue,
    /// Break out of current loop
    Break,
    /// Continue to next iteration
    Skip,
    /// Return with value
    Return(Value),
}

/// Loop metadata - available in loop context
#[derive(Debug, Clone)]
pub struct LoopMetadata {
    /// Current iteration index (0-based)
    pub index: usize,
    /// Current iteration counter (1-based)
    pub counter: usize,
    /// Total iterations (if known)
    pub total: Option<usize>,
    /// Is first iteration
    pub is_first: bool,
    /// Is last iteration (if known)
    pub is_last: bool,
    /// Loop nesting level
    pub depth: usize,
}

impl LoopMetadata {
    pub fn new(index: usize, total: Option<usize>, depth: usize) -> Self {
        let is_last = total.map(|t| index == t - 1).unwrap_or(false);
        Self {
            index,
            counter: index + 1,
            total,
            is_first: index == 0,
            is_last,
            depth,
        }
    }
    
    pub fn to_json(&self) -> Value {
        serde_json::json!({
            "index": self.index,
            "counter": self.counter,
            "total": self.total,
            "is_first": self.is_first,
            "is_last": self.is_last,
            "depth": self.depth,
        })
    }
}

/// While loop iterator
pub struct WhileLoop<F>
where
    F: FnMut() -> Result<bool, String>,
{
    condition: F,
    max_iterations: usize,
    current: usize,
}

impl<F> WhileLoop<F>
where
    F: FnMut() -> Result<bool, String>,
{
    pub fn new(condition: F) -> Self {
        Self {
            condition,
            max_iterations: 10000, // Safety limit
            current: 0,
        }
    }
    
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }
    
    pub fn next_iteration(&mut self) -> Result<Option<LoopMetadata>, String> {
        if self.current >= self.max_iterations {
            return Err(format!("While loop exceeded maximum iterations: {}", self.max_iterations));
        }
        
        let should_continue = (self.condition)()?;
        
        if should_continue {
            let metadata = LoopMetadata::new(self.current, None, 0);
            self.current += 1;
            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }
}

/// Until loop (inverse of while)
pub struct UntilLoop<F>
where
    F: FnMut() -> Result<bool, String>,
{
    condition: F,
    max_iterations: usize,
    current: usize,
}

impl<F> UntilLoop<F>
where
    F: FnMut() -> Result<bool, String>,
{
    pub fn new(condition: F) -> Self {
        Self {
            condition,
            max_iterations: 10000,
            current: 0,
        }
    }
    
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }
    
    pub fn next_iteration(&mut self) -> Result<Option<LoopMetadata>, String> {
        if self.current >= self.max_iterations {
            return Err(format!("Until loop exceeded maximum iterations: {}", self.max_iterations));
        }
        
        let should_stop = (self.condition)()?;
        
        if !should_stop {
            let metadata = LoopMetadata::new(self.current, None, 0);
            self.current += 1;
            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }
}

/// For-each loop with enhanced metadata
pub struct ForEachLoop {
    items: Vec<Value>,
    current: usize,
    depth: usize,
}

impl ForEachLoop {
    pub fn new(items: Vec<Value>) -> Self {
        Self {
            items,
            current: 0,
            depth: 0,
        }
    }
    
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
    
    pub fn next_iteration(&mut self) -> Option<(Value, LoopMetadata)> {
        if self.current < self.items.len() {
            let item = self.items[self.current].clone();
            let metadata = LoopMetadata::new(self.current, Some(self.items.len()), self.depth);
            self.current += 1;
            Some((item, metadata))
        } else {
            None
        }
    }
    
    pub fn total_items(&self) -> usize {
        self.items.len()
    }
}

/// Range loop iterator
pub struct RangeLoop {
    start: i64,
    end: i64,
    step: i64,
    current: i64,
    depth: usize,
}

impl RangeLoop {
    pub fn new(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
            step: if start <= end { 1 } else { -1 },
            current: start,
            depth: 0,
        }
    }
    
    pub fn with_step(mut self, step: i64) -> Self {
        self.step = step;
        self
    }
    
    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
    
    pub fn next_iteration(&mut self) -> Option<(i64, LoopMetadata)> {
        let in_range = if self.step > 0 {
            self.current < self.end
        } else {
            self.current > self.end
        };
        
        if in_range {
            let value = self.current;
            let total = ((self.end - self.start).abs() / self.step.abs()) as usize;
            let index = ((self.current - self.start).abs() / self.step.abs()) as usize;
            let metadata = LoopMetadata::new(index, Some(total), self.depth);
            
            self.current += self.step;
            Some((value, metadata))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_metadata_first_item() {
        let meta = LoopMetadata::new(0, Some(5), 0);
        assert_eq!(meta.index, 0);
        assert_eq!(meta.counter, 1);
        assert_eq!(meta.total, Some(5));
        assert!(meta.is_first);
        assert!(!meta.is_last);
    }
    
    #[test]
    fn test_loop_metadata_last_item() {
        let meta = LoopMetadata::new(4, Some(5), 0);
        assert_eq!(meta.index, 4);
        assert_eq!(meta.counter, 5);
        assert!(!meta.is_first);
        assert!(meta.is_last);
    }
    
    #[test]
    fn test_loop_metadata_middle_item() {
        let meta = LoopMetadata::new(2, Some(5), 0);
        assert_eq!(meta.index, 2);
        assert_eq!(meta.counter, 3);
        assert!(!meta.is_first);
        assert!(!meta.is_last);
    }
    
    #[test]
    fn test_loop_metadata_nested() {
        let meta = LoopMetadata::new(1, Some(3), 2);
        assert_eq!(meta.depth, 2);
    }
    
    #[test]
    fn test_while_loop_basic() {
        let mut counter = 0;
        let mut while_loop = WhileLoop::new(|| {
            counter += 1;
            Ok(counter <= 3)
        });
        
        let mut iterations = 0;
        while let Some(_meta) = while_loop.next_iteration().unwrap() {
            iterations += 1;
        }
        
        assert_eq!(iterations, 3);
    }
    
    #[test]
    fn test_while_loop_never_runs() {
        let mut while_loop = WhileLoop::new(|| Ok(false));
        
        let result = while_loop.next_iteration().unwrap();
        assert!(result.is_none());
    }
    
    #[test]
    fn test_while_loop_max_iterations() {
        let mut while_loop = WhileLoop::new(|| Ok(true))
            .with_max_iterations(100);
        
        let mut count = 0;
        loop {
            match while_loop.next_iteration() {
                Ok(Some(_)) => count += 1,
                Ok(None) => break,
                Err(_) => break,
            }
        }
        
        assert_eq!(count, 100);
    }
    
    #[test]
    fn test_until_loop_basic() {
        let mut counter = 0;
        let mut until_loop = UntilLoop::new(|| {
            counter += 1;
            Ok(counter > 3)
        });
        
        let mut iterations = 0;
        while let Some(_meta) = until_loop.next_iteration().unwrap() {
            iterations += 1;
        }
        
        assert_eq!(iterations, 3);
    }
    
    #[test]
    fn test_until_loop_immediate_stop() {
        let mut until_loop = UntilLoop::new(|| Ok(true));
        
        let result = until_loop.next_iteration().unwrap();
        assert!(result.is_none());
    }
    
    #[test]
    fn test_foreach_loop() {
        let items = vec![
            serde_json::json!(1),
            serde_json::json!(2),
            serde_json::json!(3),
        ];
        
        let mut loop_iter = ForEachLoop::new(items);
        
        let mut count = 0;
        while let Some((_value, meta)) = loop_iter.next_iteration() {
            count += 1;
            assert_eq!(meta.counter, count);
            
            if count == 1 {
                assert!(meta.is_first);
                assert!(!meta.is_last);
            } else if count == 3 {
                assert!(!meta.is_first);
                assert!(meta.is_last);
            }
        }
        
        assert_eq!(count, 3);
    }
    
    #[test]
    fn test_range_loop_ascending() {
        let mut range = RangeLoop::new(1, 5);
        
        let mut values = Vec::new();
        while let Some((value, _meta)) = range.next_iteration() {
            values.push(value);
        }
        
        assert_eq!(values, vec![1, 2, 3, 4]);
    }
    
    #[test]
    fn test_range_loop_descending() {
        let mut range = RangeLoop::new(5, 1);
        
        let mut values = Vec::new();
        while let Some((value, _meta)) = range.next_iteration() {
            values.push(value);
        }
        
        assert_eq!(values, vec![5, 4, 3, 2]);
    }
    
    #[test]
    fn test_range_loop_with_step() {
        let mut range = RangeLoop::new(0, 10).with_step(2);
        
        let mut values = Vec::new();
        while let Some((value, _meta)) = range.next_iteration() {
            values.push(value);
        }
        
        assert_eq!(values, vec![0, 2, 4, 6, 8]);
    }
    
    #[test]
    fn test_range_loop_metadata() {
        let mut range = RangeLoop::new(1, 4);
        
        if let Some((_value, meta)) = range.next_iteration() {
            assert!(meta.is_first);
            assert_eq!(meta.index, 0);
            assert_eq!(meta.counter, 1);
        }
    }
    
    #[test]
    fn test_loop_control() {
        assert_eq!(LoopControl::Continue, LoopControl::Continue);
        assert_eq!(LoopControl::Break, LoopControl::Break);
        assert_eq!(LoopControl::Skip, LoopControl::Skip);
    }
}
