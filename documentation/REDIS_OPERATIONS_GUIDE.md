# Redis Operations Guide

## Overview

Worpen provides native Redis operations through the `redis_op` instruction, enabling high-performance caching, session management, and real-time data operations with **zero overhead** through VM optimization.

## Key Features

- ✅ **VM-Optimized**: Direct bytecode execution with O(1) variable access
- ⚡ **High Performance**: Connection pooling with deadpool-redis
- 🔒 **Type Safe**: Strongly typed Redis commands
- 🎯 **Template Support**: Variable interpolation in keys and values using `{{variable}}`
- 📦 **Six Core Commands**: GET, SET, DEL, EXPIRE, INCR, DECR
- 🔄 **TTL Support**: Built-in expiration handling

## Configuration

### Environment Setup

```bash
# Default Redis connection
REDIS_URL=redis://127.0.0.1:6379

# With authentication
REDIS_URL=redis://:password@127.0.0.1:6379

# With database selection
REDIS_URL=redis://127.0.0.1:6379/1
```

### Connection Pooling

Worpen automatically manages Redis connections using `deadpool-redis` for optimal performance and reliability.

## Supported Commands

### 1. SET - Store a Value

Store a string value with optional TTL.

```yaml
logic:
  - redis_op:
      command: "SET"
      key: "user:123:token"
      value: "abc-def-ghi"
      output_var: "set_result"
```

**With TTL (seconds):**
```yaml
logic:
  - redis_op:
      command: "SET"
      key: "session:{{session_id}}"
      value: "{{user_data}}"
      ttl_seconds: 3600  # 1 hour
      output_var: "status"
```

**Output**: `"OK"` on success

---

### 2. GET - Retrieve a Value

Retrieve a string value by key.

```yaml
logic:
  - redis_op:
      command: "GET"
      key: "user:123:token"
      output_var: "token"
```

**Output**: String value or `null` if key doesn't exist

---

### 3. DEL - Delete a Key

Delete one or more keys.

```yaml
logic:
  - redis_op:
      command: "DEL"
      key: "user:123:token"
      output_var: "deleted_count"
```

**Output**: Number of keys deleted (integer)

---

### 4. EXPIRE - Set Key Expiration

Set or update TTL for an existing key.

```yaml
logic:
  - redis_op:
      command: "EXPIRE"
      key: "user:123:token"
      ttl_seconds: 1800  # 30 minutes
      output_var: "expire_result"
```

**Output**: `true` if TTL was set, `false` if key doesn't exist

---

### 5. INCR - Increment Counter

Atomically increment a numeric value.

```yaml
logic:
  - redis_op:
      command: "INCR"
      key: "page:views:{{page_id}}"
      output_var: "new_count"
```

**Output**: New value after increment (integer)

---

### 6. DECR - Decrement Counter

Atomically decrement a numeric value.

```yaml
logic:
  - redis_op:
      command: "DECR"
      key: "inventory:{{product_id}}"
      output_var: "remaining_stock"
```

**Output**: New value after decrement (integer)

---

## Variable Interpolation

Use `{{variable_name}}` syntax for dynamic keys and values:

```yaml
logic:
  - set:
      var: "user_id"
      value: 42
  - set:
      var: "user_email"
      value: "alice@example.com"
  
  # Dynamic key
  - redis_op:
      command: "SET"
      key: "user:{{user_id}}:email"
      value: "{{user_email}}"
      output_var: "status"
  
  # Dynamic retrieval
  - redis_op:
      command: "GET"
      key: "user:{{user_id}}:email"
      output_var: "cached_email"
```

## Real-World Examples

### Example 1: Session Management

```yaml
name: "session_manager"
path: "/api/session"
method: POST
logic:
  # Generate session ID
  - set:
      var: "session_id"
      value: "{{uuid()}}"
  
  # Store session data with 1 hour TTL
  - redis_op:
      command: "SET"
      key: "session:{{session_id}}"
      value: "{{request.body.user_data}}"
      ttl_seconds: 3600
      output_var: "status"
  
  # Return session ID
  - return:
      value:
        session_id: "{{session_id}}"
        expires_in: 3600
```

### Example 2: Rate Limiting

