# 🏗️ Worpen Architecture: Zero-Touch Extensibility

## System Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         WORPEN DYNAMIC API ENGINE                           │
│                         Zero-Touch Extensibility System                      │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                          FRONTEND LAYER (React + TypeScript)                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  SETTINGS UI (Extensions Manager)                                  │    │
│  │                                                                     │    │
│  │  • User defines Custom Operation Schema (JSON Schema Draft 7)     │    │
│  │  • Schema stored in: localStorage['worpen_custom_schemas']        │    │
│  │  • Format: { operationName, schema, id, createdAt }               │    │
│  │                                                                     │    │
│  │  Example:                                                           │    │
│  │  {                                                                  │    │
│  │    "operationName": "NotifyOp",                                    │    │
│  │    "schema": {                                                      │    │
│  │      "type": "object",                                             │    │
│  │      "required": ["message"],                                      │    │
│  │      "properties": {                                               │    │
│  │        "message": { "type": "string" }                             │    │
│  │      }                                                              │    │
│  │    }                                                                │    │
│  │  }                                                                  │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                    ↓                                         │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  MONACO EDITOR (Route Builder)                                     │    │
│  │                                                                     │    │
│  │  • Loads schemas from localStorage on mount                        │    │
│  │  • Extends WORPEN_ROUTE_SCHEMA with custom operations             │    │
│  │  • Provides:                                                        │    │
│  │    - Autocomplete (Ctrl+Space)                                     │    │
│  │    - Type validation                                               │    │
│  │    - Inline documentation (hover)                                  │    │
│  │    - Error highlighting                                            │    │
│  │                                                                     │    │
│  │  • Storage event listener for cross-tab sync                       │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└──────────────────────────────────────┬───────────────────────────────────────┘
                                       │ HTTP POST /api/v1/dynamic-routes
                                       │ {
                                       │   "logic": [
                                       │     {
                                       │       "NotifyOp": {
                                       │         "message": "{{user_msg}}"
                                       │       }
                                       │     }
                                       │   ]
                                       │ }
                                       ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                        BACKEND LAYER (Rust + Axum)                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  DESERIALIZATION LAYER (serde_json)                                │    │
