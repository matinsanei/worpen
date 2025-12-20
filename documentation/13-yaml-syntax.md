# YAML Syntax Guide

Complete guide to YAML-based Dynamic Routes in Worpen.

## Table of Contents

1. [Introduction](#introduction)
2. [Basic Structure](#basic-structure)
3. [Variable Assignment](#variable-assignment)
4. [Expressions](#expressions)
5. [Control Flow](#control-flow)
6. [Loops](#loops)
7. [SQL Operations](#sql-operations)
8. [HTTP Requests](#http-requests)
9. [Validation](#validation)
10. [Best Practices](#best-practices)

---

## Introduction

YAML format provides a cleaner, more readable alternative to JSON for defining dynamic routes. This guide covers all YAML-specific features and syntax.

### Why YAML?

**Before (JSON):**
```json
{
  "operation": "set",
  "variable": "email",
  "value": "{{request.email}}"
}
```

**After (YAML):**
```yaml
- email: "{{ request.email }}"
```

**Benefits:**
- 60-70% less verbose
- Better readability
- Support for comments
- Multi-line strings
- Shorthand syntax

---

## Basic Structure

### Minimal Route

```yaml
name: hello_world
path: /api/hello
method: GET
logic:
  - return: { message: "Hello, World!" }
```

### Complete Route Structure

```yaml
# Route metadata
name: user_registration
description: Register new users with validation
path: /api/register
method: POST

# Optional authentication
auth_required: true

# Optional rate limiting
rate_limit: 100  # requests per minute

# Input schema validation
schema:
  email:
    type: string
    required: true
    format: email
  password:
    type: string
    required: true
    min_length: 8

# Route logic
logic:
  - email: "{{ request.email | lower | trim }}"
  - if: "{{ email == '' }}"
    return: { error: "Email required" }
  
  - sql: SELECT id FROM users WHERE email = :email
    params: { email: "{{ email }}" }
  
  - if: "{{ sql_result != null }}"
    return: { error: "Email already exists" }
  
  - hashed: "{{ request.password | hash_bcrypt }}"
  - sql: |
      INSERT INTO users (email, password, created_at)
      VALUES (:email, :hashed, datetime('now'))
    params:
      email: "{{ email }}"
      hashed: "{{ hashed }}"
  
  - return:
      status: 201
      message: "User created"
      user_id: "{{ sql_result.last_insert_id }}"

# Optional response schema
response_schema:
  type: object
  properties:
    status:
      type: integer
    message:
      type: string
    user_id:
      type: integer

# Route metadata
enabled: true
version: "1.0.0"
```

---

## Variable Assignment

### Simple Assignment

```yaml
# Assign literal value
- name: "John Doe"

# Assign from request
- email: "{{ request.email }}"

# Assign from another variable
- email_copy: "{{ email }}"

# Assign expression result
- total: "{{ price * quantity }}"
```

### Multiple Assignments

```yaml
- first_name: "{{ request.first_name }}"
- last_name: "{{ request.last_name }}"
- full_name: "{{ first_name + ' ' + last_name }}"
```

### Complex Expressions

```yaml
# Mathematical
- subtotal: "{{ price * quantity }}"
- tax: "{{ subtotal * 0.09 }}"
- total: "{{ subtotal + tax }}"

# String operations
- slug: "{{ title | slugify }}"
- excerpt: "{{ content | truncate(100) }}"

# Conditional
- status: "{{ age >= 18 ? 'adult' : 'minor' }}"
```

---

## Expressions

### Expression Syntax

Expressions are enclosed in `{{ }}` and support:

#### Arithmetic Operators

```yaml
- sum: "{{ 10 + 5 }}"           # 15
- diff: "{{ 10 - 5 }}"          # 5
- product: "{{ 10 * 5 }}"       # 50
- quotient: "{{ 10 / 5 }}"      # 2
- remainder: "{{ 10 % 3 }}"     # 1
- power: "{{ 2 ** 8 }}"         # 256
```

#### Comparison Operators

```yaml
- is_equal: "{{ 10 == 10 }}"        # true
- not_equal: "{{ 10 != 5 }}"        # true
- greater: "{{ 10 > 5 }}"           # true
- less: "{{ 5 < 10 }}"              # true
- greater_eq: "{{ 10 >= 10 }}"      # true
- less_eq: "{{ 5 <= 10 }}"          # true
```

#### Logical Operators

```yaml
- and_result: "{{ true && false }}"     # false
- or_result: "{{ true || false }}"      # true
- not_result: "{{ !true }}"             # false
- complex: "{{ (a > 5) && (b < 10) }}"  # boolean
```

#### String Concatenation

```yaml
- greeting: "{{ 'Hello, ' + name }}"
- full_name: "{{ first + ' ' + last }}"
- template: "{{ 'User ' + id + ' logged in' }}"
```

#### Ternary Operator

```yaml
- status: "{{ age >= 18 ? 'adult' : 'minor' }}"
- discount: "{{ total > 100 ? 0.1 : 0 }}"
- message: "{{ count == 1 ? '1 item' : count + ' items' }}"
```

#### Nested Ternary

```yaml
- category: "{{ age < 13 ? 'child' : (age < 18 ? 'teen' : (age < 65 ? 'adult' : 'senior')) }}"
```

### Variable References

```yaml
# Simple variable
- value: "{{ my_var }}"

# Nested object access
- email: "{{ user.email }}"
- city: "{{ address.city }}"
- first_item: "{{ items[0] }}"

# Deep nesting
- value: "{{ user.address.city }}"
- value: "{{ users[0].profile.name }}"
```

### Pipe Operators

Chain operations using pipes:

```yaml
# String pipes
- clean: "{{ name | lower | trim | capitalize }}"
- slug: "{{ title | slugify }}"
- safe: "{{ html | escape }}"

# Array pipes
- count: "{{ items | length }}"
- first: "{{ items | first }}"
- last: "{{ items | last }}"
- sorted: "{{ items | sort }}"

# Number pipes
- rounded: "{{ price | round }}"
- absolute: "{{ diff | abs }}"
- floored: "{{ value | floor }}"
- ceiled: "{{ value | ceil }}"

# Type conversion
- as_string: "{{ 123 | to_string }}"
- as_number: "{{ '123' | to_number }}"
- as_bool: "{{ 'true' | to_bool }}"

# JSON operations
- parsed: "{{ json_string | json_parse }}"
- stringified: "{{ object | json_stringify }}"
```

### Functions

Built-in functions:

```yaml
# Math functions
- maximum: "{{ max(a, b, c) }}"
- minimum: "{{ min(x, y) }}"
- absolute: "{{ abs(-42) }}"
- rounded: "{{ round(3.14159, 2) }}"

# String functions
- length: "{{ len('hello') }}"        # 5
- upper: "{{ upper('hello') }}"       # HELLO
- lower: "{{ lower('HELLO') }}"       # hello
- trim: "{{ trim('  hello  ') }}"     # hello

# Array functions
- size: "{{ len(items) }}"
- has_item: "{{ has(items, 'apple') }}"
```

---

## Control Flow

### If Statements

#### Simple If

```yaml
- if: "{{ age >= 18 }}"
  then:
    - status: "adult"
    - can_vote: true
```

#### If-Else

```yaml
- if: "{{ score >= 60 }}"
  then:
    - grade: "pass"
  else:
    - grade: "fail"
```

#### Multiple Conditions

```yaml
- if: "{{ age < 13 }}"
  then:
    - category: "child"
  else_if: "{{ age < 18 }}"
  then:
    - category: "teen"
  else:
    - category: "adult"
```

#### Complex Conditions

```yaml
- if: "{{ (score >= 90) && (attendance >= 0.8) }}"
  then:
    - grade: "A+"
  else_if: "{{ (score >= 80) && (attendance >= 0.7) }}"
  then:
    - grade: "A"
  else:
    - grade: "B"
```

### Early Return

```yaml
# Validation with early return
- if: "{{ request.email == null }}"
  return:
    status: 400
    error: "Email is required"

- if: "{{ request.password == null }}"
  return:
    status: 400
    error: "Password is required"

# Continue with main logic
- email: "{{ request.email }}"
```

---

## Loops

### ForEach Loop

```yaml
# Simple iteration
- for_each: "{{ items }}"
  item: item
  do:
    - log: "Processing {{ item.name }}"

# With index
- for_each: "{{ users }}"
  item: user
  index: i
  do:
    - log: "User {{ i }}: {{ user.name }}"

# With loop metadata
- for_each: "{{ products }}"
  item: product
  loop_var: loop
  do:
    - if: "{{ loop.is_first }}"
      log: "First product: {{ product.name }}"
    
    - if: "{{ loop.is_last }}"
      log: "Last product: {{ product.name }}"
```

### Range Loop

```yaml
# Count up
- for_range:
    start: 1
    end: 10
  variable: i
  do:
    - log: "Count: {{ i }}"

# Count down
- for_range:
    start: 10
    end: 1
    step: -1
  variable: i
  do:
    - log: "Countdown: {{ i }}"

# Custom step
- for_range:
    start: 0
    end: 100
    step: 10
  variable: i
  do:
    - log: "Multiple of 10: {{ i }}"
```

### While Loop

```yaml
- counter: 0
- while: "{{ counter < 5 }}"
  max_iterations: 100
  do:
    - log: "Counter: {{ counter }}"
    - counter: "{{ counter + 1 }}"
```

### Until Loop

```yaml
- sum: 0
- until: "{{ sum >= 100 }}"
  max_iterations: 1000
  do:
    - value: "{{ random_int(1, 30) }}"
    - sum: "{{ sum + value }}"
    - log: "Sum is now: {{ sum }}"
```

### Loop Control

```yaml
# Break - exit loop
- for_each: "{{ items }}"
  item: item
  do:
    - if: "{{ item.price > 1000 }}"
      break: true
    - log: "Processing {{ item.name }}"

# Continue - skip iteration
- for_each: "{{ users }}"
  item: user
  do:
    - if: "{{ !user.active }}"
      continue: true
    - log: "Active user: {{ user.name }}"
```

### Nested Loops

```yaml
- for_range:
    start: 1
    end: 3
  variable: row
  do:
    - for_range:
        start: 1
        end: 3
      variable: col
      do:
        - cell: "{{ row * col }}"
        - log: "Cell[{{ row }},{{ col }}] = {{ cell }}"
```

---

## SQL Operations

### Simple Query

```yaml
- sql: SELECT * FROM users WHERE id = 1
```

### Parameterized Query

```yaml
# Positional parameters
- sql: SELECT * FROM users WHERE email = ?
  params: ["{{ email }}"]

# Named parameters (recommended)
- sql: SELECT * FROM users WHERE email = :email
  params:
    email: "{{ email }}"
```

### Multi-line Query

```yaml
- sql: |
    SELECT u.*, p.name as profile_name
    FROM users u
    LEFT JOIN profiles p ON u.id = p.user_id
    WHERE u.email = :email
      AND u.active = true
  params:
    email: "{{ email }}"
```

### INSERT Query

```yaml
- sql: |
    INSERT INTO users (email, password, created_at)
    VALUES (:email, :password, datetime('now'))
  params:
    email: "{{ email }}"
    password: "{{ hashed_password }}"

# Access inserted ID
- user_id: "{{ sql_result.last_insert_id }}"
```

### UPDATE Query

```yaml
- sql: |
    UPDATE users
    SET last_login = datetime('now'),
        login_count = login_count + 1
    WHERE id = :user_id
  params:
    user_id: "{{ user_id }}"

# Check affected rows
- if: "{{ sql_result.rows_affected == 0 }}"
  return: { error: "User not found" }
```

### DELETE Query

```yaml
- sql: DELETE FROM sessions WHERE user_id = :user_id
  params:
    user_id: "{{ user_id }}"

- log: "Deleted {{ sql_result.rows_affected }} sessions"
```

### Transaction

```yaml
- sql: BEGIN TRANSACTION

- sql: INSERT INTO orders (user_id, total) VALUES (:uid, :total)
  params:
    uid: "{{ user_id }}"
    total: "{{ total }}"

- order_id: "{{ sql_result.last_insert_id }}"

- for_each: "{{ items }}"
  item: item
  do:
    - sql: |
        INSERT INTO order_items (order_id, product_id, quantity)
        VALUES (:order_id, :product_id, :quantity)
      params:
        order_id: "{{ order_id }}"
        product_id: "{{ item.product_id }}"
        quantity: "{{ item.quantity }}"

- sql: COMMIT
```

---

## HTTP Requests

### GET Request

```yaml
- http:
    method: GET
    url: "https://api.example.com/users/{{ user_id }}"
    headers:
      Authorization: "Bearer {{ api_token }}"

- user_data: "{{ http_response.body }}"
```

### POST Request

```yaml
- http:
    method: POST
    url: "https://api.example.com/users"
    headers:
      Content-Type: "application/json"
      Authorization: "Bearer {{ api_token }}"
    body:
      name: "{{ name }}"
      email: "{{ email }}"

- if: "{{ http_response.status != 200 }}"
  return: { error: "Failed to create user" }

- created_user: "{{ http_response.body }}"
```

### Error Handling

```yaml
- http:
    method: GET
    url: "{{ external_api_url }}"
    timeout: 5000  # milliseconds

- if: "{{ http_response.error != null }}"
  log: "HTTP request failed: {{ http_response.error }}"
  return: { error: "External service unavailable" }

- data: "{{ http_response.body }}"
```

---

## Validation

### Input Schema

```yaml
schema:
  # String validation
  email:
    type: string
    required: true
    format: email
    
  username:
    type: string
    required: true
    min_length: 3
    max_length: 20
    pattern: "^[a-zA-Z0-9_]+$"
  
  # Number validation
  age:
    type: number
    required: true
    minimum: 0
    maximum: 150
  
  price:
    type: number
    required: true
    minimum: 0
    exclusive_minimum: true
  
  # Boolean validation
  terms_accepted:
    type: boolean
    required: true
  
  # Array validation
  tags:
    type: array
    required: false
    min_items: 1
    max_items: 10
    items:
      type: string
  
  # Object validation
  address:
    type: object
    required: false
    properties:
      street:
        type: string
        required: true
      city:
        type: string
        required: true
      zip_code:
        type: string
        pattern: "^\\d{5}$"
```

### Custom Validation

```yaml
logic:
  # Manual validation
  - if: "{{ request.email == null || request.email == '' }}"
    return:
      status: 400
      error: "Email is required"
  
  - if: "{{ len(request.password) < 8 }}"
    return:
      status: 400
      error: "Password must be at least 8 characters"
  
  # Email format check
  - if: "{{ !is_email(request.email) }}"
    return:
      status: 400
      error: "Invalid email format"
```

---

## Best Practices

### 1. Use Descriptive Variable Names

```yaml
# ❌ Bad
- x: "{{ request.e }}"
- y: "{{ x | lower }}"

# ✅ Good
- email: "{{ request.email }}"
- normalized_email: "{{ email | lower | trim }}"
```

### 2. Add Comments

```yaml
# User registration endpoint
name: user_registration
path: /api/register
method: POST

logic:
  # Normalize email address
  - email: "{{ request.email | lower | trim }}"
  
  # Check for duplicate email
  - sql: SELECT id FROM users WHERE email = :email
    params: { email: "{{ email }}" }
  
  # Return error if email exists
  - if: "{{ sql_result != null }}"
    return:
      status: 409
      error: "Email already registered"
```

### 3. Use Named SQL Parameters

```yaml
# ❌ Bad - positional parameters
- sql: SELECT * FROM users WHERE email = ? AND active = ?
  params: ["{{ email }}", true]

# ✅ Good - named parameters
- sql: SELECT * FROM users WHERE email = :email AND active = :active
  params:
    email: "{{ email }}"
    active: true
```

### 4. Validate Early, Return Early

```yaml
logic:
  # Validate all inputs first
  - if: "{{ request.email == null }}"
    return: { status: 400, error: "Email required" }
  
  - if: "{{ request.password == null }}"
    return: { status: 400, error: "Password required" }
  
  # Continue with main logic
  - email: "{{ request.email }}"
  # ... rest of logic
```

### 5. Use Pipe Operators for Clarity

```yaml
# ❌ Bad - nested operations
- temp1: "{{ name }}"
- temp2: "{{ lower(temp1) }}"
- temp3: "{{ trim(temp2) }}"
- final_name: "{{ capitalize(temp3) }}"

# ✅ Good - pipe chain
- final_name: "{{ name | lower | trim | capitalize }}"
```

### 6. Break Down Complex Logic

```yaml
# ❌ Bad - complex one-liner
- total: "{{ ((price * quantity) * (1 - (discount / 100))) * (1 + (tax / 100)) }}"

# ✅ Good - step by step
- subtotal: "{{ price * quantity }}"
- discounted: "{{ subtotal * (1 - discount / 100) }}"
- total: "{{ discounted * (1 + tax / 100) }}"
```

### 7. Use Schema Validation

```yaml
# Define schema for automatic validation
schema:
  email:
    type: string
    required: true
    format: email
  password:
    type: string
    required: true
    min_length: 8

# No need for manual validation
logic:
  # Input is already validated by schema
  - email: "{{ request.email }}"
  - hashed: "{{ request.password | hash_bcrypt }}"
```

### 8. Handle Errors Gracefully

```yaml
logic:
  - sql: SELECT * FROM users WHERE id = :id
    params: { id: "{{ user_id }}" }
  
  - if: "{{ sql_result == null }}"
    return:
      status: 404
      error: "User not found"
  
  - user: "{{ sql_result }}"
  
  # Continue with found user
  - return:
      status: 200
      data: "{{ user }}"
```

### 9. Use Multi-line for Long Strings

```yaml
# ❌ Bad - long single line
- sql: "SELECT u.id, u.name, u.email, p.address, p.city FROM users u LEFT JOIN profiles p ON u.id = p.user_id WHERE u.active = true"

# ✅ Good - multi-line
- sql: |
    SELECT u.id, u.name, u.email, p.address, p.city
    FROM users u
    LEFT JOIN profiles p ON u.id = p.user_id
    WHERE u.active = true
```

### 10. Document Complex Expressions

```yaml
# Calculate order total with tax and shipping
# Formula: (items_total + shipping) * (1 + tax_rate)
- items_total: "{{ items | sum('price') }}"
- shipping_cost: "{{ items_total > 100 ? 0 : 10 }}"  # Free shipping over $100
- tax_amount: "{{ items_total * 0.09 }}"  # 9% tax
- total: "{{ items_total + shipping_cost + tax_amount }}"
```

---

## Complete Examples

### E-commerce Checkout

```yaml
name: checkout
description: Process order checkout with payment
path: /api/checkout
method: POST

schema:
  cart_items:
    type: array
    required: true
    min_items: 1
  payment_method:
    type: string
    required: true
    enum: [credit_card, paypal]

logic:
  # Calculate totals
  - subtotal: "{{ cart_items | sum('price') }}"
  - shipping: "{{ subtotal > 100 ? 0 : 10 }}"
  - tax: "{{ subtotal * 0.09 }}"
  - total: "{{ subtotal + shipping + tax }}"
  
  # Create order
  - sql: |
      INSERT INTO orders (user_id, subtotal, shipping, tax, total, status)
      VALUES (:uid, :subtotal, :shipping, :tax, :total, 'pending')
    params:
      uid: "{{ user_id }}"
      subtotal: "{{ subtotal }}"
      shipping: "{{ shipping }}"
      tax: "{{ tax }}"
      total: "{{ total }}"
  
  - order_id: "{{ sql_result.last_insert_id }}"
  
  # Insert order items
  - for_each: "{{ cart_items }}"
    item: item
    do:
      - sql: |
          INSERT INTO order_items (order_id, product_id, quantity, price)
          VALUES (:order_id, :product_id, :quantity, :price)
        params:
          order_id: "{{ order_id }}"
          product_id: "{{ item.product_id }}"
          quantity: "{{ item.quantity }}"
          price: "{{ item.price }}"
  
  # Process payment (external API)
  - http:
      method: POST
      url: "{{ payment_api_url }}"
      body:
        amount: "{{ total }}"
        currency: "USD"
        method: "{{ payment_method }}"
        order_id: "{{ order_id }}"
  
  - if: "{{ http_response.status != 200 }}"
    sql: UPDATE orders SET status = 'failed' WHERE id = :id
    params: { id: "{{ order_id }}" }
    return:
      status: 402
      error: "Payment failed"
  
  # Update order status
  - sql: UPDATE orders SET status = 'paid' WHERE id = :id
    params: { id: "{{ order_id }}" }
  
  # Return success
  - return:
      status: 200
      message: "Order placed successfully"
      order_id: "{{ order_id }}"
      total: "{{ total }}"
```

---

## Summary

This guide covered:

✅ Basic YAML structure for routes  
✅ Variable assignment and expressions  
✅ Control flow (if/else)  
✅ Loops (foreach, range, while, until)  
✅ SQL operations with named parameters  
✅ HTTP requests  
✅ Input validation with schemas  
✅ Best practices and patterns  
✅ Complete real-world examples  

For more information:
- [Expression Reference](14-expressions.md)
- [Migration Guide](15-migration-guide.md)
- [Best Practices](16-best-practices.md)
