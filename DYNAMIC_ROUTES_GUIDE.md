# 🚀 Dynamic Route Engine

## Overview

The **Dynamic Route Engine** is a revolutionary feature that allows you to create, register, and execute custom API endpoints **at runtime** using JSON configuration. No code deployment needed!

## 🎯 Key Features

- **JSON-Driven Route Creation**: Define routes using simple JSON
- **Runtime Logic Execution**: Execute complex logic without code changes
- **Visual Testing Interface**: Test routes immediately with live results
- **Multiple Logic Operations**: Support for DB queries, HTTP requests, conditionals, loops, and more
- **Import/Export**: Share route configurations as JSON files
- **Hot Reload**: Routes are active immediately after registration

## 📋 Route Structure

```json
{
  "name": "My Custom API",
  "description": "Description of what this route does",
  "path": "/api/custom/my-endpoint",
  "method": "GET",  // GET, POST, PUT, DELETE, PATCH
  "logic": [
    // Array of operations to execute
  ],
  "enabled": true
}
```

## 🔧 Available Logic Operations

### 1. Return Value
```json
{
  "return": {
    "value": {
      "message": "Hello World",
      "status": "success"
    }
  }
}
```

### 2. Set Variable
```json
{
  "set": {
    "var": "userName",
    "value": "John Doe"
  }
}
```

### 3. Get Variable
```json
{
  "get": {
    "var": "userName"
  }
}
```

### 4. Query Database (Legacy)
```json
{
  "query_db": {
    "query": "SELECT * FROM agents WHERE status = ?",
    "params": ["ONLINE"]
  }
}
```

### 4b. SQL Operations (VM-Optimized) ⚡ NEW
Execute SQL with full VM optimization and variable resolution:

```json
{
  "sql_op": {
    "query": "SELECT * FROM users WHERE age > ? AND status = ?",
    "args": ["{{min_age}}", "{{user_status}}"],
    "output_var": "filtered_users"
  }
}
```

**Complete Example:**
```json
{
  "logic": [
    {
      "set": { "var": "min_age", "value": 18 }
    },
    {
      "set": { "var": "user_status", "value": "active" }
    },
    {
      "sql_op": {
        "query": "SELECT id, name, age FROM users WHERE age > ? AND status = ?",
        "args": ["{{min_age}}", "{{user_status}}"],
        "output_var": "users"
      }
    },
    {
      "return": { "value": "{{users}}" }
    }
  ]
}
```

**Benefits:**
- ✅ Parameterized queries (SQL injection safe)
- ✅ Variable interpolation
- ✅ VM-optimized execution
- ✅ Named output for result storage

### 4.5. Redis Operations (redis_op) ⚡ NEW
Execute Redis commands for high-performance caching and sessions:

```json
{
  "redis_op": {
    "command": "SET",
    "key": "user:{{user_id}}:token",
    "value": "{{auth_token}}",
    "ttl_seconds": 3600,
    "output_var": "cache_status"
  }
}
```

**Complete Caching Example:**
```json
{
  "logic": [
    {
      "set": { "var": "user_id", "value": 123 }
    },
    {
      "redis_op": {
        "command": "GET",
        "key": "user:{{user_id}}:profile",
        "output_var": "cached_profile"
      }
    },
    {
      "if": {
        "condition": "{{cached_profile}} != null",
        "then": [
          { "return": { "value": "{{cached_profile}}" } }
        ],
        "otherwise": [
          {
            "sql_op": {
              "query": "SELECT * FROM users WHERE id = ?",
              "args": ["{{user_id}}"],
              "output_var": "user_data"
            }
          },
          {
            "redis_op": {
              "command": "SET",
              "key": "user:{{user_id}}:profile",
              "value": "{{json_stringify(user_data)}}",
              "ttl_seconds": 300,
              "output_var": "cache_set"
            }
          },
          { "return": { "value": "{{user_data}}" } }
        ]
      }
    }
  ]
}
```