```yaml
name: "rate_limiter"
path: "/api/limited-endpoint"
method: GET
logic:
  - set:
      var: "client_ip"
      value: "{{request.headers.x-forwarded-for}}"
  
  # Increment request counter
  - redis_op:
      command: "INCR"
      key: "ratelimit:{{client_ip}}:{{date('YYYY-MM-DD-HH')}}"
      output_var: "request_count"
  
  # Set 1 hour expiration on first request
  - if:
      condition: "{{request_count}} == 1"
      then:
        - redis_op:
            command: "EXPIRE"
            key: "ratelimit:{{client_ip}}:{{date('YYYY-MM-DD-HH')}}"
            ttl_seconds: 3600
            output_var: "expire_status"
  
  # Check rate limit (100 requests per hour)
  - if:
      condition: "{{request_count}} > 100"
      then:
        - return:
            value:
              error: "Rate limit exceeded"
              retry_after: 3600
      otherwise:
        - return:
            value:
              success: true
              requests_remaining: "{{100 - request_count}}"
```

### Example 3: Caching Database Queries

```yaml
name: "cached_user_profile"
path: "/api/users/{{id}}"
method: GET
logic:
  - set:
      var: "user_id"
      value: "{{path_params.id}}"
  
  # Try to get from cache
  - redis_op:
      command: "GET"
      key: "user:{{user_id}}:profile"
      output_var: "cached_profile"
  
  # If cache hit, return it
  - if:
      condition: "{{cached_profile}} != null"
      then:
        - return:
            value: "{{cached_profile}}"
  
  # Cache miss - query database
  - sql_op:
      query: "SELECT * FROM users WHERE id = ?"
      args: ["{{user_id}}"]
      output_var: "user_data"
  
  # Store in cache for 5 minutes
  - redis_op:
      command: "SET"
      key: "user:{{user_id}}:profile"
      value: "{{json_stringify(user_data)}}"
      ttl_seconds: 300
      output_var: "cache_status"
  
  # Return user data
  - return:
      value: "{{user_data}}"
```

### Example 4: Distributed Counters

```yaml
name: "analytics_tracker"
path: "/api/track/{{event}}"
method: POST
logic:
  - set:
      var: "event_name"
      value: "{{path_params.event}}"
  
  - set:
      var: "today"
      value: "{{date('YYYY-MM-DD')}}"
  
  # Increment daily counter
  - redis_op:
      command: "INCR"
      key: "analytics:{{event_name}}:{{today}}"
      output_var: "daily_count"
  
  # Set expiration for 90 days
  - redis_op:
      command: "EXPIRE"
      key: "analytics:{{event_name}}:{{today}}"
      ttl_seconds: 7776000
      output_var: "expire_status"
  
  # Increment all-time counter
  - redis_op:
      command: "INCR"
      key: "analytics:{{event_name}}:total"
      output_var: "total_count"
  
  - return:
      value:
        event: "{{event_name}}"
        daily_count: "{{daily_count}}"
        total_count: "{{total_count}}"
```

### Example 5: Feature Flags

```yaml
name: "feature_flag_check"
path: "/api/features/{{flag_name}}/check"
method: GET
logic:
  - set:
      var: "flag"
      value: "{{path_params.flag_name}}"
  
  - set:
      var: "user_id"
      value: "{{query_params.user_id}}"
  
  # Check if feature is enabled globally
  - redis_op:
      command: "GET"
      key: "feature:{{flag}}:enabled"
      output_var: "global_enabled"
  
  # If not enabled globally, return false
  - if:
      condition: "{{global_enabled}} != 'true'"
      then:
        - return:
            value:
              enabled: false
              reason: "Feature disabled globally"
  
  # Check user-specific override
  - redis_op:
      command: "GET"
      key: "feature:{{flag}}:user:{{user_id}}"
      output_var: "user_override"
  
  - return:
      value:
        enabled: "{{user_override == 'true' || global_enabled == 'true'}}"
        flag: "{{flag}}"
        user_id: "{{user_id}}"
```

## Best Practices

### 1. Key Naming Conventions

Use hierarchical keys with colons:
```
user:123:profile
session:abc-def:data
cache:query:users:list
ratelimit:192.168.1.1:2026-01-02
```

### 2. Always Set TTLs

Prevent memory leaks by setting expiration on temporary data:

```yaml
# ✅ Good - with TTL
- redis_op:
    command: "SET"
    key: "temp:{{id}}"
    value: "{{data}}"
    ttl_seconds: 3600

# ❌ Bad - no expiration
- redis_op:
    command: "SET"
    key: "temp:{{id}}"
    value: "{{data}}"
```

### 3. Check Cache Existence

Always handle null returns from GET:

