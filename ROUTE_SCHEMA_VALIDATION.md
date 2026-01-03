# Route Editor Schema Validation

## 🎯 Overview

The Route Editor now includes **real-time JSON Schema validation** powered by Monaco Editor. This prevents syntax errors before they reach the server, providing instant feedback with red squiggly lines under invalid operations.

## ✨ Features

### 1. **Automatic Validation**
- Red squiggly lines appear under invalid operations
- Hover tooltips show detailed error messages
- Validates operation names, required fields, and data types

### 2. **Smart Error Detection**
```yaml
logic:
  - print: "Hello"  # ❌ Error: Unknown operation "print". Did you mean "log"?
    
  - log:           # ✅ Correct
      message: "Hello"
```

### 3. **Required Field Validation**
```yaml
logic:
  - http_request:    # ❌ Error: Missing required field "url"
      method: GET
      
  - http_request:    # ✅ Correct
      url: "https://api.example.com"
      method: GET
```

### 4. **Type Checking**
```yaml
logic:
  - set:
      var: age
      value: "25"    # ✅ Valid (strings work for templates)
      
  - return:
      value: response
      status: "200"   # ❌ Error: "status" must be a number, not string
      
  - return:
      value: response
      status: 200     # ✅ Correct
```

## 📝 Supported Operations

### Core Operations
- ✅ `comment` - Documentation comments (NEW!)
- ✅ `return` - Return response (with status, headers, raw)
- ✅ `set` - Set variable
- ✅ `get` - Get variable
- ✅ `log` - Log message

### Control Flow
- ✅ `if` - Conditional logic
- ✅ `loop` - Iterate over collection
- ✅ `while` - While loop
- ✅ `switch` - Switch statement
- ✅ `break` - Break loop
- ✅ `continue` - Continue loop

### Data Operations
- ✅ `http_request` - HTTP requests
- ✅ `query_db` - Database queries (legacy)
- ✅ `sql_op` - SQL operations
- ✅ `redis_op` - Redis operations
- ✅ `ws_op` - WebSocket operations

### Transformations
- ✅ `math_op` - Math operations
- ✅ `string_op` - String manipulation
- ✅ `date_op` - Date operations
- ✅ `json_op` - JSON operations
- ✅ `map` - Map transformation
- ✅ `filter` - Filter collection
- ✅ `aggregate` - Aggregate data

### Advanced
- ✅ `try` - Error handling
- ✅ `throw` - Throw error
- ✅ `parallel` - Parallel execution
- ✅ `define_function` - Define function
- ✅ `call_function` - Call function
- ✅ `execute_script` - Execute script
- ✅ `sleep` - Delay execution

## 🔧 Schema Structure

The schema is defined in `src/utils/worpenSchema.ts` and follows JSON Schema Draft-07 specification.

### Example: HTTP Request Operation

```typescript
{
  "type": "object",
  "required": ["http_request"],
  "properties": {
    "http_request": {
      "type": "object",
      "required": ["url", "method"],
      "properties": {
        "url": {
          "type": "string",
          "description": "Request URL (supports templates)"
        },
        "method": {
          "type": "string",
          "enum": ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"]
        },
        "body": {},
        "headers": {
          "type": "object",
          "additionalProperties": { "type": "string" }
        },
        "timeout_ms": {
          "type": "integer",
          "minimum": 1
        }
      }
    }
  }
}
```

## 🎨 Visual Indicators

### Error (Red Squiggly)
```yaml
logic:
  - unknown_op:    # Red underline
      value: test
```

### Warning (Yellow Squiggly)
```yaml
logic:
  - return:
      value: result
      # Warning: Consider adding status code
```

### Info (Blue Underline)
```yaml
logic:
  - comment: "This is optional"  # Info: Comments are ignored at runtime
```

## 🚀 Usage Examples

