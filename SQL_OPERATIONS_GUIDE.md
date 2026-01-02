# 💾 SQL Operations Guide - VM-Optimized Database Access

## Overview

The `sql_op` operation provides native, VM-optimized SQL query execution with full variable resolution and parameterized queries for security.

## ⚡ Features

- ✅ **VM-Optimized Execution**: Compiled to bytecode with 3-5µs overhead
- ✅ **Parameterized Queries**: SQL injection safe by design
- ✅ **Variable Interpolation**: Seamless integration with execution context
- ✅ **Direct Memory Access**: O(1) variable lookup via integer indexing
- ✅ **Type Conversion**: Automatic conversion of SQL types to JSON
- ✅ **Named Output**: Store results in named variables for reuse

## 🔧 Syntax

```yaml
sql_op:
  query: "SELECT * FROM table WHERE column = ?"
  args: ["{{variable_name}}"]
  output_var: "result_variable"
```

### Parameters

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String | ✅ Yes | SQL query with `?` placeholders |
| `args` | Array | ✅ Yes | Array of values/variables to bind to `?` placeholders |
| `output_var` | String | ✅ Yes | Variable name to store query results |

## 📊 Result Format

Results are returned as a JSON array of objects:

```json
[
  {
    "id": 1,
    "name": "Alice",
    "age": 30
  },
  {
    "id": 2,
    "name": "Bob",
    "age": 25
  }
]
```

Empty results return an empty array: `[]`

## 🎯 Type Conversions

| SQL Type | JSON Type | Example |
|----------|-----------|---------|
| INTEGER | Number | `42` |
| REAL | Number | `3.14` |
| TEXT | String | `"hello"` |
| BLOB | String (base64) | `"SGVsbG8="` |
| NULL | null | `null` |
| BOOLEAN | Boolean | `true` / `false` |

## 📝 Examples

### Example 1: Simple SELECT

```yaml
logic:
  - set:
      var: user_id
      value: 123
  
  - sql_op:
      query: "SELECT id, name, email FROM users WHERE id = ?"
      args: ["{{user_id}}"]
      output_var: user_data
  
  - return:
      value: "{{user_data}}"
```

**Result:**
```json
[
  {
    "id": 123,
    "name": "Alice",
    "email": "alice@example.com"
  }
]
```

### Example 2: INSERT with Multiple Parameters

```yaml
logic:
  - set:
      var: name
      value: "Bob"
  
  - set:
      var: email
      value: "bob@example.com"
  
  - set:
      var: age
      value: 30
  
  - sql_op:
      query: "INSERT INTO users (name, email, age) VALUES (?, ?, ?)"
      args: ["{{name}}", "{{email}}", "{{age}}"]
      output_var: insert_result
  
  - return:
      value:
        message: "User created"
        result: "{{insert_result}}"
```

### Example 3: UPDATE with Conditionals

```yaml
logic:
  - set:
      var: user_id
      value: "{{request.params.id}}"
  
  - set:
      var: new_status
      value: "{{request.payload.status}}"
  
  # Validate new status
  - if:
      condition: "new_status != 'active' && new_status != 'inactive'"
      then:
        - return:
            value:
              error: "Invalid status"
  
  # Update
  - sql_op:
      query: "UPDATE users SET status = ? WHERE id = ?"
      args: ["{{new_status}}", "{{user_id}}"]
      output_var: update_result
  
  # Get updated user
  - sql_op:
      query: "SELECT * FROM users WHERE id = ?"
      args: ["{{user_id}}"]
      output_var: updated_user
  
  - return:
      value:
        message: "Updated successfully"
        user: "{{updated_user[0]}}"
```

### Example 4: Complex JOIN Query

```yaml
logic:
  - set:
      var: min_age
      value: 18
  
  - sql_op:
      query: |
        SELECT 
          u.id,
          u.name,
          u.email,
          COUNT(o.id) as order_count,
          SUM(o.total) as total_spent
        FROM users u
        LEFT JOIN orders o ON u.id = o.user_id
        WHERE u.age >= ?
        GROUP BY u.id
        ORDER BY total_spent DESC
        LIMIT 10
      args: ["{{min_age}}"]
      output_var: top_customers
  
  - return:
      value:
        customers: "{{top_customers}}"
        count: "{{top_customers.length}}"
```

### Example 5: Transaction-like Pattern

```yaml
logic:
  # Step 1: Check if user exists
  - sql_op:
      query: "SELECT id FROM users WHERE email = ?"
      args: ["{{email}}"]
      output_var: existing_user
  
  - if:
      condition: "existing_user.length > 0"
      then:
        - return:
            value:
              error: "User already exists"
              status: 409
  
  # Step 2: Insert user
  - sql_op:
      query: "INSERT INTO users (name, email, age) VALUES (?, ?, ?)"
      args: ["{{name}}", "{{email}}", "{{age}}"]
      output_var: insert_result
  
  # Step 3: Create user profile
  - sql_op:
      query: "INSERT INTO profiles (user_id, bio) VALUES (last_insert_rowid(), ?)"
      args: ["{{bio}}"]
      output_var: profile_result
  
  # Step 4: Get complete user data
  - sql_op:
      query: |
        SELECT u.*, p.bio
        FROM users u
        JOIN profiles p ON u.id = p.user_id
        WHERE u.email = ?
      args: ["{{email}}"]
      output_var: complete_user
  
  - return:
      value:
        message: "User created with profile"
        user: "{{complete_user[0]}}"
```

