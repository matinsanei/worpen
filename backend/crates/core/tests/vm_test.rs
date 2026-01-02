use worpen_core::compiler::symbol_table::SymbolTable;
use worpen_core::vm::memory::ExecutionMemory;
use worpen_core::vm::instructions::OptimizedOperation;
use serde_json::Value;

#[test]
fn test_symbol_table_and_memory() {
    // Create a SymbolTable
    let mut symbol_table = SymbolTable::new();

    // Register variables "x" and "y"
    let x_index = symbol_table.register("x".to_string());
    let y_index = symbol_table.register("y".to_string());

    // Assert indices are unique
    assert_eq!(x_index, 0);
    assert_eq!(y_index, 1);

    // Register "x" again, should return same index
    let x_index_again = symbol_table.register("x".to_string());
    assert_eq!(x_index_again, 0);

    // Check get_index
    assert_eq!(symbol_table.get_index("x"), Some(0));
    assert_eq!(symbol_table.get_index("y"), Some(1));
    assert_eq!(symbol_table.get_index("z"), None);

    // Check get_name
    assert_eq!(symbol_table.get_name(0), Some("x"));
    assert_eq!(symbol_table.get_name(1), Some("y"));
    assert_eq!(symbol_table.get_name(2), None);

    // Create ExecutionMemory
    let mut memory = ExecutionMemory::new();

    // Store values using the indices
    memory.set(x_index, Value::Number(42.into()));
    memory.set(y_index, Value::String("hello".to_string()));

    // Retrieve values and assert correctness
    assert_eq!(memory.get(x_index), Some(&Value::Number(42.into())));
    assert_eq!(memory.get(y_index), Some(&Value::String("hello".to_string())));

    // Check non-existent index
    assert_eq!(memory.get(2), None);
}

#[test]
fn test_logic_compiler() {
    use worpen_core::compiler::lowerer::LogicCompiler;
    use proto::models::LogicOperation;
    use serde_json::Value;

    let mut compiler = LogicCompiler::new();

    // Sample logic: Set x=10, Set y=20, MathOp add x and y
    let logic = vec![
        LogicOperation::Set { var: "x".to_string(), value: Value::Number(10.into()) },
        LogicOperation::Set { var: "y".to_string(), value: Value::Number(20.into()) },
        LogicOperation::MathOp { operation: "add".to_string(), args: vec![Value::String("{{x}}".to_string()), Value::String("{{y}}".to_string())] },
    ];

    let optimized = compiler.compile(&logic);

    // Check symbol table
    let symbol_table = compiler.get_symbol_table();
    assert_eq!(symbol_table.get_index("x"), Some(0));
    assert_eq!(symbol_table.get_index("y"), Some(1));

    // Check optimized operations
    assert_eq!(optimized.len(), 3);
    match &optimized[0] {
        OptimizedOperation::Set { var_index, value } => {
            assert_eq!(*var_index, 0);
            assert_eq!(value, &Value::Number(10.into()));
        }
        _ => panic!("Expected Set"),
    }
    match &optimized[1] {
        OptimizedOperation::Set { var_index, value } => {
            assert_eq!(*var_index, 1);
            assert_eq!(value, &Value::Number(20.into()));
        }
        _ => panic!("Expected Set"),
    }
    match &optimized[2] {
        OptimizedOperation::MathOp { operation, args } => {
            assert_eq!(operation, "add");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], Value::String("{{x}}".to_string()));
            assert_eq!(args[1], Value::String("{{y}}".to_string()));
        }
        _ => panic!("Expected MathOp"),
    }
}

#[test]
fn test_vm_execution() {
    use worpen_core::compiler::lowerer::LogicCompiler;
    use worpen_core::vm::machine::VirtualMachine;
    use proto::models::LogicOperation;
    use serde_json::Value;
    use tokio::runtime::Runtime;

    // Create logic: Set a=5, Set b=10, Return "{{a}}"
    let logic = vec![
        LogicOperation::Set { var: "a".to_string(), value: Value::Number(5.into()) },
        LogicOperation::Set { var: "b".to_string(), value: Value::Number(10.into()) },
        LogicOperation::Return { value: Value::String("{{a}}".to_string()) },
    ];

    // Compile
    let mut compiler = LogicCompiler::new();
    let optimized = compiler.compile(&logic);
    let symbol_table = compiler.get_symbol_table();

    // Create VM
    let memory = ExecutionMemory::new();
    let mut vm = VirtualMachine::new(memory, symbol_table.clone());

    // Execute
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(vm.execute(&optimized)).unwrap();

    // Assert
    assert_eq!(result, Value::Number(5.into()));
}