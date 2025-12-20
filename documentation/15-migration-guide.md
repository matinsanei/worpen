# Migration Guide: JSON to YAML

Complete guide for migrating from JSON-based dynamic routes to YAML format.

## Table of Contents

1. [Why Migrate to YAML?](#why-migrate-to-yaml)
2. [Key Differences](#key-differences)
3. [Step-by-Step Migration](#step-by-step-migration)
4. [Conversion Examples](#conversion-examples)
5. [Common Patterns](#common-patterns)
6. [Automated Migration](#automated-migration)
7. [Testing Your Migration](#testing-your-migration)
8. [Troubleshooting](#troubleshooting)

---

## Why Migrate to YAML?

### Benefits of YAML

✅ **More Readable** - Less syntax noise, cleaner structure  
✅ **Easier to Write** - No commas, fewer quotes, natural indentation  
✅ **Better for Humans** - More intuitive and maintainable  
✅ **Same Performance** - No runtime overhead (3-5µs parsing)  
✅ **Full Compatibility** - All JSON features work in YAML  

### When to Migrate

- ✅ New projects - start with YAML
- ✅ Complex routes - YAML is easier to read
- ✅ Team collaboration - YAML reduces errors
- ⚠️ Existing JSON works fine - no urgent need to migrate
- ⚠️ Generated routes - JSON might be easier programmatically

---

## Key Differences

### Syntax Comparison

| Feature | JSON | YAML |
|---------|------|------|
| **Quotes** | Required | Optional for most strings |
| **Commas** | Required | Not used |
| **Braces** | `{}` for objects | Indentation only |
| **Brackets** | `[]` for arrays | Dash `-` or brackets |
| **Comments** | Not supported | `#` for comments |
| **Multiline** | Escape `\n` | `|` or `>` blocks |

### JSON Example

```json
{
  "logic": [
    {
      "variable": "user_id",
      "value": "{{ request.params.id }}"
    },
    {
      "sql": {
        "query": "SELECT * FROM users WHERE id = :user_id",
        "params": {
          "user_id": "{{ user_id }}"
        }
      },
      "assign": "user"
    }
  ],
  "response": {
    "status": 200,
    "body": "{{ user }}"
  }
}
```

### YAML Equivalent

```yaml
logic:
  - variable: user_id
    value: "{{ request.params.id }}"
  
  - sql:
      query: SELECT * FROM users WHERE id = :user_id
      params:
        user_id: "{{ user_id }}"
    assign: user

response:
  status: 200
  body: "{{ user }}"
```

---

## Step-by-Step Migration

### Step 1: Backup Your Routes

```powershell
# Backup all JSON routes
Copy-Item routes/*.json routes/backup/
```

### Step 2: Convert Basic Structure

**JSON:**
```json
{
  "logic": [],
  "response": {}
}
```

**YAML:**
```yaml
logic:
  - # steps here

response:
  # response config
```

### Step 3: Convert Variables

**JSON:**
```json
{
  "variable": "name",
  "value": "{{ request.body.name }}"
}
```

**YAML:**
```yaml
- variable: name
  value: "{{ request.body.name }}"
```

### Step 4: Convert SQL Queries

**JSON:**
```json
{
  "sql": {
    "query": "SELECT * FROM users",
    "params": {}
  },
  "assign": "users"
}
```

**YAML:**
```yaml
- sql:
    query: SELECT * FROM users
    params: {}
  assign: users
```

### Step 5: Convert HTTP Requests

**JSON:**
```json
{
  "http": {
    "method": "POST",
    "url": "https://api.example.com/users",
    "body": {
      "name": "{{ name }}"
    }
  },
  "assign": "api_response"
}
```

**YAML:**
```yaml
- http:
    method: POST
    url: https://api.example.com/users
    body:
      name: "{{ name }}"
  assign: api_response
```

### Step 6: Convert Control Flow

**JSON:**
```json
{
  "if": "{{ age >= 18 }}",
  "then": [
    {
      "variable": "status",
      "value": "adult"
    }
  ],
  "else": [
    {
      "variable": "status",
      "value": "minor"
    }
  ]
}
```

**YAML:**
```yaml
- if: "{{ age >= 18 }}"
  then:
    - variable: status
      value: adult
  else:
    - variable: status
      value: minor
```

### Step 7: Convert Loops

**JSON:**
```json
{
  "for_each": "{{ items }}",
  "as": "item",
  "do": [
    {
      "variable": "item_name",
      "value": "{{ item.name }}"
    }
  ]
}
```

**YAML:**
```yaml
- for_each: "{{ items }}"
  as: item
  do:
    - variable: item_name
      value: "{{ item.name }}"
```

### Step 8: Test the Route

```powershell
# Test YAML route
curl -X POST http://localhost:3000/api/test -H "Content-Type: application/json" -d '{...}'
```

---

## Conversion Examples

### Example 1: Simple GET Endpoint

**JSON:**
```json
{
  "path": "/users/:id",
  "method": "GET",
  "logic": [
    {
      "variable": "user_id",
      "value": "{{ request.params.id }}"
    },
    {
      "sql": {
        "query": "SELECT id, name, email FROM users WHERE id = :user_id",
        "params": {
          "user_id": "{{ user_id }}"
        }
      },
      "assign": "user"
    },
    {
      "if": "{{ user == null }}",
      "then": [
        {
          "return": {
            "status": 404,
            "body": {
              "error": "User not found"
            }
          }
        }
      ]
    }
  ],
  "response": {
    "status": 200,
    "body": "{{ user }}"
  }
}
```

**YAML:**
```yaml
path: /users/:id
method: GET

logic:
  - variable: user_id
    value: "{{ request.params.id }}"
  
  - sql:
      query: SELECT id, name, email FROM users WHERE id = :user_id
      params:
        user_id: "{{ user_id }}"
    assign: user
  
  - if: "{{ user == null }}"
    then:
      - return:
          status: 404
          body:
            error: User not found

response:
  status: 200
  body: "{{ user }}"
```

**Changes:**
- ✅ Removed all commas
- ✅ Removed braces for objects
- ✅ Removed quotes from query (multiline safe)
- ✅ Cleaner structure with blank lines

---

### Example 2: POST with Validation

**JSON:**
```json
{
  "path": "/users",
  "method": "POST",
  "logic": [
    {
      "variable": "name",
      "value": "{{ request.body.name }}"
    },
    {
      "variable": "email",
      "value": "{{ request.body.email | lower | trim }}"
    },
    {
      "if": "{{ !is_email(email) }}",
      "then": [
        {
          "return": {
            "status": 400,
            "body": {
              "error": "Invalid email format"
            }
          }
        }
      ]
    },
    {
      "sql": {
        "query": "INSERT INTO users (name, email) VALUES (:name, :email) RETURNING *",
        "params": {
          "name": "{{ name }}",
          "email": "{{ email }}"
        }
      },
      "assign": "new_user"
    }
  ],
  "response": {
    "status": 201,
    "body": "{{ new_user }}"
  }
}
```

**YAML:**
```yaml
path: /users
method: POST

logic:
  - variable: name
    value: "{{ request.body.name }}"
  
  - variable: email
    value: "{{ request.body.email | lower | trim }}"
  
  - if: "{{ !is_email(email) }}"
    then:
      - return:
          status: 400
          body:
            error: Invalid email format
  
  - sql:
      query: |
        INSERT INTO users (name, email) 
        VALUES (:name, :email) 
        RETURNING *
      params:
        name: "{{ name }}"
        email: "{{ email }}"
    assign: new_user

response:
  status: 201
  body: "{{ new_user }}"
```

**Changes:**
- ✅ Multiline SQL query using `|`
- ✅ Better formatting for readability
- ✅ Cleaner validation logic

---

### Example 3: Complex Workflow with Loops

**JSON:**
```json
{
  "path": "/orders/:id/process",
  "method": "POST",
  "logic": [
    {
      "variable": "order_id",
      "value": "{{ request.params.id }}"
    },
    {
      "sql": {
        "query": "SELECT * FROM orders WHERE id = :order_id",
        "params": {
          "order_id": "{{ order_id }}"
        }
      },
      "assign": "order"
    },
    {
      "if": "{{ order.status != 'pending' }}",
      "then": [
        {
          "return": {
            "status": 400,
            "body": {
              "error": "Order already processed"
            }
          }
        }
      ]
    },
    {
      "sql": {
        "query": "SELECT * FROM order_items WHERE order_id = :order_id",
        "params": {
          "order_id": "{{ order_id }}"
        }
      },
      "assign": "items"
    },
    {
      "variable": "total",
      "value": 0
    },
    {
      "for_each": "{{ items }}",
      "as": "item",
      "do": [
        {
          "variable": "item_total",
          "value": "{{ item.price * item.quantity }}"
        },
        {
          "variable": "total",
          "value": "{{ total + item_total }}"
        }
      ]
    },
    {
      "sql": {
        "query": "UPDATE orders SET status = 'completed', total = :total WHERE id = :order_id",
        "params": {
          "total": "{{ total }}",
          "order_id": "{{ order_id }}"
        }
      }
    }
  ],
  "response": {
    "status": 200,
    "body": {
      "message": "Order processed",
      "total": "{{ total }}"
    }
  }
}
```

**YAML:**
```yaml
path: /orders/:id/process
method: POST

logic:
  - variable: order_id
    value: "{{ request.params.id }}"
  
  - sql:
      query: SELECT * FROM orders WHERE id = :order_id
      params:
        order_id: "{{ order_id }}"
    assign: order
  
  - if: "{{ order.status != 'pending' }}"
    then:
      - return:
          status: 400
          body:
            error: Order already processed
  
  - sql:
      query: SELECT * FROM order_items WHERE order_id = :order_id
      params:
        order_id: "{{ order_id }}"
    assign: items
  
  - variable: total
    value: 0
  
  - for_each: "{{ items }}"
    as: item
    do:
      - variable: item_total
        value: "{{ item.price * item.quantity }}"
      
      - variable: total
        value: "{{ total + item_total }}"
  
  - sql:
      query: |
        UPDATE orders 
        SET status = 'completed', total = :total 
        WHERE id = :order_id
      params:
        total: "{{ total }}"
        order_id: "{{ order_id }}"

response:
  status: 200
  body:
    message: Order processed
    total: "{{ total }}"
```

**Changes:**
- ✅ Much cleaner structure
- ✅ Loop logic more readable
- ✅ Multiline SQL updates
- ✅ Better visual hierarchy

---

## Common Patterns

### Pattern 1: Authentication Middleware

**JSON:**
```json
{
  "variable": "auth_token",
  "value": "{{ request.headers.authorization }}"
},
{
  "if": "{{ auth_token == null }}",
  "then": [
    {
      "return": {
        "status": 401,
        "body": {
          "error": "Unauthorized"
        }
      }
    }
  ]
}
```

**YAML:**
```yaml
- variable: auth_token
  value: "{{ request.headers.authorization }}"

- if: "{{ auth_token == null }}"
  then:
    - return:
        status: 401
        body:
          error: Unauthorized
```

---

### Pattern 2: Pagination

**JSON:**
```json
{
  "variable": "page",
  "value": "{{ request.query.page || 1 }}"
},
{
  "variable": "limit",
  "value": "{{ request.query.limit || 10 }}"
},
{
  "variable": "offset",
  "value": "{{ (page - 1) * limit }}"
},
{
  "sql": {
    "query": "SELECT * FROM items LIMIT :limit OFFSET :offset",
    "params": {
      "limit": "{{ limit }}",
      "offset": "{{ offset }}"
    }
  },
  "assign": "items"
}
```

**YAML:**
```yaml
- variable: page
  value: "{{ request.query.page || 1 }}"

- variable: limit
  value: "{{ request.query.limit || 10 }}"

- variable: offset
  value: "{{ (page - 1) * limit }}"

- sql:
    query: SELECT * FROM items LIMIT :limit OFFSET :offset
    params:
      limit: "{{ limit }}"
      offset: "{{ offset }}"
  assign: items
```

---

### Pattern 3: Error Handling

**JSON:**
```json
{
  "try": [
    {
      "http": {
        "method": "GET",
        "url": "https://api.external.com/data"
      },
      "assign": "api_data"
    }
  ],
  "catch": [
    {
      "variable": "error_message",
      "value": "{{ error }}"
    },
    {
      "return": {
        "status": 500,
        "body": {
          "error": "External API failed",
          "details": "{{ error_message }}"
        }
      }
    }
  ]
}
```

**YAML:**
```yaml
- try:
    - http:
        method: GET
        url: https://api.external.com/data
      assign: api_data
  
  catch:
    - variable: error_message
      value: "{{ error }}"
    
    - return:
        status: 500
        body:
          error: External API failed
          details: "{{ error_message }}"
```

---

### Pattern 4: Data Transformation

**JSON:**
```json
{
  "variable": "user_emails",
  "value": "{{ users | map('email') | join(',') }}"
},
{
  "variable": "total_price",
  "value": "{{ items | sum('price') }}"
},
{
  "variable": "active_users",
  "value": "{{ users | filter('status == \"active\"') | length }}"
}
```

**YAML:**
```yaml
- variable: user_emails
  value: "{{ users | map('email') | join(',') }}"

- variable: total_price
  value: "{{ items | sum('price') }}"

- variable: active_users
  value: "{{ users | filter('status == \"active\"') | length }}"
```

---

## Automated Migration

### Using the CLI Tool (Day 18)

```powershell
# Convert single file
worpen convert routes/user.json routes/user.yaml

# Batch convert directory
worpen convert routes/*.json --output routes/yaml/

# Validate after conversion
worpen validate routes/yaml/*.yaml

# Preview diff before conversion
worpen convert routes/user.json --dry-run
```

### Manual Conversion Checklist

```yaml
✅ Remove all commas
✅ Replace {} with indentation
✅ Replace [] with - or keep []
✅ Remove quotes from keys
✅ Add # comments for clarity
✅ Use | for multiline strings
✅ Format for readability (blank lines)
✅ Test the route
✅ Verify response matches
```

---

## Testing Your Migration

### Step 1: Unit Test

```powershell
# Test route parsing
cargo test --test route_tests

# Test specific route
cargo test test_yaml_route
```

### Step 2: Integration Test

```powershell
# Start server
cargo run

# Test endpoint
curl -X GET http://localhost:3000/users/1

# Compare with JSON version
curl -X GET http://localhost:3000/users/1 > yaml_response.json
curl -X GET http://localhost:3000/users/1 > json_response.json
diff yaml_response.json json_response.json
```

### Step 3: Load Test

```powershell
# Benchmark performance (should be identical)
cargo bench

# Load test with wrk
wrk -t12 -c400 -d30s http://localhost:3000/users/1
```

---

## Troubleshooting

### Issue 1: Indentation Errors

**Error:**
```
YAML parse error: invalid indentation at line 10
```

**Solution:**
```yaml
# ❌ Bad - inconsistent indentation
logic:
  - variable: name
   value: "{{ name }}"

# ✅ Good - consistent 2 spaces
logic:
  - variable: name
    value: "{{ name }}"
```

---

### Issue 2: String Quoting

**Error:**
```
YAML parse error: unexpected character ':' at line 5
```

**Solution:**
```yaml
# ❌ Bad - unquoted string with special chars
value: http://example.com

# ✅ Good - quote strings with special chars
value: "http://example.com"

# ✅ Also good - use >
value: >
  http://example.com
```

---

### Issue 3: Array Syntax

**Error:**
```
YAML parse error: expected array but found object
```

**Solution:**
```yaml
# ❌ Bad - mixed syntax
logic: [
  - variable: name
]

# ✅ Good - consistent dash syntax
logic:
  - variable: name

# ✅ Also good - bracket syntax for simple arrays
tags: [admin, user, guest]
```

---

### Issue 4: Comments Breaking Logic

**Problem:**
```yaml
# This breaks because # in expression
if: "{{ value # comment }}"
```

**Solution:**
```yaml
# ✅ Comment outside expression
# Check if value exists
if: "{{ value != null }}"

# ✅ Or use multiline
if: >
  {{ value != null }}  # this works
```

---

## Migration Strategy

### Phase 1: New Routes (Week 1)

- ✅ All new routes in YAML
- ✅ Team training on YAML syntax
- ✅ Documentation updates

### Phase 2: Low-Risk Routes (Week 2-3)

- ✅ Convert read-only GET routes
- ✅ Convert simple POST routes
- ✅ Test in staging

### Phase 3: Critical Routes (Week 4)

- ✅ Convert complex workflows
- ✅ Convert authentication routes
- ✅ Extensive testing

### Phase 4: Deprecate JSON (Week 5)

- ✅ All routes migrated
- ✅ Remove JSON support (optional)
- ✅ Update documentation

---

## Best Practices

### 1. Migrate Incrementally

```yaml
# Don't migrate everything at once
# Start with simple routes first
# Test each route individually
```

### 2. Keep JSON Backup

```powershell
# Always backup before migration
Copy-Item routes/*.json routes/backup/
```

### 3. Use Version Control

```powershell
# Commit JSON version first
git add routes/*.json
git commit -m "Pre-migration backup"

# Then migrate
# Convert to YAML

# Commit YAML version
git add routes/*.yaml
git commit -m "Migrated to YAML"
```

### 4. Test Thoroughly

```yaml
✅ Unit tests pass
✅ Integration tests pass
✅ Response matches JSON version
✅ Performance is equivalent
✅ Error handling works
```

---

## Summary

Migration checklist:

✅ Understand key differences (syntax, structure)  
✅ Back up all JSON routes  
✅ Convert incrementally (simple → complex)  
✅ Test each route thoroughly  
✅ Use automated tools for batch conversion  
✅ Monitor performance (should be identical)  
✅ Update documentation and team knowledge  

Benefits achieved:

🎯 More readable and maintainable code  
🎯 Easier collaboration and reviews  
🎯 Better developer experience  
🎯 Same performance (3-5µs parsing)  
🎯 Full feature compatibility  

For more information:
- [YAML Syntax Guide](13-yaml-syntax.md)
- [Expression Reference](14-expressions.md)
- [Best Practices](16-best-practices.md)