**Supported Commands:**
- `GET` - Retrieve value (returns string or null)
- `SET` - Store value with optional TTL (returns "OK")
- `DEL` - Delete key (returns count)
- `EXPIRE` - Set TTL on existing key (returns boolean)
- `INCR` - Increment counter atomically (returns new value)
- `DECR` - Decrement counter atomically (returns new value)

**Benefits:**
- ✅ Sub-millisecond response times
- ✅ Connection pooling
- ✅ Variable interpolation in keys/values
- ✅ Automatic TTL management
- ✅ Atomic operations for counters

### 5. HTTP Request
```json
{
  "http_request": {
    "url": "https://api.example.com/data",
    "method": "GET",
    "body": null
  }
}
```

### 6. Conditional Logic
```json
{
  "if": {
    "condition": "status == active",
    "then": [
      {"return": {"value": {"message": "Active"}}}
    ],
    "otherwise": [
      {"return": {"value": {"message": "Inactive"}}}
    ]
  }
}
```

### 7. Loop Over Collection
```json
{
  "loop": {
    "collection": "users",
    "var": "user",
    "body": [
      {"return": {"value": "{{user}}"}}
    ]
  }
}
```

### 8. Map Transformation
```json
{
  "map": {
    "input": "numbers",
    "transform": "x * 2"
  }
}
```

### 9. Filter Collection
```json
{
  "filter": {
    "input": "users",
    "condition": "age > 18"
  }
}
```

### 10. Aggregate Data
```json
{
  "aggregate": {
    "input": "prices",
    "operation": "sum"  // sum, count, avg, min, max
  }
}
```

### 11. Execute Script
```json
{
  "execute_script": {
    "language": "javascript",
    "code": "return 2 + 2;"
  }
}
```

## 📝 Complete Examples

### Example 1: Simple Echo API
```json
{
  "name": "Echo API",
  "description": "Returns whatever you send",
  "path": "/api/custom/echo",
  "method": "POST",
  "logic": [
    {
      "return": {
        "value": {
          "message": "Echo response",
          "received": "{{request.body}}"
        }
      }
    }
  ],
  "enabled": true
}
```

### Example 2: Database Query
```json
{
  "name": "Active Agents List",
  "description": "Get all online agents from database",
  "path": "/api/custom/active-agents",
  "method": "GET",
  "logic": [
    {
      "query_db": {
        "query": "SELECT * FROM agents WHERE status = 'Online' LIMIT 10",
        "params": []
      }
    },
    {
      "return": {
        "value": {
          "agents": "{{db_result}}",
          "count": "{{db_result.length}}"
        }
      }
    }
  ],
  "enabled": true
}
```

### Example 3: Conditional Logic
```json
{
  "name": "System Status Check",
  "description": "Check if system is operational",
  "path": "/api/custom/health",
  "method": "GET",
  "logic": [
    {
      "query_db": {
        "query": "SELECT COUNT(*) as active_count FROM agents WHERE status = 'Online'",
        "params": []
      }
    },
    {
      "set": {
        "var": "activeCount",
        "value": "{{db_result[0].active_count}}"
      }
    },
    {
      "if": {
        "condition": "activeCount > 5",
        "then": [
          {
            "return": {
              "value": {
                "status": "HEALTHY",
                "active_nodes": "{{activeCount}}",
                "message": "System is operational"
              }
            }
          }
        ],
        "otherwise": [
          {
            "return": {
              "value": {
                "status": "WARNING",
                "active_nodes": "{{activeCount}}",
                "message": "Low node count"
              }
            }
          }
        ]
      }
    }
  ],
  "enabled": true
}
```

### Example 4: Multi-Step Processing
```json
{
  "name": "User Registration",
  "description": "Register new user with validation",
  "path": "/api/custom/register",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "email",
        "value": "{{request.body.email}}"
      }
    },
    {
      "query_db": {
        "query": "SELECT * FROM users WHERE email = ?",
        "params": ["{{email}}"]
      }
    },
    {
      "if": {
        "condition": "db_result.length > 0",
        "then": [
          {
            "return": {
              "value": {
                "error": "Email already exists",
                "code": 409
              }
            }
          }
        ],
        "otherwise": [
          {
            "query_db": {
              "query": "INSERT INTO users (email, name) VALUES (?, ?)",
              "params": ["{{email}}", "{{request.body.name}}"]
            }
          },
          {
            "return": {
              "value": {
                "success": true,
                "message": "User registered successfully"
              }
            }
          }
        ]
      }
    }
  ],
  "enabled": true
}
```