### Valid Route Definition
```yaml
name: User Profile API
description: Get user profile with validation
path: /api/users/:id
method: GET
enabled: true
version: 1.0.0
auth_required: true

logic:
  - comment: "Validate user ID parameter"
  
  - set:
      var: user_id
      value: "{{params.id}}"
  
  - if:
      condition: user_id == ""
      then:
        - return:
            value:
              error: "User ID is required"
            status: 400
            headers:
              Content-Type: "application/json"
  
  - comment: "Fetch user from database"
  
  - sql_op:
      query: "SELECT * FROM users WHERE id = ?"
      args: ["{{user_id}}"]
      output_var: user
  
  - comment: "Return user data"
  
  - return:
      value: "{{user}}"
      status: 200
      headers:
        Content-Type: "application/json"
        Cache-Control: "max-age=300"
```

### Common Errors & Fixes

#### ❌ Wrong Operation Name
```yaml
logic:
  - print: "Hello"   # Error: Unknown operation
```
✅ **Fix:**
```yaml
logic:
  - log:
      message: "Hello"
```

#### ❌ Missing Required Field
```yaml
logic:
  - http_request:
      method: GET      # Error: Missing "url"
```
✅ **Fix:**
```yaml
logic:
  - http_request:
      url: "https://api.example.com"
      method: GET
```

#### ❌ Wrong Type
```yaml
logic:
  - sleep:
      duration_ms: "1000"  # Error: Must be number
```
✅ **Fix:**
```yaml
logic:
  - sleep:
      duration_ms: 1000
```

#### ❌ Invalid Enum Value
```yaml
logic:
  - http_request:
      url: "https://api.example.com"
      method: FETCH    # Error: Invalid method
```
✅ **Fix:**
```yaml
logic:
  - http_request:
      url: "https://api.example.com"
      method: GET      # Valid: GET, POST, PUT, DELETE, PATCH, HEAD
```

## 🔍 Advanced Validation

### Custom Error Messages
The editor provides context-aware error messages:

```yaml
logic:
  - return:
      value: result
      status: 999      # Error: status must be between 100-599
```

### Template Validation
Templates are validated as strings:

```yaml
logic:
  - set:
      var: name
      value: "{{user.name}}"   # ✅ Valid template
      
  - set:
      var: age
      value: {{user.age}}      # ❌ YAML syntax error (quotes needed)
```

## 📚 Integration Details

### Monaco Editor Configuration

The schema is configured in `EditorPanel.tsx`:

```typescript
monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
    validate: true,
    allowComments: false,
    schemas: [{
        uri: 'http://worpen.io/schemas/route.json',
        fileMatch: ['*'],
        schema: WORPEN_ROUTE_SCHEMA
    }],
    enableSchemaRequest: false,
    schemaValidation: 'error'
});
```

### Custom Validators

Additional validation logic checks for:
- Common typos (e.g., "print" instead of "log")
- Missing required route fields
- YAML syntax errors

## 🎯 Benefits

1. **Catch Errors Early** - Before sending to server
2. **Better Developer Experience** - Instant feedback
3. **Self-Documenting** - Hover tooltips explain each field
4. **Autocomplete** - IntelliSense for operation names
5. **Consistency** - Ensures all routes follow the same structure

## 🔄 Future Enhancements

- [ ] YAML-specific schema validation
- [ ] Template syntax validation (e.g., `{{var}}`)
- [ ] Expression validation (e.g., `age >= 18`)
- [ ] Cross-field validation (e.g., conditional requirements)
- [ ] Performance optimization for large routes
- [ ] Custom snippets for common patterns

## 🐛 Troubleshooting

### Schema Not Working?
1. **Check Console** - Look for "[EditorPanel] Monaco schema configured"
2. **Refresh Editor** - Try closing and reopening the route
3. **Clear Cache** - Browser might cache old schema

### False Positives?
1. **Check YAML Syntax** - Ensure proper indentation
2. **Quotes** - Some values need quotes in YAML
3. **Report Issue** - If validation seems wrong, let us know!

## 📝 Related Files

- `src/utils/worpenSchema.ts` - Schema definition
- `components/RouteBuilder/EditorPanel.tsx` - Integration
- `backend/crates/proto/src/models/routes.rs` - Rust types
- `COMMENTS_AND_RAW_RESPONSES.md` - New features guide

## 🎉 Conclusion

With JSON Schema validation, writing routes is now safer and faster. The editor catches mistakes before they cause runtime errors, making development more pleasant and productive!

Happy coding! 🚀
