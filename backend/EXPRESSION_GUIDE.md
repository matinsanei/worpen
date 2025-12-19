# 🎯 Expression Syntax Guide

Complete guide to using expressions in YAML Dynamic Routes.

## Table of Contents
- [Overview](#overview)
- [Basic Syntax](#basic-syntax)
- [Variables](#variables)
- [Operators](#operators)
- [Pipe Filters](#pipe-filters)
- [Functions](#functions)
- [Examples](#examples)
- [Best Practices](#best-practices)

---

## Overview

Expressions enable dynamic data transformation in YAML routes using a powerful, intuitive syntax:

```yaml
response:
  total: "{{price * quantity}}"
  email: "{{email | trim | lower}}"
  status: "{{age >= 18 ? 'adult' : 'minor'}}"
```

### Expression Types

1. **Pure Expression**: `{{expr}}` - Returns actual value (number, array, object, boolean)
2. **Mixed Template**: `"Hello {{name}}"` - Returns string with embedded expressions

---

## Basic Syntax

### Simple Variables

```yaml
# Access request data
user_id: "{{user_id}}"
email: "{{email}}"
```

### Arithmetic Operations

```yaml
# Addition, subtraction, multiplication, division
subtotal: "{{price * quantity}}"
discount: "{{subtotal * 0.1}}"
total: "{{subtotal - discount}}"
tax: "{{total * 0.15}}"

# Power operator
area: "{{radius ** 2 * 3.14}}"
```

### Comparison Operators

```yaml
# Equal, not equal
is_premium: "{{plan == 'premium'}}"
is_different: "{{old_price != new_price}}"

# Greater than, less than
is_adult: "{{age >= 18}}"
is_cheap: "{{price < 100}}"
```

### Logical Operators

```yaml
# AND, OR, NOT
can_access: "{{is_logged_in && is_premium}}"
show_banner: "{{is_new_user || has_discount}}"
is_invalid: "{{!is_valid}}"
```

### Ternary Operator

```yaml
# Simple ternary
status: "{{is_active ? 'Active' : 'Inactive'}}"

# Nested ternary
tier: "{{score > 90 ? 'gold' : (score > 70 ? 'silver' : 'bronze')}}"

# With arithmetic
shipping: "{{quantity > 5 ? 0 : 10}}"
discount: "{{is_premium ? total * 0.2 : 0}}"
```

---

## Variables

Access any data from the request context:

```yaml
# Request body fields
username: "{{username}}"
email: "{{email}}"

# Nested objects (if supported in context)
address: "{{user.address}}"
city: "{{user.address.city}}"

# Arrays (if supported)
first_item: "{{items[0]}}"
```

---

## Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `{{a + b}}` |
| `-` | Subtraction | `{{a - b}}` |
| `*` | Multiplication | `{{a * b}}` |
| `/` | Division | `{{a / b}}` |
| `%` | Modulo | `{{n % 2}}` |
| `**` | Power | `{{base ** exp}}` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `{{a == b}}` |
| `!=` | Not equal | `{{a != b}}` |
| `>` | Greater than | `{{a > b}}` |
| `>=` | Greater or equal | `{{a >= b}}` |
| `<` | Less than | `{{a < b}}` |
| `<=` | Less or equal | `{{a <= b}}` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&&` | Logical AND | `{{a && b}}` |
| `||` | Logical OR | `{{a || b}}` |
| `!` | Logical NOT | `{{!a}}` |

### Operator Precedence

1. `!` (unary NOT)
2. `**` (power)
3. `*`, `/`, `%` (multiplicative)
4. `+`, `-` (additive)
5. `==`, `!=`, `>`, `>=`, `<`, `<=` (comparison)
6. `&&` (logical AND)
7. `||` (logical OR)
8. `? :` (ternary)

Use parentheses to override precedence:
```yaml
result: "{{(a + b) * (c - d)}}"
```

---

## Pipe Filters

Pipe filters transform values using `|` operator:

```yaml
# Basic syntax
value: "{{input | filter}}"

# Chaining filters
value: "{{email | trim | lower}}"

# Filters with arguments
value: "{{text | substring(0, 10)}}"
```

### String Filters

#### `upper`
Converts string to uppercase.
```yaml
name_upper: "{{name | upper}}"
# "john" → "JOHN"
```

#### `lower`
Converts string to lowercase.
```yaml
email_lower: "{{email | lower}}"
# "USER@EXAMPLE.COM" → "user@example.com"
```

#### `trim`
Removes leading and trailing whitespace.
```yaml
username: "{{input | trim}}"
# "  john  " → "john"
```

#### `capitalize`
Capitalizes first letter, lowercases rest.
```yaml
title: "{{name | capitalize}}"
# "john doe" → "John doe"
```

#### `reverse`
Reverses string characters.
```yaml
reversed: "{{text | reverse}}"
# "hello" → "olleh"
```

#### `length`
Returns string or array length.
```yaml
name_length: "{{name | length}}"
# "john" → 4
```

#### `substring(start, length)`
Extracts substring.
```yaml
initials: "{{name | substring(0, 1)}}"
# "John" → "J"

preview: "{{text | substring(0, 100)}}"
# First 100 characters
```

#### `slice(start, end)`
Extracts slice of string or array.
```yaml
preview: "{{text | slice(0, 50)}}"
# Characters 0-50

middle: "{{text | slice(10, 20)}}"
# Characters 10-20
```

#### `split(delimiter)`
Splits string into array.
```yaml
words: "{{text | split(' ')}}"
# "hello world" → ["hello", "world"]

parts: "{{csv | split(',')}}"
# "a,b,c" → ["a", "b", "c"]
```

#### `replace(old, new)`
Replaces text in string.
```yaml
fixed: "{{text | replace('bad', 'good')}}"
# "bad day" → "good day"
```

#### `repeat(count)`
Repeats string N times.
```yaml
stars: "{{'*' | repeat(5)}}"
# "*****"
```

### Array Filters

#### `length`
Returns array length.
```yaml
item_count: "{{items | length}}"
# [1, 2, 3] → 3
```

#### `first`
Returns first element.
```yaml
first_item: "{{items | first}}"
# [1, 2, 3] → 1
```

#### `last`
Returns last element.
```yaml
last_item: "{{items | last}}"
# [1, 2, 3] → 3
```

#### `sort`
Sorts array in ascending order.
```yaml
sorted: "{{items | sort}}"
# [3, 1, 2] → [1, 2, 3]
```

#### `reverse`
Reverses array order.
```yaml
reversed: "{{items | reverse}}"
# [1, 2, 3] → [3, 2, 1]
```

#### `unique`
Removes duplicate elements.
```yaml
unique_items: "{{items | unique}}"
# [1, 2, 2, 3] → [1, 2, 3]
```

#### `join(delimiter)`
Joins array elements into string.
```yaml
csv: "{{items | join(', ')}}"
# [1, 2, 3] → "1, 2, 3"

tags: "{{tags | join(' #')}}"
# ["react", "vue"] → "react #vue"
```

### Object Filters

#### `keys`
Returns array of object keys.
```yaml
field_names: "{{data | keys}}"
# {"a": 1, "b": 2} → ["a", "b"]
```

#### `values`
Returns array of object values.
```yaml
field_values: "{{data | values}}"
# {"a": 1, "b": 2} → [1, 2]
```

#### `has(key)`
Checks if object has key.
```yaml
has_email: "{{user | has('email')}}"
# {"email": "..."} → true
```

### Math Filters

#### `abs`
Returns absolute value.
```yaml
absolute: "{{number | abs}}"
# -42 → 42
```

#### `round`
Rounds to nearest integer.
```yaml
rounded: "{{price | round}}"
# 42.7 → 43
```

#### `floor`
Rounds down to integer.
```yaml
floored: "{{price | floor}}"
# 42.7 → 42
```

#### `ceil`
Rounds up to integer.
```yaml
ceiled: "{{price | ceil}}"
# 42.3 → 43
```

### Type Conversion Filters

#### `to_string`
Converts to string.
```yaml
str: "{{number | to_string}}"
# 42 → "42"
```

#### `to_number`
Converts to number.
```yaml
num: "{{string | to_number}}"
# "42" → 42
```

#### `to_bool`
Converts to boolean.
```yaml
bool: "{{value | to_bool}}"
# "true" → true
# 1 → true
# "" → false
```

### Filter Chaining

Combine multiple filters:

```yaml
# Email normalization
email_clean: "{{email | trim | lower}}"
# "  USER@EXAMPLE.COM  " → "user@example.com"

# Name formatting
display_name: "{{first_name | trim | capitalize}} {{last_name | trim | capitalize}}"
# "  john ", "  doe  " → "John Doe"

# Text preview
preview: "{{description | trim | substring(0, 100)}}..."
# Long text → "First 100 characters..."

# Array manipulation
unique_sorted: "{{items | unique | sort}}"
# [3, 1, 2, 1] → [1, 2, 3]

# String array to CSV
csv: "{{names | unique | sort | join(', ')}}"
# ["Bob", "Alice", "Bob"] → "Alice, Bob"
```

---

## Functions

Built-in functions for common operations:

### Math Functions

#### `max(a, b, ...)`
Returns maximum value.
```yaml
highest: "{{max(10, 25, 15, 30, 5)}}"
# 30
```

#### `min(a, b, ...)`
Returns minimum value.
```yaml
lowest: "{{min(10, 25, 15, 30, 5)}}"
# 5
```

#### `abs(n)`
Returns absolute value.
```yaml
absolute: "{{abs(-42)}}"
# 42
```

#### `len(arr)`
Returns array length.
```yaml
count: "{{len(items)}}"
# Same as {{items | length}}
```

---

## Examples

### User Registration

```yaml
name: user_registration
path: /api/register
method: POST

response:
  success: true
  user:
    email: "{{email | trim | lower}}"
    username: "{{username | trim}}"
    display_name: "{{first_name | capitalize}} {{last_name | capitalize}}"
    is_adult: "{{age >= 18}}"
    account_type: "{{is_premium ? 'Premium' : 'Free'}}"
```

### E-commerce Checkout

```yaml
name: checkout
path: /api/checkout
method: POST

response:
  subtotal: "{{price * quantity}}"
  tax: "{{price * quantity * 0.1}}"
  shipping: "{{quantity > 5 ? 0 : 10}}"
  total: "{{price * quantity * 1.1 + (quantity > 5 ? 0 : 10)}}"
  discount_eligible: "{{price * quantity > 100}}"
```

### Complex Pricing Logic

```yaml
name: order_pricing
path: /api/orders/price
method: POST

response:
  customer:
    email: "{{email | trim | lower}}"
    level: "{{customer_level | upper}}"
  pricing:
    subtotal: "{{base_price * quantity}}"
    discount_rate: "{{customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05)}}"
    discount_amount: "{{base_price * quantity * (customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05))}}"
    shipping: "{{base_price * quantity > 100 ? 0 : 15}}"
    final_total: "{{base_price * quantity * (1 - (customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05))) + (base_price * quantity > 100 ? 0 : 15)}}"
```

### Array Operations

```yaml
name: array_demo
path: /api/array
method: POST

response:
  original: "{{items}}"
  count: "{{items | length}}"
  first: "{{items | first}}"
  last: "{{items | last}}"
  sorted: "{{items | sort}}"
  unique: "{{items | unique}}"
  joined: "{{items | join(', ')}}"
```

### String Operations

```yaml
name: string_demo
path: /api/string
method: POST

response:
  original: "{{text}}"
  upper: "{{text | upper}}"
  lower: "{{text | lower}}"
  trimmed: "{{text | trim}}"
  capitalized: "{{text | trim | lower | capitalize}}"
  reversed: "{{text | trim | reverse}}"
  length: "{{text | trim | length}}"
```

---

## Best Practices

### 1. Always Trim User Input

```yaml
# ❌ Don't
email: "{{email | lower}}"

# ✅ Do
email: "{{email | trim | lower}}"
```

### 2. Use Parentheses for Complex Expressions

```yaml
# ❌ Hard to read
total: "{{price * quantity + tax - discount}}"

# ✅ Clear precedence
total: "{{(price * quantity) + tax - discount}}"
```

### 3. Chain Filters for Readability

```yaml
# ❌ Nested expressions
name: "{{capitalize(trim(lower(first_name)))}}"

# ✅ Pipe chaining
name: "{{first_name | lower | trim | capitalize}}"
```

### 4. Use Descriptive Variable Names

```yaml
# ❌ Unclear
result: "{{a * b * 1.1}}"

# ✅ Clear intent
total_with_tax: "{{price * quantity * 1.1}}"
```

### 5. Extract Complex Logic

```yaml
# ❌ Repeated logic
discount: "{{customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05)}}"
final: "{{price * (1 - (customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05)))}}"

# ✅ Calculate once, reuse
discount_rate: "{{customer_level == 'premium' ? 0.2 : (customer_level == 'gold' ? 0.15 : 0.05)}}"
final: "{{price * (1 - discount_rate)}}"
```

### 6. Handle Edge Cases

```yaml
# ✅ Safe division
average: "{{total > 0 ? total / count : 0}}"

# ✅ Default values
name: "{{user_name | trim | length > 0 ? user_name : 'Anonymous'}}"
```

### 7. Type Consistency

```yaml
# ✅ Consistent types
id: "{{user_id}}"                    # Number
name: "{{username}}"                 # String
is_active: "{{status == 'active'}}"  # Boolean
items: "{{product_list}}"            # Array
```

---

## Performance Tips

1. **Avoid Redundant Filters**: `{{text | trim | trim}}` (redundant)
2. **Short-circuit Logical Operators**: Use `&&` and `||` efficiently
3. **Minimize Nested Ternary**: More than 2 levels reduces readability
4. **Cache Complex Expressions**: Calculate once, reference multiple times

---

## Error Handling

### Common Errors

1. **Undefined Variable**: `{{missing_var}}` → Error
2. **Type Mismatch**: `{{"text" + 42}}` → Error
3. **Invalid Filter**: `{{value | unknown_filter}}` → Error
4. **Division by Zero**: `{{10 / 0}}` → Error

### Best Practices

```yaml
# Use ternary for safe operations
safe_divide: "{{denominator != 0 ? numerator / denominator : 0}}"

# Check before accessing
has_email: "{{email | length > 0}}"
```

---

## Migration Guide

### From JSON Logic Operations

**Before (JSON):**
```json
{
  "logic": [
    {"operation": "set", "variable": "email_lower", "value": "{{email}}"},
    {"operation": "call_function", "name": "string_lower", "args": ["{{email_lower}}"]}
  ]
}
```

**After (YAML with Expressions):**
```yaml
response:
  email_lower: "{{email | lower}}"
```

### Benefits
- **90% fewer lines**: 60+ lines → 6 lines
- **Better readability**: Self-documenting syntax
- **Less boilerplate**: No explicit operations
- **Type safety**: Automatic type inference

---

## Advanced Topics

### Expression Precedence

```yaml
# Without parentheses (default precedence)
result: "{{2 + 3 * 4}}"  # 14 (multiplication first)

# With parentheses (override precedence)
result: "{{(2 + 3) * 4}}"  # 20 (addition first)
```

### Truthy/Falsy Values

**Falsy values:**
- `false`
- `0`
- `""` (empty string)
- `null`

**Truthy values:**
- `true`
- Non-zero numbers
- Non-empty strings
- Arrays (even if empty)
- Objects (even if empty)

```yaml
is_valid: "{{value}}"  # Truthy check
is_empty: "{{!value}}"  # Falsy check
```

---

## Summary

Expression syntax provides:
- ✅ **Concise**: 90% less code than JSON operations
- ✅ **Readable**: Self-documenting, intuitive syntax
- ✅ **Powerful**: 26 filters, functions, operators
- ✅ **Type-safe**: Automatic type inference and conversion
- ✅ **Chainable**: Pipe filters for complex transformations
- ✅ **Flexible**: Pure expressions or mixed templates

Start using expressions today to make your YAML routes more maintainable and expressive! 🚀