## 🔒 Security Best Practices

### ✅ DO: Use Parameterized Queries

```yaml
# GOOD - SQL injection safe
sql_op:
  query: "SELECT * FROM users WHERE id = ?"
  args: ["{{user_id}}"]
  output_var: result
```

### ❌ DON'T: Concatenate User Input

```yaml
# BAD - Vulnerable to SQL injection
sql_op:
  query: "SELECT * FROM users WHERE id = {{user_id}}"
  args: []
  output_var: result
```

### ✅ DO: Validate Input Before Query

```yaml
logic:
  - set:
      var: user_id
      value: "{{request.params.id}}"
  
  # Validate user_id is numeric
  - if:
      condition: "isNaN(parseInt(user_id))"
      then:
        - return:
            value:
              error: "Invalid user ID"
  
  - sql_op:
      query: "SELECT * FROM users WHERE id = ?"
      args: ["{{user_id}}"]
      output_var: user
```

## 🚀 Performance Characteristics

| Operation | Time | Notes |
|-----------|------|-------|
| Variable Resolution | ~50ns | Direct integer indexing |
| Query Compilation | 0ns | Compiled at route registration |
| Query Execution | ~100µs - 10ms | Depends on query complexity |
| Result Conversion | ~10-50µs | JSON serialization |
| **Total Overhead** | **~3-5µs** | VM execution overhead only |

## 🆚 Comparison: sql_op vs query_db

| Feature | `sql_op` (NEW) | `query_db` (Legacy) |
|---------|----------------|---------------------|
| VM Optimization | ✅ Yes | ❌ No |
| Variable Interpolation | ✅ Native | ⚠️ Manual |
| Memory Access | ✅ O(1) indexed | ❌ O(n) HashMap |
| Output Storage | ✅ Named variable | ❌ Anonymous result |
| Parameterized Queries | ✅ Yes | ✅ Yes |
| Performance | 🚀 Fast | 🐌 Slower |

## 🧪 Testing

Use the test endpoint to validate your SQL operations:

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes/test \
  -H "Content-Type: application/json" \
  -d '{
    "route_id": "your-route-id",
    "test_payload": {
      "user_id": 123,
      "status": "active"
    }
  }'
```

## 📚 Advanced Patterns

### Pattern 1: Pagination

```yaml
logic:
  - set:
      var: page
      value: "{{request.query.page || 1}}"
  
  - set:
      var: per_page
      value: "{{request.query.per_page || 20}}"
  
  - math_op:
      operation: mul
      args: ["{{page}} - 1", "{{per_page}}"]
  
  - set:
      var: offset
      value: "{{math_result}}"
  
  - sql_op:
      query: "SELECT * FROM users LIMIT ? OFFSET ?"
      args: ["{{per_page}}", "{{offset}}"]
      output_var: users
  
  - sql_op:
      query: "SELECT COUNT(*) as total FROM users"
      args: []
      output_var: count
  
  - return:
      value:
        data: "{{users}}"
        page: "{{page}}"
        per_page: "{{per_page}}"
        total: "{{count[0].total}}"
```

### Pattern 2: Search with Filters

```yaml
logic:
  - set:
      var: search_term
      value: "%{{request.query.q}}%"
  
  - set:
      var: min_age
      value: "{{request.query.min_age || 0}}"
  
  - set:
      var: max_age
      value: "{{request.query.max_age || 999}}"
  
  - sql_op:
      query: |
        SELECT * FROM users
        WHERE (name LIKE ? OR email LIKE ?)
          AND age BETWEEN ? AND ?
        ORDER BY created_at DESC
      args: ["{{search_term}}", "{{search_term}}", "{{min_age}}", "{{max_age}}"]
      output_var: results
  
  - return:
      value:
        results: "{{results}}"
        count: "{{results.length}}"
```

### Pattern 3: Aggregations

```yaml
logic:
  - sql_op:
      query: |
        SELECT
          COUNT(*) as total_users,
          AVG(age) as avg_age,
          MIN(age) as min_age,
          MAX(age) as max_age
        FROM users
        WHERE status = ?
      args: ["active"]
      output_var: stats
  
  - return:
      value: "{{stats[0]}}"
```

## 🐛 Troubleshooting

### Error: "Database pool not available"

**Cause:** VM was not initialized with a database pool.

**Solution:** Ensure the route uses the VM execution path (routes are automatically compiled to VM bytecode).

### Error: "Variable at index X not set"

**Cause:** A variable referenced in `args` was not defined before the `sql_op`.

**Solution:** Add a `set` operation before the `sql_op` to define the variable.

### Empty Results When Data Exists

**Cause:** Incorrect parameter binding or query syntax.

**Solution:** 
1. Check that the number of `?` placeholders matches the number of args
2. Verify variable names are correct with `{{var_name}}`
3. Test the raw SQL query in a SQL client

## 📖 See Also

- [Dynamic Routes Complete Guide](DYNAMIC_ROUTES_COMPLETE_GUIDE.md)
- [Expression Guide](backend/EXPRESSION_GUIDE.md)
- [VM Architecture Documentation](documentation/)
