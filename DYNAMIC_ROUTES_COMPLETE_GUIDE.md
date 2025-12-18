# 🚀 Dynamic Routes Engine - Complete Guide

## 📋 Table of Contents
- [Overview](#overview)
- [Operations Reference](#operations-reference)
- [Examples](#examples)
- [Advanced Features](#advanced-features)

---

## Overview

The Dynamic Routes Engine is now **completely unlimited**! Create any API logic with JSON - no code deployment needed.

### ✅ What's Now Possible:

| Feature | Status | Description |
|---------|--------|-------------|
| **Custom Functions** | ✅ | Define and call reusable functions |
| **Recursion** | ✅ | Functions can call themselves |
| **Try/Catch** | ✅ | Advanced error handling |
| **Switch/Case** | ✅ | Clean multi-condition logic |
| **While Loops** | ✅ | Condition-based iteration |
| **Break/Continue** | ✅ | Loop control flow |
| **Parallel Execution** | ✅ | Run operations concurrently |
| **Async HTTP** | ✅ | Non-blocking HTTP requests |
| **Helper Functions** | ✅ | String, Math, Date, JSON utilities |
| **Complex Data** | ✅ | Full JSON object support |

---

## Operations Reference

### 🎯 Control Flow

#### `if` - Conditional Logic
```json
{
  "if": {
    "condition": "age >= 18",
    "then": [
      { "return": { "value": "Adult" } }
    ],
    "otherwise": [
      { "return": { "value": "Minor" } }
    ]
  }
}
```

**Supported Operators:** `==`, `!=`, `>`, `<`, `>=`, `<=`

#### `switch` - Multi-Condition
```json
{
  "switch": {
    "value": "status",
    "cases": [
      {
        "value": "active",
        "operations": [
          { "return": { "value": { "message": "User is active" } } }
        ]
      },
      {
        "value": "suspended",
        "operations": [
          { "return": { "value": { "message": "User is suspended" } } }
        ]
      }
    ],
    "default": [
      { "return": { "value": { "message": "Unknown status" } } }
    ]
  }
}
```

#### `while` - Condition-Based Loop
```json
{
  "while": {
    "condition": "counter < 10",
    "max_iterations": 100,
    "body": [
      { "set": { "var": "counter", "value": "{{counter}} + 1" } },
      { "log": { "level": "info", "message": "Counter: {{counter}}" } }
    ]
  }
}
```

#### `loop` (for_each) - Iterate Over Collections
```json
{
  "loop": {
    "collection": "users",
    "var": "user",
    "body": [
      { "log": { "level": "info", "message": "Processing user: {{user.name}}" } }
    ]
  }
}
```

#### `break` - Exit Loop
```json
{
  "if": {
    "condition": "user.role == admin",
    "then": [
      { "break": {} }
    ]
  }
}
```

#### `continue` - Skip Iteration
```json
{
  "if": {
    "condition": "user.active == false",
    "then": [
      { "continue": {} }
    ]
  }
}
```

---

### ⚠️ Error Handling

#### `try` / `catch` / `finally`
```json
{
  "try": {
    "body": [
      { "query_db": { "query": "SELECT * FROM users", "params": [] } }
    ],
    "catch": [
      { "log": { "level": "error", "message": "DB Error: {{error.message}}" } },
      { "return": { "value": { "error": "Database unavailable" } } }
    ],
    "finally": [
      { "log": { "level": "info", "message": "Query attempt finished" } }
    ]
  }
}
```

#### `throw` - Raise Error
```json
{
  "throw": {
    "message": "Invalid user ID",
    "code": "INVALID_ID"
  }
}
```

---

### 🔄 Parallel Execution

#### `parallel` - Run Tasks Concurrently
```json
{
  "parallel": {
    "tasks": [
      [
        { "http_request": { "url": "https://api1.com/data", "method": "GET" } }
      ],
      [
        { "http_request": { "url": "https://api2.com/data", "method": "GET" } }
      ],
      [
        { "query_db": { "query": "SELECT COUNT(*) FROM users", "params": [] } }
      ]
    ],
    "max_concurrent": 3
  }
}
```

---

### 🎭 Custom Functions

#### `define_function` - Create Reusable Logic
```json
{
  "define_function": {
    "name": "calculate_tax",
    "params": ["amount", "rate"],
    "body": [
      { "math_op": { 
        "operation": "sum", 
        "args": ["{{amount}}", "{{amount}} * {{rate}}"] 
      } },
      { "return": { "value": "{{math_result}}" } }
    ]
  }
}
```

#### `call_function` - Invoke Custom Function
```json
{
  "call_function": {
    "name": "calculate_tax",
    "args": [100, 0.15]
  }
}
```

**Recursion Example:**
```json
{
  "define_function": {
    "name": "factorial",
    "params": ["n"],
    "body": [
      {
        "if": {
          "condition": "n <= 1",
          "then": [{ "return": { "value": 1 } }],
          "otherwise": [
            { "call_function": { "name": "factorial", "args": ["{{n}} - 1"] } },
            { "math_op": { "operation": "sum", "args": ["{{n}} * {{factorial_result}}"] } },
            { "return": { "value": "{{math_result}}" } }
          ]
        }
      }
    ]
  }
}
```

---

### 🌐 HTTP Requests

#### `http_request` - External API Calls
```json
{
  "http_request": {
    "url": "https://api.example.com/users/{{user_id}}",
    "method": "POST",
    "body": {
      "name": "{{user_name}}",
      "email": "{{user_email}}"
    },
    "headers": {
      "Authorization": "Bearer {{token}}",
      "Content-Type": "application/json"
    },
    "timeout_ms": 5000
  }
}
```

---

### 💾 Database Operations

#### `query_db` - Execute SQL
```json
{
  "query_db": {
    "query": "SELECT * FROM users WHERE age > ? AND status = ?",
    "params": [18, "active"]
  }
}
```

---

### 🔧 Helper Functions

#### `string_op` - String Manipulations
```json
{
  "string_op": {
    "operation": "split",
    "input": "john,jane,bob",
    "args": [","]
  }
}
```

**Operations:** `split`, `join`, `upper`, `lower`, `trim`, `replace`

#### `math_op` - Mathematical Operations
```json
{
  "math_op": {
    "operation": "sum",
    "args": [10, 20, 30, 40]
  }
}
```

**Operations:** `sum`, `avg`, `min`, `max`, `round`, `ceil`, `floor`, `abs`, `pow`, `sqrt`

#### `date_op` - Date/Time Operations
```json
{
  "date_op": {
    "operation": "now",
    "args": []
  }
}
```

**Operations:** `now`, `timestamp`, `format`

#### `json_op` - JSON Utilities
```json
{
  "json_op": {
    "operation": "get_path",
    "input": "user_data",
    "args": ["profile.address.city"]
  }
}
```

**Operations:** `stringify`, `parse`, `get_path`

---

### 📦 Variables

#### `set` - Store Value
```json
{
  "set": {
    "var": "total_price",
    "value": 125.50
  }
}
```

#### `get` - Retrieve Value
```json
{
  "get": {
    "var": "total_price"
  }
}
```

**Variable Interpolation:**
```json
{
  "log": {
    "level": "info",
    "message": "Total: {{total_price}}, Tax: {{tax_amount}}"
  }
}
```

---

### 🎯 Utilities

#### `log` - Debug Output
```json
{
  "log": {
    "level": "info",
    "message": "Processing user: {{user_name}}"
  }
}
```

**Levels:** `info`, `warn`, `error`, `debug`

#### `sleep` - Delay Execution
```json
{
  "sleep": {
    "duration_ms": 1000
  }
}
```

#### `return` - Exit with Result
```json
{
  "return": {
    "value": {
      "success": true,
      "data": "{{result}}"
    }
  }
}
```

---

## Examples

### 1. Multi-Step User Registration
```json
{
  "name": "User Registration API",
  "path": "/api/custom/register",
  "method": "POST",
  "logic": [
    {
      "try": {
        "body": [
          { "log": { "level": "info", "message": "Starting registration for {{email}}" } },
          
          { "query_db": {
            "query": "SELECT id FROM users WHERE email = ?",
            "params": ["{{email}}"]
          }},
          
          {
            "if": {
              "condition": "db_result.count > 0",
              "then": [
                { "throw": { "message": "Email already registered", "code": "DUPLICATE_EMAIL" } }
              ]
            }
          },
          
          { "string_op": { "operation": "lower", "input": "{{email}}", "args": [] } },
          { "set": { "var": "normalized_email", "value": "{{string_result}}" } },
          
          { "query_db": {
            "query": "INSERT INTO users (email, name, created_at) VALUES (?, ?, datetime('now'))",
            "params": ["{{normalized_email}}", "{{name}}"]
          }},
          
          { "http_request": {
            "url": "https://api.sendgrid.com/v3/mail/send",
            "method": "POST",
            "body": {
              "to": "{{normalized_email}}",
              "subject": "Welcome!",
              "text": "Welcome to our platform, {{name}}!"
            },
            "headers": {
              "Authorization": "Bearer {{sendgrid_api_key}}"
            }
          }},
          
          { "return": {
            "value": {
              "success": true,
              "message": "User registered successfully",
              "user_id": "{{db_result.last_insert_id}}"
            }
          }}
        ],
        "catch": [
          { "log": { "level": "error", "message": "Registration failed: {{error.message}}" } },
          { "return": {
            "value": {
              "success": false,
              "error": "{{error.message}}"
            }
          }}
        ]
      }
    }
  ]
}
```

### 2. Parallel Data Aggregation
```json
{
  "name": "Dashboard Stats API",
  "path": "/api/custom/dashboard",
  "method": "GET",
  "logic": [
    {
      "parallel": {
        "tasks": [
          [
            { "query_db": { "query": "SELECT COUNT(*) as count FROM users", "params": [] } },
            { "set": { "var": "user_count", "value": "{{db_result.count}}" } }
          ],
          [
            { "query_db": { "query": "SELECT COUNT(*) as count FROM orders WHERE status='pending'", "params": [] } },
            { "set": { "var": "pending_orders", "value": "{{db_result.count}}" } }
          ],
          [
            { "http_request": { "url": "https://api.stripe.com/v1/balance", "method": "GET" } },
            { "set": { "var": "stripe_balance", "value": "{{http_response.available[0].amount}}" } }
          ]
        ]
      }
    },
    
    { "return": {
      "value": {
        "users": "{{user_count}}",
        "pending_orders": "{{pending_orders}}",
        "balance": "{{stripe_balance}}"
      }
    }}
  ]
}
```

### 3. Recursive Fibonacci
```json
{
  "name": "Fibonacci API",
  "path": "/api/custom/fibonacci",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "fib",
        "params": ["n"],
        "body": [
          {
            "if": {
              "condition": "n <= 1",
              "then": [{ "return": { "value": "{{n}}" } }]
            }
          },
          { "call_function": { "name": "fib", "args": ["{{n}} - 1"] } },
          { "set": { "var": "fib_n_1", "value": "{{fib_result}}" } },
          { "call_function": { "name": "fib", "args": ["{{n}} - 2"] } },
          { "set": { "var": "fib_n_2", "value": "{{fib_result}}" } },
          { "math_op": { "operation": "sum", "args": ["{{fib_n_1}}", "{{fib_n_2}}"] } },
          { "return": { "value": "{{math_result}}" } }
        ]
      }
    },
    { "call_function": { "name": "fib", "args": ["{{n}}"] } },
    { "return": { "value": { "fibonacci": "{{fib_result}}" } } }
  ]
}
```

### 4. Complex E-commerce Workflow
```json
{
  "name": "Process Order API",
  "path": "/api/custom/orders/process",
  "method": "POST",
  "logic": [
    {
      "try": {
        "body": [
          { "query_db": {
            "query": "SELECT * FROM orders WHERE id = ?",
            "params": ["{{order_id}}"]
          }},
          
          { "set": { "var": "order", "value": "{{db_result.rows[0]}}" } },
          
          {
            "switch": {
              "value": "{{order.status}}",
              "cases": [
                {
                  "value": "pending",
                  "operations": [
                    { "http_request": {
                      "url": "https://api.stripe.com/v1/charges",
                      "method": "POST",
                      "body": {
                        "amount": "{{order.total}}",
                        "currency": "usd",
                        "source": "{{payment_token}}"
                      }
                    }},
                    
                    { "query_db": {
                      "query": "UPDATE orders SET status = 'paid' WHERE id = ?",
                      "params": ["{{order_id}}"]
                    }},
                    
                    { "parallel": {
                      "tasks": [
                        [
                          { "http_request": {
                            "url": "https://api.sendgrid.com/v3/mail/send",
                            "method": "POST",
                            "body": { "to": "{{order.customer_email}}", "subject": "Order Confirmed" }
                          }}
                        ],
                        [
                          { "http_request": {
                            "url": "https://api.warehouse.com/v1/fulfill",
                            "method": "POST",
                            "body": { "order_id": "{{order_id}}" }
                          }}
                        ]
                      ]
                    }},
                    
                    { "return": { "value": { "success": true, "message": "Order processed" } } }
                  ]
                },
                {
                  "value": "paid",
                  "operations": [
                    { "throw": { "message": "Order already paid", "code": "ALREADY_PAID" } }
                  ]
                }
              ],
              "default": [
                { "throw": { "message": "Invalid order status", "code": "INVALID_STATUS" } }
              ]
            }
          }
        ],
        "catch": [
          { "log": { "level": "error", "message": "Order processing failed: {{error.message}}" } },
          { "return": {
            "value": {
              "success": false,
              "error": "{{error.message}}",
              "code": "{{error.code}}"
            }
          }}
        ]
      }
    }
  ]
}
```

---

## Advanced Features

### 🔄 Recursive Functions
Functions can call themselves, enabling algorithms like factorial, fibonacci, tree traversal, etc.

### ⚡ Parallel Execution
Run multiple HTTP requests or DB queries concurrently for faster response times.

### 🎯 Complex Conditionals
Nest `if`, `switch`, and `while` statements infinitely for any business logic.

### 🛡️ Error Recovery
Use `try/catch/finally` to handle failures gracefully and maintain data consistency.

### 🧩 Modular Code
Define reusable functions once and call them multiple times with different arguments.

### 📊 Data Transformation
Built-in helpers for string manipulation, math operations, date formatting, and JSON parsing.

---

## 🎓 Best Practices

1. **Use Functions**: Break complex logic into reusable functions
2. **Error Handling**: Wrap risky operations in try/catch
3. **Parallel Execution**: Run independent operations concurrently
4. **Variable Names**: Use descriptive names like `user_email`, not `x`
5. **Logging**: Add debug logs for troubleshooting
6. **Validation**: Check inputs before processing
7. **Timeouts**: Set HTTP timeouts to prevent hanging requests

---

## 🚀 Performance Tips

- Use `parallel` for multiple HTTP requests
- Set `max_iterations` on `while` loops to prevent infinite loops
- Use DB indexes for faster queries
- Cache results in variables to avoid re-computation
- Use `switch` instead of nested `if` statements

---

## 📝 Notes

- All HTTP requests are **async** and non-blocking
- Variables are scoped to the execution context
- Functions can access parent scope variables
- Loop control (`break`/`continue`) only works inside loops
- Errors thrown in `catch` blocks will propagate up

---

**The Dynamic Routes Engine is now UNLIMITED! 🎉**
