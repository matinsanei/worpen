# Best Practices Guide

Production-ready patterns, anti-patterns, and optimization strategies for Worpen Dynamic Routes.

## Table of Contents

1. [Code Organization](#code-organization)
2. [Performance Optimization](#performance-optimization)
3. [Security Best Practices](#security-best-practices)
4. [Error Handling](#error-handling)
5. [Testing Strategies](#testing-strategies)
6. [SQL Best Practices](#sql-best-practices)
7. [HTTP Integration](#http-integration)
8. [Common Anti-Patterns](#common-anti-patterns)

---

## Code Organization

### File Structure

```
routes/
  ├── users/
  │   ├── get-user.yaml          # GET /users/:id
  │   ├── create-user.yaml       # POST /users
  │   ├── update-user.yaml       # PUT /users/:id
  │   └── delete-user.yaml       # DELETE /users/:id
  ├── orders/
  │   ├── create-order.yaml
  │   └── process-order.yaml
  └── shared/
      └── middleware.yaml         # Shared validation logic
```

**✅ Do:**
- Group routes by resource
- Use descriptive filenames
- Keep related logic together

**❌ Don't:**
- Put all routes in one file
- Use generic names like `route1.yaml`
- Mix unrelated logic

---

### Route Naming

```yaml
# ✅ Good - descriptive and consistent
path: /users/:id/orders
path: /api/v1/products/:id
path: /auth/login

# ❌ Bad - unclear or inconsistent
path: /u/:i/o
path: /api/stuff
path: /doLogin
```

---

### Variable Naming

```yaml
# ✅ Good - clear and consistent
- variable: user_id
- variable: total_price
- variable: is_authenticated

# ❌ Bad - cryptic or inconsistent
- variable: uid
- variable: tp
- variable: auth
```

---

### Comments

```yaml
# ✅ Good - explain WHY, not WHAT
logic:
  # Validate token before expensive operations
  - variable: auth_token
    value: "{{ request.headers.authorization }}"
  
  # Calculate discount based on user tier (see pricing doc)
  - variable: discount_rate
    value: "{{ user.tier == 'premium' ? 0.2 : 0.1 }}"

# ❌ Bad - state the obvious
logic:
  # Assign variable
  - variable: x
    value: 5
```

---

## Performance Optimization

### 1. Cache Expensive Operations

```yaml
# ❌ Bad - recalculates every time
logic:
  - for_each: "{{ items }}"
    do:
      - variable: tax_rate
        value: "{{ calculate_tax(item.category) }}"  # expensive!

# ✅ Good - calculate once
logic:
  - variable: electronics_tax
    value: "{{ calculate_tax('electronics') }}"
  
  - variable: books_tax
    value: "{{ calculate_tax('books') }}"
  
  - for_each: "{{ items }}"
    do:
      - variable: item_tax
        value: "{{ item.category == 'electronics' ? electronics_tax : books_tax }}"
```

---

### 2. Minimize Database Queries

```yaml
# ❌ Bad - N+1 query problem
logic:
  - sql:
      query: SELECT * FROM orders
    assign: orders
  
  - for_each: "{{ orders }}"
    do:
      - sql:
          query: SELECT * FROM users WHERE id = :user_id
          params:
            user_id: "{{ item.user_id }}"
        assign: user

# ✅ Good - single JOIN query
logic:
  - sql:
      query: |
        SELECT o.*, u.name, u.email
        FROM orders o
        JOIN users u ON o.user_id = u.id
    assign: orders_with_users
```

---

### 3. Use Efficient Filters

```yaml
# ❌ Bad - filters entire array then takes first
- variable: active_user
  value: "{{ users | filter('status == \"active\"') | first }}"

# ✅ Good - SQL with LIMIT 1
- sql:
    query: SELECT * FROM users WHERE status = 'active' LIMIT 1
  assign: active_user
```

---

### 4. Short-Circuit Conditionals

```yaml
# ❌ Bad - always evaluates both
- if: "{{ expensive_check() && another_check() }}"

# ✅ Good - stops at first false
- if: "{{ simple_check && expensive_check() }}"

# ✅ Good - return early
- if: "{{ !is_authenticated }}"
  then:
    - return:
        status: 401
        body: { error: Unauthorized }
```

---

### 5. Limit Data Transfer

```yaml
# ❌ Bad - selects everything
- sql:
    query: SELECT * FROM users

# ✅ Good - select only needed columns
- sql:
    query: SELECT id, name, email FROM users

# ✅ Good - paginate large results
- sql:
    query: SELECT * FROM items LIMIT :limit OFFSET :offset
    params:
      limit: 100
      offset: "{{ page * 100 }}"
```

---

## Security Best Practices

### 1. Input Validation

```yaml
# ✅ Always validate user input
logic:
  - variable: email
    value: "{{ request.body.email | lower | trim }}"
  
  - if: "{{ !is_email(email) }}"
    then:
      - return:
          status: 400
          body: { error: Invalid email format }
  
  - variable: age
    value: "{{ request.body.age }}"
  
  - if: "{{ age < 0 || age > 150 }}"
    then:
      - return:
          status: 400
          body: { error: Invalid age }
```

---

### 2. SQL Injection Prevention

```yaml
# ❌ DANGEROUS - SQL injection vulnerability
- sql:
    query: "SELECT * FROM users WHERE email = '{{ email }}'"

# ✅ Good - parameterized query
- sql:
    query: SELECT * FROM users WHERE email = :email
    params:
      email: "{{ email }}"
```

---

### 3. Authentication & Authorization

```yaml
# ✅ Check authentication first
logic:
  - variable: auth_token
    value: "{{ request.headers.authorization }}"
  
  - if: "{{ auth_token == null }}"
    then:
      - return:
          status: 401
          body: { error: Unauthorized }
  
  # Verify token (could be JWT, session, etc.)
  - sql:
      query: SELECT user_id FROM sessions WHERE token = :token
      params:
        token: "{{ auth_token }}"
    assign: session
  
  - if: "{{ session == null }}"
    then:
      - return:
          status: 401
          body: { error: Invalid token }
  
  # Check authorization
  - sql:
      query: SELECT role FROM users WHERE id = :user_id
      params:
        user_id: "{{ session.user_id }}"
    assign: user
  
  - if: "{{ user.role != 'admin' }}"
    then:
      - return:
          status: 403
          body: { error: Forbidden }
```

---

### 4. Password Security

```yaml
# ✅ Always hash passwords
logic:
  - variable: password_hash
    value: "{{ request.body.password | hash_bcrypt }}"
  
  - sql:
      query: |
        INSERT INTO users (email, password_hash)
        VALUES (:email, :password_hash)
      params:
        email: "{{ email }}"
        password_hash: "{{ password_hash }}"

# ❌ NEVER store plain passwords
# ❌ NEVER log passwords
```

---

### 5. Rate Limiting

```yaml
# ✅ Implement rate limiting for sensitive endpoints
logic:
  - variable: client_ip
    value: "{{ request.ip }}"
  
  - sql:
      query: |
        SELECT COUNT(*) as request_count
        FROM rate_limits
        WHERE ip = :ip AND created_at > datetime('now', '-1 hour')
      params:
        ip: "{{ client_ip }}"
    assign: rate_limit_check
  
  - if: "{{ rate_limit_check.request_count > 100 }}"
    then:
      - return:
          status: 429
          body: { error: Too many requests }
  
  # Log request
  - sql:
      query: INSERT INTO rate_limits (ip) VALUES (:ip)
      params:
        ip: "{{ client_ip }}"
```

---

### 6. Data Exposure

```yaml
# ❌ Bad - exposes sensitive data
response:
  body: "{{ user }}"  # includes password_hash, etc.

# ✅ Good - selective fields
response:
  body:
    id: "{{ user.id }}"
    name: "{{ user.name }}"
    email: "{{ user.email }}"
    # password_hash intentionally omitted
```

---

## Error Handling

### 1. Graceful Degradation

```yaml
# ✅ Handle external API failures
logic:
  - try:
      - http:
          method: GET
          url: https://api.external.com/data
          timeout: 5000
        assign: api_data
    
    catch:
      # Fallback to default value
      - variable: api_data
        value: { status: unavailable }
```

---

### 2. Meaningful Error Messages

```yaml
# ❌ Bad - generic error
response:
  status: 400
  body: { error: Bad request }

# ✅ Good - specific error
response:
  status: 400
  body:
    error: Validation failed
    details:
      - field: email
        message: Invalid email format
      - field: age
        message: Must be between 0 and 150
```

---

### 3. Consistent Error Format

```yaml
# ✅ Standardized error response
response:
  status: "{{ error_code }}"
  body:
    error: "{{ error_message }}"
    code: "{{ error_type }}"
    timestamp: "{{ now_iso() }}"
    request_id: "{{ request.id }}"
```

---

### 4. Don't Expose Internal Details

```yaml
# ❌ Bad - leaks implementation
response:
  status: 500
  body: { error: "Database connection failed: host=localhost port=5432" }

# ✅ Good - generic message with logging
logic:
  - log:
      level: error
      message: "DB connection failed: {{ error }}"
  
  - return:
      status: 500
      body: { error: Internal server error }
```

---

## Testing Strategies

### 1. Unit Tests

```yaml
# Test individual operations
tests:
  - name: validate_email
    input:
      email: "user@example.com"
    expect:
      is_email: true
  
  - name: calculate_total
    input:
      items:
        - price: 10
          quantity: 2
        - price: 5
          quantity: 1
    expect:
      total: 25
```

---

### 2. Integration Tests

```yaml
# Test full route flow
tests:
  - name: create_user_success
    request:
      method: POST
      path: /users
      body:
        name: John Doe
        email: john@example.com
    expect:
      status: 201
      body:
        name: John Doe
        email: john@example.com
```

---

### 3. Edge Cases

```yaml
# Test boundary conditions
tests:
  - name: empty_input
    input: []
    expect: []
  
  - name: null_input
    input: null
    expect: { error: Invalid input }
  
  - name: large_input
    input: "{{ generate_array(10000) }}"
    expect: { processed: 10000 }
```

---

## SQL Best Practices

### 1. Use Transactions

```yaml
# ✅ Atomic operations
logic:
  - sql:
      query: BEGIN TRANSACTION
  
  - sql:
      query: INSERT INTO orders (...) VALUES (...)
      params: {...}
  
  - sql:
      query: UPDATE inventory SET quantity = quantity - :qty
      params:
        qty: "{{ order_quantity }}"
  
  - sql:
      query: COMMIT
```

---

### 2. Index Optimization

```yaml
# ✅ Use indexed columns in WHERE clauses
- sql:
    query: SELECT * FROM users WHERE email = :email  # email is indexed
    params:
      email: "{{ email }}"

# ❌ Avoid functions on indexed columns
- sql:
    query: SELECT * FROM users WHERE LOWER(email) = :email
    # This prevents index usage
```

---

### 3. Avoid SELECT *

```yaml
# ❌ Bad - unnecessary data transfer
- sql:
    query: SELECT * FROM users

# ✅ Good - select only needed columns
- sql:
    query: SELECT id, name, email FROM users
```

---

### 4. Batch Operations

```yaml
# ❌ Bad - multiple individual inserts
- for_each: "{{ items }}"
  do:
    - sql:
        query: INSERT INTO items (name) VALUES (:name)
        params:
          name: "{{ item.name }}"

# ✅ Good - single batch insert
- sql:
    query: |
      INSERT INTO items (name)
      VALUES {{ items | map('(:name)') | join(',') }}
    params:
      names: "{{ items | map('name') }}"
```

---

## HTTP Integration

### 1. Timeout Configuration

```yaml
# ✅ Always set timeouts
- http:
    method: GET
    url: https://api.external.com/data
    timeout: 5000  # 5 seconds
  assign: api_data
```

---

### 2. Retry Logic

```yaml
# ✅ Retry with exponential backoff
- variable: max_retries
  value: 3

- variable: retry_count
  value: 0

- while: "{{ retry_count < max_retries }}"
  do:
    - try:
        - http:
            method: GET
            url: https://api.external.com/data
          assign: api_data
        
        - break  # Success, exit loop
      
      catch:
        - variable: retry_count
          value: "{{ retry_count + 1 }}"
        
        - variable: delay
          value: "{{ 1000 * (2 ** retry_count) }}"  # exponential backoff
        
        - sleep: "{{ delay }}"
```

---

### 3. Header Management

```yaml
# ✅ Include necessary headers
- http:
    method: POST
    url: https://api.external.com/data
    headers:
      Content-Type: application/json
      Authorization: "Bearer {{ api_token }}"
      User-Agent: Worpen/1.0
    body:
      data: "{{ payload }}"
```

---

## Common Anti-Patterns

### ❌ Anti-Pattern 1: Magic Numbers

```yaml
# ❌ Bad
- if: "{{ status == 2 }}"

# ✅ Good
- variable: STATUS_ACTIVE
  value: 2

- if: "{{ status == STATUS_ACTIVE }}"
```

---

### ❌ Anti-Pattern 2: Deep Nesting

```yaml
# ❌ Bad - hard to read
- if: "{{ a }}"
  then:
    - if: "{{ b }}"
      then:
        - if: "{{ c }}"
          then:
            - variable: result
              value: "{{ d }}"

# ✅ Good - early returns
- if: "{{ !a }}"
  then:
    - return: { error: A failed }

- if: "{{ !b }}"
  then:
    - return: { error: B failed }

- if: "{{ !c }}"
  then:
    - return: { error: C failed }

- variable: result
  value: "{{ d }}"
```

---

### ❌ Anti-Pattern 3: Duplicated Logic

```yaml
# ❌ Bad - repeated validation
- if: "{{ !is_email(request.body.email) }}"
  then:
    - return: { error: Invalid email }

# ... later ...

- if: "{{ !is_email(request.body.email) }}"
  then:
    - return: { error: Invalid email }

# ✅ Good - validate once
- variable: email
  value: "{{ request.body.email }}"

- if: "{{ !is_email(email) }}"
  then:
    - return: { error: Invalid email }
```

---

### ❌ Anti-Pattern 4: Ignoring Errors

```yaml
# ❌ Bad - silently fails
- try:
    - sql:
        query: INSERT INTO logs (...)
  catch: []  # ignores error

# ✅ Good - handle or log
- try:
    - sql:
        query: INSERT INTO logs (...)
  catch:
    - log:
        level: error
        message: "Failed to insert log: {{ error }}"
```

---

### ❌ Anti-Pattern 5: Over-Engineering

```yaml
# ❌ Bad - unnecessarily complex
- variable: result
  value: "{{ (((a + b) * c) - d) / e }}"

- if: "{{ result > 0 }}"
  then:
    - variable: final
      value: "{{ result * 2 }}"
  else:
    - variable: final
      value: 0

# ✅ Good - simple and clear
- variable: final
  value: "{{ max(0, ((a + b) * c - d) / e) * 2 }}"
```

---

## Summary

Key takeaways:

✅ **Organization** - Group by resource, use clear names  
✅ **Performance** - Cache, minimize queries, short-circuit  
✅ **Security** - Validate input, use parameterized queries, hash passwords  
✅ **Errors** - Handle gracefully, provide context, don't expose internals  
✅ **Testing** - Unit, integration, edge cases  
✅ **SQL** - Transactions, indexes, specific columns  
✅ **HTTP** - Timeouts, retries, proper headers  
✅ **Avoid** - Magic numbers, deep nesting, duplication, ignoring errors  

For more information:
- [YAML Syntax Guide](13-yaml-syntax.md)
- [Expression Reference](14-expressions.md)
- [Migration Guide](15-migration-guide.md)