│  │                                                                     │    │
│  │  pub enum LogicOperation {                                         │    │
│  │      #[serde(rename = "log")]                                      │    │
│  │      Log { level: String, message: String },                       │    │
│  │                                                                     │    │
│  │      #[serde(rename = "sql_op")]                                   │    │
│  │      SqlOp { query: String, ... },                                 │    │
│  │                                                                     │    │
│  │      // 🎯 KEY FEATURE: Custom Operations Support                  │    │
│  │      #[serde(untagged)]                                            │    │
│  │      CustomOp(HashMap<String, Value>),  // ← Accepts ANYTHING!    │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  ✅ No deserialization errors for unknown operations!              │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                    ↓                                         │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  GENERIC WALKER (Compiler Service)                                 │    │
│  │                                                                     │    │
│  │  fn scan_logic_for_variables(operation: &LogicOperation) {        │    │
│  │      match operation {                                             │    │
│  │          // Known operations: Direct field access                  │    │
│  │          LogicOperation::Log { message, .. } => {                  │    │
│  │              scan_string(message);                                 │    │
│  │          }                                                          │    │
│  │                                                                     │    │
│  │          // 🚀 GENERIC FALLBACK: Works for ANY operation!          │    │
│  │          _ => {                                                     │    │
│  │              let json = serde_json::to_value(operation).unwrap(); │    │
│  │              scan_json_tree_for_variables(&json, vars);           │    │
│  │          }                                                          │    │
│  │      }                                                              │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  fn scan_json_tree_for_variables(value: &Value, vars: &mut Vec) { │    │
│  │      match value {                                                  │    │
│  │          Value::String(s) => {                                     │    │
│  │              // Find {{variable}} pattern                          │    │
│  │              for cap in REGEX.captures_iter(s) {                   │    │
│  │                  vars.push(cap[1].to_string());                    │    │
│  │              }                                                      │    │
│  │          }                                                          │    │
│  │          Value::Object(map) => {                                   │    │
│  │              // Recurse into nested structures                     │    │
│  │              for v in map.values() {                               │    │
│  │                  scan_json_tree_for_variables(v, vars);           │    │
│  │              }                                                      │    │
│  │          }                                                          │    │
│  │          Value::Array(arr) => { /* recurse */ }                    │    │
│  │          _ => {}                                                    │    │
│  │      }                                                              │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  ✅ Zero-Touch: No code changes needed for new operations!         │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                    ↓                                         │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  VARIABLE SCOPING (Inline & Scope Logic)                           │    │
│  │                                                                     │    │
│  │  Before Scoping:                                                    │    │
│  │  {                                                                  │    │
│  │    "NotifyOp": {                                                    │    │
│  │      "message": "Alert: {{user_msg}}"                              │    │
│  │    }                                                                │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  After Scoping (scope_id = 0):                                     │    │
│  │  {                                                                  │    │
│  │    "NotifyOp": {                                                    │    │
│  │      "message": "Alert: {{_0_user_msg}}"  // ← Scoped!            │    │
│  │    }                                                                │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  ✅ Variables automatically scoped in ALL string fields!           │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                    ↓                                         │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  COMPILATION (Lowering to OptimizedOperation)                      │    │
│  │                                                                     │    │
│  │  pub enum OptimizedOperation {                                     │    │
│  │      Log { level: String, message: String },                       │    │
│  │      SqlOp { query: String, arg_indices: Vec<usize>, ... },        │    │
│  │                                                                     │    │
│  │      // Custom operations preserved                                │    │
│  │      #[serde(untagged)]                                            │    │
│  │      CustomOp(HashMap<String, Value>),                             │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  fn lower(op: &LogicOperation) -> OptimizedOperation {            │    │
│  │      match op {                                                     │    │
│  │          LogicOperation::CustomOp(map) => {                        │    │
│  │              // Scan for variables in all values                   │    │
│  │              for value in map.values() {                           │    │
│  │                  register_variables_in_value(value);               │    │
│  │              }                                                      │    │
│  │              OptimizedOperation::CustomOp(map.clone())             │    │
│  │          }                                                          │    │
│  │          _ => { /* standard lowering */ }                          │    │
│  │      }                                                              │    │
│  │  }                                                                  │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                    ↓                                         │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  PERSISTENCE (Async File Write)                                    │    │
│  │                                                                     │    │
│  │  • Compiled route saved to: backend/data/routes/{route_id}.json   │    │
│  │  • Contains scoped variables and optimized operations              │    │
│  │  • No blocking on hot path                                         │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└──────────────────────────────────────┬───────────────────────────────────────┘
                                       │ Runtime Request
                                       │ POST /api/your-route
                                       ↓
┌─────────────────────────────────────────────────────────────────────────────┐
│                          EXECUTION LAYER (VM)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────┐    │
│  │  VIRTUAL MACHINE (Bytecode Executor)                               │    │
│  │                                                                     │    │
│  │  fn execute(&mut self, ops: Vec<OptimizedOperation>) {            │    │
│  │      for op in ops {                                               │    │
│  │          match op {                                                 │    │
│  │              OptimizedOperation::Log { .. } => {                   │    │
│  │                  // Direct execution                               │    │
│  │              }                                                      │    │
│  │              OptimizedOperation::SqlOp { .. } => {                 │    │
│  │                  // Database operation                             │    │
│  │              }                                                      │    │
│  │              OptimizedOperation::CustomOp(map) => {                │    │
│  │                  // Current: Debug output                          │    │
│  │                  // Future: Plugin system lookup & execution       │    │
│  │                  log::info!("CustomOp: {:?}", map.keys());         │    │
│  │              }                                                      │    │
│  │          }                                                          │    │
│  │      }                                                              │    │
│  │  }                                                                  │    │
│  │                                                                     │    │
│  │  ⚠️ Note: CustomOp currently returns debug info                    │    │
│  │  ✅ Variables are properly scoped and accessible                   │    │
│  └────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Data Flow Example: NotifyOp

### Step 1: User Defines Schema (Frontend)
```typescript
// User goes to Settings → Extensions
const schema = {
  operationName: "NotifyOp",
  schema: {
    type: "object",
    required: ["message"],
    properties: {
      message: { type: "string", description: "Notification message" }
    }
  }
};

// Saved to localStorage
localStorage.setItem('worpen_custom_schemas', JSON.stringify([schema]));
```

### Step 2: Monaco Editor Configuration
```typescript
// EditorPanel.tsx loads schemas
const customSchemas = JSON.parse(localStorage.getItem('worpen_custom_schemas') || '[]');

// Extends JSON Schema
const extendedSchema = {
  ...WORPEN_ROUTE_SCHEMA,
  definitions: {
    LogicOperation: {
      oneOf: [
        ...existingOperations,
        {
          type: "object",
          properties: {
            NotifyOp: {
              type: "object",
              required: ["message"],
              properties: {
                message: { type: "string" }
              }
            }
          }
        }
      ]
    }
  }
};

// Monaco provides autocomplete & validation
monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
  schemas: [{ schema: extendedSchema }]
});
```

### Step 3: User Creates Route
```json
{
  "name": "notify_route",
  "path": "/api/notify",
  "method": "POST",
  "logic": [
    { "set": { "var": "msg", "value": "{{request.body.text}}" } },
    {
      "NotifyOp": {
        "message": "Alert: {{msg}}"
      }
    }
  ]
}
```

### Step 4: Backend Deserialization
```rust
// routes.rs
#[derive(Deserialize)]
pub enum LogicOperation {
    #[serde(rename = "log")]
    Log { level: String, message: String },
    
    // This catches NotifyOp!
    #[serde(untagged)]
    CustomOp(HashMap<String, Value>),
}

// Deserialization succeeds:
LogicOperation::CustomOp({
    "NotifyOp": {
        "message": "Alert: {{msg}}"
    }
})
```

### Step 5: Generic Walker Scanning
```rust
// service.rs
fn scan_logic_for_variables(op: &LogicOperation) {
    match op {
        LogicOperation::CustomOp(map) => {
            // Serialize to JSON
            let json = serde_json::to_value(map).unwrap();
            
            // Recursive scan
            scan_json_tree_for_variables(&json, &mut variables);
            
            // Found: ["msg"]
        }
        _ => { /* known operations */ }
    }
}

fn scan_json_tree_for_variables(value: &Value, vars: &mut Vec<String>) {
    match value {
        Value::String(s) => {
            // Regex: r"\{\{([^}]+)\}\}"
            // Finds: "{{msg}}"
            vars.push("msg".to_string());
        }
        Value::Object(map) => {
            for v in map.values() {
                scan_json_tree_for_variables(v, vars); // Recurse
            }
        }
        _ => {}
    }
}
```

### Step 6: Variable Scoping
```rust
// service.rs
fn inline_scoped_logic(ops: &[LogicOperation], scope_id: u32) -> Vec<LogicOperation> {
    ops.iter().map(|op| {
        match op {
            LogicOperation::CustomOp(map) => {
                // Serialize
                let mut json = serde_json::to_value(map).unwrap();
                
                // Scope in-place
                scope_json_tree_references(&mut json, scope_id);
                
                // Deserialize back
                LogicOperation::CustomOp(serde_json::from_value(json).unwrap())
            }
            _ => { /* standard scoping */ }
        }
    }).collect()
}

// Result:
{
  "NotifyOp": {
    "message": "Alert: {{_0_msg}}"  // Scoped!
  }
}
```

### Step 7: Saved to Disk
```json
// backend/data/routes/{route_id}.json
{
  "id": "abc-123",
  "name": "notify_route",
  "path": "/api/notify",
  "logic": [
    { "set": { "var": "msg", "value": "{{request.body.text}}" } },
    {
      "NotifyOp": {
        "message": "Alert: {{_0_msg}}"
      }
    }
  ]
}
```

### Step 8: Runtime Execution
```rust
// VM execution
match optimized_op {
    OptimizedOperation::CustomOp(map) => {
        // Current: Debug info
        log::info!("Executing CustomOp: {:?}", map);
        
        // Variables are accessible via context
        let msg = context.get_variable("_0_msg");
        
        // Future: Plugin executor
        // let executor = plugin_registry.get(operation_name);
        // executor.execute(map, context)?;
    }
    _ => { /* standard execution */ }
}
```

---

## Key Features Enabled

### ✅ Zero-Touch Extensibility
- ✨ **No backend rebuild** needed for new operations
- ✨ **Generic JSON walker** handles any structure
- ✨ **Automatic variable scoping** in all string fields

### ✅ Developer Experience
- 🎨 **Monaco autocomplete** for custom operations
- 🎨 **Type validation** from JSON Schema
- 🎨 **Inline documentation** on hover

### ✅ Type Safety
- 🔒 **Frontend**: JSON Schema validation
- 🔒 **Backend**: Rust type system + generic fallback
- 🔒 **Runtime**: Variable scoping prevents collisions

### ✅ Performance
- ⚡ **O(1) localStorage** access
- ⚡ **Single-pass JSON tree** traversal
- ⚡ **No HashMap lookups** at runtime (integer indices)

---

## Future: Plugin System

```rust
// Planned architecture
pub trait OperationExecutor: Send + Sync {
    fn execute(&self, params: HashMap<String, Value>, context: &mut Context) -> Result<Value>;
}

pub struct PluginRegistry {
    executors: HashMap<String, Box<dyn OperationExecutor>>,
}

// User-defined plugin
pub struct NotifyOpExecutor;

impl OperationExecutor for NotifyOpExecutor {
    fn execute(&self, params: HashMap<String, Value>, ctx: &mut Context) -> Result<Value> {
        let message = params.get("message").unwrap();
        
        // Send actual notification
        notification_service.send(message)?;
        
        Ok(json!({"status": "sent"}))
    }
}

// Registration
plugin_registry.register("NotifyOp", Box::new(NotifyOpExecutor));
```

---

**Architecture Version:** 2.0  
**Last Updated:** 2026-01-03  
**Status:** Generic Walker ✅ | CustomOp Execution ⏳ (Future)