```yaml
- redis_op:
    command: "GET"
    key: "user:{{id}}"
    output_var: "cached_user"

- if:
    condition: "{{cached_user}} == null"
    then:
      # Fetch from database
      - sql_op:
          query: "SELECT * FROM users WHERE id = ?"
          args: ["{{id}}"]
          output_var: "user"
```

### 4. Use Atomic Operations

Prefer INCR/DECR over GET-then-SET for counters:

```yaml
# ✅ Good - atomic
- redis_op:
    command: "INCR"
    key: "counter:{{id}}"
    output_var: "new_count"

# ❌ Bad - race condition
- redis_op:
    command: "GET"
    key: "counter:{{id}}"
    output_var: "count"
- set:
    var: "new_count"
    value: "{{count + 1}}"
- redis_op:
    command: "SET"
    key: "counter:{{id}}"
    value: "{{new_count}}"
```

## Performance Characteristics

| Operation | Time Complexity | Use Case |
|-----------|----------------|----------|
| GET | O(1) | Read cached data |
| SET | O(1) | Write/update cache |
| DEL | O(1) | Remove single key |
| EXPIRE | O(1) | Update TTL |
| INCR | O(1) | Atomic increment |
| DECR | O(1) | Atomic decrement |

## Error Handling

Redis operations return errors in these cases:

1. **Connection Failed**: Redis server unavailable
2. **Type Mismatch**: INCR/DECR on non-numeric values
3. **Out of Memory**: Redis memory limit reached

Example error handling:

```yaml
logic:
  - redis_op:
      command: "GET"
      key: "user:{{id}}"
      output_var: "result"
  
  # Check for null (key doesn't exist)
  - if:
      condition: "{{result}} == null"
      then:
        - return:
            value:
              error: "User not found in cache"
```

## Comparison with SqlOp

| Feature | RedisOp | SqlOp |
|---------|---------|-------|
| **Storage** | In-memory | On-disk |
| **Speed** | Sub-millisecond | 1-10ms |
| **Persistence** | Optional | Always |
| **Data Structure** | Key-value | Relational |
| **Best For** | Caching, sessions | Persistent data |
| **TTL Support** | Native | Manual |

## Common Patterns

### Cache-Aside Pattern

```yaml
# Read-through cache
- redis_op:
    command: "GET"
    key: "cache:{{key}}"
    output_var: "cached"

- if:
    condition: "{{cached}} != null"
    then:
      - return:
          value: "{{cached}}"
    otherwise:
      - sql_op:
          query: "SELECT * FROM table WHERE id = ?"
          args: ["{{id}}"]
          output_var: "data"
      - redis_op:
          command: "SET"
          key: "cache:{{key}}"
          value: "{{data}}"
          ttl_seconds: 300
      - return:
          value: "{{data}}"
```

### Write-Through Cache

```yaml
# Update both cache and database
- sql_op:
    query: "UPDATE users SET name = ? WHERE id = ?"
    args: ["{{new_name}}", "{{user_id}}"]
    output_var: "update_result"

- redis_op:
    command: "SET"
    key: "user:{{user_id}}:name"
    value: "{{new_name}}"
    ttl_seconds: 3600
    output_var: "cache_status"
```

### Cache Invalidation

```yaml
# Delete cache on update
- sql_op:
    query: "UPDATE products SET price = ? WHERE id = ?"
    args: ["{{new_price}}", "{{product_id}}"]
    output_var: "update_result"

- redis_op:
    command: "DEL"
    key: "product:{{product_id}}:details"
    output_var: "deleted"
```

## Next Steps

- Learn about [SQL Operations](SQL_OPERATIONS_GUIDE.md)
- Explore [Dynamic Routes Guide](DYNAMIC_ROUTES_GUIDE.md)
- Check [Complete API Reference](DYNAMIC_ROUTES_COMPLETE_GUIDE.md)

## Technical Details

### VM Optimization

RedisOp compiles to optimized bytecode:

```rust
OptimizedOperation::RedisOp {
    command: String,
    key: String,                    // Template with {{vars}}
    value: Option<String>,          // Template with {{vars}}
    ttl_seconds: Option<u64>,
    output_var_index: Option<usize> // Direct memory index (O(1))
}
```

### Connection Pooling

Worpen uses `deadpool-redis` for:
- Automatic connection reuse
- Connection health checks
- Configurable pool size
- Graceful error recovery

---

**Built with ❤️ for high-performance caching in Worpen**