## 🧪 Testing Routes

1. Navigate to **Dynamic Routes** view in the UI
2. Select a registered route
3. Enter test payload in JSON format
4. Click **RUN_TEST**
5. View execution results, steps, and timing

## 🔄 API Endpoints

### Register Route
```
POST /api/v1/dynamic-routes
Content-Type: application/json

{...route definition...}
```

### List Routes
```
GET /api/v1/dynamic-routes
```

### Get Route Details
```
GET /api/v1/dynamic-routes/{id}
```

### Update Route
```
PUT /api/v1/dynamic-routes/{id}
Content-Type: application/json

{...updated route...}
```

### Delete Route
```
DELETE /api/v1/dynamic-routes/{id}
```

### Test Route
```
POST /api/v1/dynamic-routes/test
Content-Type: application/json

{
  "route_id": "...",
  "test_payload": {...},
  "test_params": {}
}
```

### Execute Route
```
POST /api/v1/dynamic-routes/{id}/execute
Content-Type: application/json

{...execution payload...}
```

### Export Route
```
GET /api/v1/dynamic-routes/{id}/export
```

### Import Route
```
POST /api/v1/dynamic-routes/import
Content-Type: text/plain

{...route JSON...}
```

### Get Statistics
```
GET /api/v1/dynamic-routes/stats
```

## 🎨 UI Features

- **Template Library**: Pre-built route templates for common use cases
- **JSON Editor**: Syntax-highlighted JSON editing
- **Live Testing**: Instant route testing with detailed results
- **Execution Steps**: See each logic operation as it executes
- **Timing**: Track execution performance
- **Import/Export**: Share routes as JSON files

## 🔐 Security Considerations

- Routes can be enabled/disabled individually
- Optional authentication requirement per route
- Optional rate limiting
- Input validation through parameters
- Execution sandboxing (coming soon)

## 📊 Use Cases

1. **Rapid Prototyping**: Create test endpoints without coding
2. **Data Aggregation**: Combine multiple data sources dynamically
3. **Webhook Handlers**: Process incoming webhooks with custom logic
4. **Report Generation**: Generate custom reports on-demand
5. **Data Transformation**: Transform data between systems
6. **Monitoring Endpoints**: Create custom health checks
7. **Integration Points**: Connect external services without deployment

## 🚧 Limitations (Current Version)

- Database operations are mocked (ready for real implementation)
- HTTP requests are mocked (ready for real implementation)
- Script execution is mocked (ready for sandboxed execution)
- No persistent storage yet (routes stored in memory)

## 🔮 Roadmap

- [ ] Database integration (SQLite query execution)
- [ ] Real HTTP request execution
- [ ] JavaScript/Lua script sandbox
- [ ] Route versioning and rollback
- [ ] Performance monitoring and analytics
- [ ] Webhook subscriptions
- [ ] Rate limiting implementation
- [ ] Authentication/Authorization hooks
- [ ] Route templates marketplace
- [ ] GraphQL support
- [x] ✅ **WebSocket route support** - See [WEBSOCKET_GUIDE.md](WEBSOCKET_GUIDE.md)

## 💡 Tips

1. Start with simple templates and modify them
2. Test routes before enabling them
3. Use descriptive names and paths
4. Keep logic operations simple and focused
5. Export routes as backup before major changes
6. Use conditionals for error handling
7. Set variables for reusable values

## 🐛 Troubleshooting

**Route not executing?**
- Check if route is enabled
- Verify JSON syntax is valid
- Test with simple payload first

**Logic not working as expected?**
- Review execution steps in test results
- Check variable names match
- Verify conditions are correct

**Can't register route?**
- Ensure path starts with `/`
- Check for duplicate paths
- Validate all required fields

---

**Built with ❤️ for Worpen - The Future of DevOps**
