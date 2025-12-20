# Expression Reference

Complete reference for expression syntax, operators, functions, and pipes in Worpen Dynamic Routes.

## Table of Contents

1. [Expression Basics](#expression-basics)
2. [Operators](#operators)
3. [Functions](#functions)
4. [Pipe Filters](#pipe-filters)
5. [Helper Functions](#helper-functions)
6. [Type System](#type-system)
7. [Evaluation Rules](#evaluation-rules)
8. [Advanced Patterns](#advanced-patterns)

---

## Expression Basics

### Syntax

Expressions are enclosed in double curly braces with spaces:

```yaml
"{{ expression }}"
```

### Variable References

```yaml
# Simple variable
"{{ username }}"

# Object property
"{{ user.email }}"

# Array index
"{{ items[0] }}"

# Deep nesting
"{{ order.customer.address.city }}"
```

### Literals

```yaml
# Numbers
"{{ 42 }}"
"{{ 3.14159 }}"
"{{ -10 }}"

# Strings
"{{ 'hello' }}"
"{{ \"world\" }}"

# Booleans
"{{ true }}"
"{{ false }}"

# Null
"{{ null }}"
```

---

## Operators

### Arithmetic Operators

#### Addition `+`

```yaml
"{{ 10 + 5 }}"              # 15
"{{ price + tax }}"         # numeric addition
"{{ 'Hello' + ' World' }}"  # string concatenation
```

#### Subtraction `-`

```yaml
"{{ 10 - 5 }}"           # 5
"{{ total - discount }}"  # 95 if total=100, discount=5
```

#### Multiplication `*`

```yaml
"{{ 10 * 5 }}"            # 50
"{{ price * quantity }}"   # 200 if price=20, quantity=10
```

#### Division `/`

```yaml
"{{ 10 / 5 }}"         # 2
"{{ 10 / 3 }}"         # 3.333...
"{{ total / count }}"   # average
```

#### Modulo `%`

```yaml
"{{ 10 % 3 }}"        # 1
"{{ 17 % 5 }}"        # 2
"{{ number % 2 }}"    # 0 for even, 1 for odd
```

#### Power `**`

```yaml
"{{ 2 ** 8 }}"     # 256
"{{ 10 ** 2 }}"    # 100
"{{ x ** y }}"     # x to the power of y
```

#### Precedence

```yaml
"{{ 2 + 3 * 4 }}"      # 14 (multiplication first)
"{{ (2 + 3) * 4 }}"    # 20 (parentheses first)
"{{ 2 ** 3 + 1 }}"     # 9 (power first)
```

---

### Comparison Operators

#### Equal `==`

```yaml
"{{ 10 == 10 }}"          # true
"{{ 'hello' == 'hello' }}" # true
"{{ x == null }}"         # check if x is null
"{{ status == 'active' }}" # string comparison
```

#### Not Equal `!=`

```yaml
"{{ 10 != 5 }}"           # true
"{{ status != 'deleted' }}" # true if not deleted
```

#### Greater Than `>`

```yaml
"{{ 10 > 5 }}"      # true
"{{ age > 18 }}"    # check if adult
"{{ score > 90 }}"  # check if A grade
```

#### Less Than `<`

```yaml
"{{ 5 < 10 }}"         # true
"{{ price < budget }}"  # check affordability
```

#### Greater Than or Equal `>=`

```yaml
"{{ 10 >= 10 }}"    # true
"{{ age >= 21 }}"   # drinking age check
```

#### Less Than or Equal `<=`

```yaml
"{{ 5 <= 10 }}"        # true
"{{ score <= 100 }}"   # valid score check
```

---

### Logical Operators

#### AND `&&`

```yaml
"{{ true && true }}"              # true
"{{ (age >= 18) && (age <= 65) }}" # working age
"{{ is_active && !is_deleted }}"   # active and not deleted
```

#### OR `||`

```yaml
"{{ true || false }}"                    # true
"{{ is_admin || is_moderator }}"        # has privileges
"{{ (score > 90) || (attendance > 0.95) }}" # excellent performance
```

#### NOT `!`

```yaml
"{{ !true }}"              # false
"{{ !is_banned }}"         # not banned
"{{ !(age < 18) }}"        # not minor (equivalent to age >= 18)
```

#### Complex Logic

```yaml
"{{ (a && b) || (c && d) }}"
"{{ !(a || b) }}"  # same as (!a && !b)
"{{ (score >= 60) && ((attendance >= 0.8) || has_excuse) }}"
```

---

### Ternary Operator

#### Syntax

```
condition ? true_value : false_value
```

#### Examples

```yaml
# Simple
"{{ age >= 18 ? 'adult' : 'minor' }}"

# With expressions
"{{ score >= 60 ? 'pass' : 'fail' }}"
"{{ items_count == 1 ? '1 item' : items_count + ' items' }}"

# Nested
"{{ age < 13 ? 'child' : (age < 18 ? 'teen' : 'adult') }}"

# With calculations
"{{ total > 100 ? total * 0.9 : total }}"  # 10% discount
```

---

## Functions

### Math Functions

#### `max(...)`

Returns the maximum value.

```yaml
"{{ max(10, 20, 30) }}"        # 30
"{{ max(a, b, c) }}"           # largest of three variables
"{{ max(price1, price2) }}"    # higher price
```

#### `min(...)`

Returns the minimum value.

```yaml
"{{ min(10, 20, 30) }}"        # 10
"{{ min(a, b, c) }}"           # smallest of three variables
"{{ min(budget, price) }}"     # lower value
```

#### `abs(x)`

Returns absolute value.

```yaml
"{{ abs(-42) }}"     # 42
"{{ abs(10) }}"      # 10
"{{ abs(diff) }}"    # always positive
```

#### `round(x, decimals?)`

Rounds to specified decimal places (default 0).

```yaml
"{{ round(3.14159) }}"     # 3
"{{ round(3.14159, 2) }}"  # 3.14
"{{ round(price, 2) }}"    # round to cents
```

#### `floor(x)`

Rounds down to nearest integer.

```yaml
"{{ floor(3.7) }}"    # 3
"{{ floor(9.99) }}"   # 9
```

#### `ceil(x)`

Rounds up to nearest integer.

```yaml
"{{ ceil(3.1) }}"     # 4
"{{ ceil(0.01) }}"    # 1
```

---

### String Functions

#### `len(x)`

Returns length of string or array.

```yaml
"{{ len('hello') }}"        # 5
"{{ len(username) }}"       # length of username
"{{ len(items) }}"          # array size
```

#### `upper(x)`

Converts to uppercase.

```yaml
"{{ upper('hello') }}"      # HELLO
"{{ upper(name) }}"         # uppercase name
```

#### `lower(x)`

Converts to lowercase.

```yaml
"{{ lower('HELLO') }}"      # hello
"{{ lower(email) }}"        # lowercase email
```

#### `trim(x)`

Removes leading/trailing whitespace.

```yaml
"{{ trim('  hello  ') }}"   # hello
"{{ trim(input) }}"         # clean input
```

#### `substring(x, start, end?)`

Extracts substring.

```yaml
"{{ substring('hello', 0, 2) }}"   # he
"{{ substring(text, 5) }}"         # from index 5 to end
```

#### `replace(x, search, replace)`

Replaces occurrences.

```yaml
"{{ replace('hello world', 'world', 'there') }}"  # hello there
"{{ replace(text, ' ', '_') }}"                   # replace spaces
```

---

### Array Functions

#### `has(array, value)`

Checks if array contains value.

```yaml
"{{ has(tags, 'urgent') }}"           # true if urgent in tags
"{{ has(permissions, 'admin') }}"     # check permission
```

#### `first(array)`

Returns first element.

```yaml
"{{ first(items) }}"        # first item
"{{ first(users).name }}"   # name of first user
```

#### `last(array)`

Returns last element.

```yaml
"{{ last(items) }}"         # last item
"{{ last(history).date }}"  # date of last entry
```

#### `slice(array, start, end?)`

Extracts subarray.

```yaml
"{{ slice(items, 0, 5) }}"    # first 5 items
"{{ slice(array, 10) }}"      # from index 10 to end
```

---

## Pipe Filters

### String Pipes

#### `upper`

Converts to uppercase.

```yaml
"{{ name | upper }}"                 # JOHN
"{{ 'hello' | upper }}"              # HELLO
```

#### `lower`

Converts to lowercase.

```yaml
"{{ name | lower }}"                 # john
"{{ email | lower }}"                # ensure lowercase
```

#### `trim`

Removes whitespace.

```yaml
"{{ input | trim }}"                 # clean input
"{{ '  hello  ' | trim }}"           # hello
```

#### `capitalize`

Capitalizes first letter.

```yaml
"{{ name | capitalize }}"            # John
"{{ 'hello world' | capitalize }}"   # Hello world
```

#### `title`

Capitalizes each word.

```yaml
"{{ name | title }}"                 # John Doe
"{{ 'hello world' | title }}"        # Hello World
```

#### `slugify`

Converts to URL-safe slug.

```yaml
"{{ title | slugify }}"              # hello-world
"{{ 'Hello World!' | slugify }}"     # hello-world
```

#### `truncate(length)`

Truncates to length.

```yaml
"{{ text | truncate(100) }}"         # first 100 chars
"{{ description | truncate(50) }}"   # preview
```

#### `replace(search, replace)`

Replaces text.

```yaml
"{{ text | replace('old', 'new') }}"
"{{ email | replace('@', ' at ') }}"
```

#### `split(delimiter)`

Splits into array.

```yaml
"{{ csv | split(',') }}"             # array of values
"{{ path | split('/') }}"            # path segments
```

#### `join(delimiter)`

Joins array into string.

```yaml
"{{ tags | join(', ') }}"            # tag1, tag2, tag3
"{{ parts | join('/') }}"            # path/to/file
```

---

### Number Pipes

#### `abs`

Absolute value.

```yaml
"{{ diff | abs }}"                   # always positive
"{{ -42 | abs }}"                    # 42
```

#### `round`

Rounds number.

```yaml
"{{ price | round }}"                # nearest integer
"{{ 3.14159 | round }}"              # 3
```

#### `floor`

Rounds down.

```yaml
"{{ value | floor }}"                # 3 from 3.7
"{{ 9.99 | floor }}"                 # 9
```

#### `ceil`

Rounds up.

```yaml
"{{ value | ceil }}"                 # 4 from 3.1
"{{ 0.01 | ceil }}"                  # 1
```

---

### Array Pipes

#### `length`

Array or string length.

```yaml
"{{ items | length }}"               # number of items
"{{ users | length }}"               # user count
```

#### `first`

First element.

```yaml
"{{ items | first }}"                # first item
"{{ users | first | get('name') }}"  # first user's name
```

#### `last`

Last element.

```yaml
"{{ items | last }}"                 # last item
"{{ history | last }}"               # most recent
```

#### `sort`

Sorts array.

```yaml
"{{ numbers | sort }}"               # ascending order
"{{ items | sort('price') }}"        # sort by price
```

#### `reverse`

Reverses array.

```yaml
"{{ items | reverse }}"              # reversed order
"{{ [1,2,3] | reverse }}"            # [3,2,1]
```

#### `unique`

Removes duplicates.

```yaml
"{{ tags | unique }}"                # unique tags
"{{ ids | unique }}"                 # deduplicated
```

#### `filter(condition)`

Filters array.

```yaml
"{{ items | filter('price > 100') }}"
"{{ users | filter('active == true') }}"
```

#### `map(property)`

Extracts property.

```yaml
"{{ users | map('email') }}"         # array of emails
"{{ items | map('price') }}"         # array of prices
```

#### `sum(property?)`

Sums values.

```yaml
"{{ numbers | sum }}"                # sum of numbers
"{{ items | sum('price') }}"         # total price
```

---

### Type Conversion Pipes

#### `to_string`

Converts to string.

```yaml
"{{ 123 | to_string }}"              # "123"
"{{ true | to_string }}"             # "true"
```

#### `to_number`

Converts to number.

```yaml
"{{ '123' | to_number }}"            # 123
"{{ '3.14' | to_number }}"           # 3.14
```

#### `to_bool`

Converts to boolean.

```yaml
"{{ 'true' | to_bool }}"             # true
"{{ 1 | to_bool }}"                  # true
"{{ 0 | to_bool }}"                  # false
```

---

### JSON Pipes

#### `json_parse`

Parses JSON string.

```yaml
"{{ json_string | json_parse }}"
"{{ '{\"name\":\"John\"}' | json_parse }}"
```

#### `json_stringify`

Converts to JSON string.

```yaml
"{{ object | json_stringify }}"
"{{ user | json_stringify }}"
```

---

### Crypto/Hash Pipes

#### `hash_bcrypt`

Hashes with bcrypt (for passwords).

```yaml
"{{ password | hash_bcrypt }}"
"{{ request.password | hash_bcrypt }}"
```

#### `hash_md5`

MD5 hash.

```yaml
"{{ data | hash_md5 }}"
"{{ email | hash_md5 }}"             # gravatar hash
```

#### `base64_encode`

Base64 encoding.

```yaml
"{{ data | base64_encode }}"
"{{ image | base64_encode }}"
```

#### `base64_decode`

Base64 decoding.

```yaml
"{{ encoded | base64_decode }}"
```

---

### Encoding Pipes

#### `url_encode`

URL-safe encoding.

```yaml
"{{ query | url_encode }}"
"{{ 'hello world' | url_encode }}"   # hello%20world
```

#### `html_escape`

HTML entity escaping.

```yaml
"{{ html | html_escape }}"
"{{ '<script>' | html_escape }}"     # &lt;script&gt;
```

---

## Helper Functions

### Date/Time Functions

#### `now_iso()`

Current time in ISO format.

```yaml
"{{ now_iso() }}"                    # 2025-12-20T10:30:00Z
```

#### `now_unix()`

Current Unix timestamp.

```yaml
"{{ now_unix() }}"                   # 1734690600
```

#### `today()`

Current date (YYYY-MM-DD).

```yaml
"{{ today() }}"                      # 2025-12-20
```

#### `now_time()`

Current time (HH:MM:SS).

```yaml
"{{ now_time() }}"                   # 10:30:00
```

#### `add_days(date, days)`

Adds days to date.

```yaml
"{{ add_days(today(), 7) }}"         # one week from now
"{{ add_days('2025-01-01', 30) }}"   # 2025-01-31
```

#### `format_timestamp(timestamp, format)`

Formats timestamp.

```yaml
"{{ format_timestamp(now_unix(), '%Y-%m-%d') }}"
"{{ format_timestamp(created_at, '%B %d, %Y') }}"
```

---

### String Helper Functions

#### `generate_uuid()`

Generates UUID v4.

```yaml
"{{ generate_uuid() }}"              # 550e8400-e29b-41d4-a716-446655440000
```

#### `random_string(length)`

Generates random string.

```yaml
"{{ random_string(32) }}"            # 32-char random string
"{{ random_string(16) }}"            # for tokens
```

#### `slugify(text)`

URL-safe slug.

```yaml
"{{ slugify('Hello World!') }}"      # hello-world
"{{ slugify(title) }}"               # article slug
```

#### `truncate(text, length)`

Truncates text.

```yaml
"{{ truncate(description, 100) }}"   # first 100 chars
"{{ truncate(text, 50) }}"           # preview
```

#### `initials(name)`

Extracts initials.

```yaml
"{{ initials('John Doe') }}"         # JD
"{{ initials(full_name) }}"          # user initials
```

#### `word_count(text)`

Counts words.

```yaml
"{{ word_count('hello world') }}"    # 2
"{{ word_count(article) }}"          # article length
```

---

### Number Helper Functions

#### `random_int(min, max)`

Generates random integer.

```yaml
"{{ random_int(1, 100) }}"           # 1-100
"{{ random_int(0, 1) }}"             # coin flip
```

#### `random_float(min, max)`

Generates random float.

```yaml
"{{ random_float(0.0, 1.0) }}"       # 0.0-1.0
"{{ random_float(1.5, 5.5) }}"       # decimal range
```

#### `format_number(number, decimals)`

Formats number with commas.

```yaml
"{{ format_number(1234567, 2) }}"    # 1,234,567.00
"{{ format_number(price, 2) }}"      # currency format
```

#### `format_bytes(bytes)`

Formats byte size.

```yaml
"{{ format_bytes(1024) }}"           # 1.00 KB
"{{ format_bytes(1048576) }}"        # 1.00 MB
```

---

### Validation Functions

#### `is_email(text)`

Validates email format.

```yaml
"{{ is_email('user@example.com') }}" # true
"{{ is_email(input) }}"              # check validity
```

#### `is_url(text)`

Validates URL format.

```yaml
"{{ is_url('https://example.com') }}" # true
"{{ is_url(link) }}"                  # check validity
```

#### `email_username(email)`

Extracts username from email.

```yaml
"{{ email_username('user@example.com') }}" # user
```

#### `email_domain(email)`

Extracts domain from email.

```yaml
"{{ email_domain('user@example.com') }}" # example.com
```

---

### JSON Helper Functions

#### `json_parse(text)`

Parses JSON.

```yaml
"{{ json_parse(json_string) }}"
"{{ json_parse(request.body) }}"
```

#### `json_stringify(value)`

Converts to JSON.

```yaml
"{{ json_stringify(object) }}"
"{{ json_stringify(user) }}"
```

---

### Crypto Helper Functions

#### `hash_password(password)`

Hashes password with bcrypt.

```yaml
"{{ hash_password(request.password) }}"
```

#### `md5_hash(text)`

MD5 hash.

```yaml
"{{ md5_hash(data) }}"
"{{ md5_hash(email) }}"              # gravatar
```

---

## Type System

### Supported Types

```yaml
# Number
number: 42
decimal: 3.14

# String
text: "hello"
multiline: |
  Line 1
  Line 2

# Boolean
flag: true
active: false

# Null
value: null

# Array
items: [1, 2, 3]
list:
  - item1
  - item2

# Object
user:
  name: "John"
  age: 30
```

### Type Coercion

```yaml
# String to Number
"{{ '42' + 10 }}"                    # 52

# Number to String
"{{ 42 + ' items' }}"                # "42 items"

# Boolean conversion
"{{ 1 ? 'true' : 'false' }}"         # "true"
"{{ 0 ? 'true' : 'false' }}"         # "false"

# Null handling
"{{ null || 'default' }}"            # "default"
"{{ value ?? 'default' }}"           # nullish coalescing
```

---

## Evaluation Rules

### Order of Operations

1. Parentheses `()`
2. Power `**`
3. Unary `-`, `!`
4. Multiplication `*`, Division `/`, Modulo `%`
5. Addition `+`, Subtraction `-`
6. Comparison `>`, `<`, `>=`, `<=`
7. Equality `==`, `!=`
8. Logical AND `&&`
9. Logical OR `||`
10. Ternary `? :`

### Short-Circuit Evaluation

```yaml
# AND - stops at first false
"{{ false && expensive_operation() }}"  # doesn't call function

# OR - stops at first true
"{{ true || expensive_operation() }}"   # doesn't call function
```

### Null Safety

```yaml
# Safe navigation
"{{ user?.address?.city }}"             # returns null if any is null

# Null coalescing
"{{ value ?? 'default' }}"              # use default if null

# Null check
"{{ value == null ? 'N/A' : value }}"
```

---

## Advanced Patterns

### Chaining Pipes

```yaml
"{{ name | lower | trim | capitalize }}"
"{{ email | lower | trim }}"
"{{ text | truncate(100) | html_escape }}"
```

### Complex Conditionals

```yaml
"{{ (score >= 90) && (attendance >= 0.8) ? 'A+' : 
    (score >= 80) && (attendance >= 0.7) ? 'A' : 
    (score >= 70) ? 'B' : 'C' }}"
```

### Array Operations

```yaml
"{{ items | filter('price > 100') | map('name') | join(', ') }}"
"{{ numbers | sort | reverse | first }}"  # maximum
"{{ tags | unique | sort | join(',') }}"
```

### Dynamic Property Access

```yaml
"{{ user[property_name] }}"
"{{ data[key1][key2] }}"
"{{ items[index].property }}"
```

### Expression Composition

```yaml
# Calculate with multiple variables
"{{ (price * quantity) * (1 - discount / 100) * (1 + tax / 100) }}"

# String formatting
"{{ 'Order #' + order_id + ' - $' + total | format_number(2) }}"

# Conditional chaining
"{{ is_premium ? (price * 0.8) : (price * 0.9) }}"
```

---

## Performance Tips

### 1. Avoid Expensive Operations in Loops

```yaml
# ❌ Bad
- for_each: "{{ items }}"
  do:
    - result: "{{ expensive_function(item) }}"

# ✅ Good - cache outside loop
- cached_value: "{{ expensive_function() }}"
- for_each: "{{ items }}"
  do:
    - result: "{{ item * cached_value }}"
```

### 2. Use Pipe Chains Efficiently

```yaml
# ❌ Bad - multiple variables
- step1: "{{ text | lower }}"
- step2: "{{ step1 | trim }}"
- result: "{{ step2 | capitalize }}"

# ✅ Good - single pipe chain
- result: "{{ text | lower | trim | capitalize }}"
```

### 3. Short-Circuit Complex Conditions

```yaml
# ❌ Bad - always evaluates both
"{{ complex_check() && another_check() }}"

# ✅ Good - stops at first false
"{{ simple_check && complex_check() }}"
```

### 4. Cache Repeated Calculations

```yaml
# ❌ Bad - calculates twice
- discount: "{{ price * 0.1 }}"
- total: "{{ price - (price * 0.1) }}"

# ✅ Good - calculate once
- discount: "{{ price * 0.1 }}"
- total: "{{ price - discount }}"
```

---

## Error Handling

### Common Expression Errors

```yaml
# Division by zero
"{{ 10 / 0 }}"                       # Error: division by zero

# Undefined variable
"{{ unknown_var }}"                  # Error: undefined variable

# Type mismatch
"{{ 'hello' * 5 }}"                  # Error: invalid operation

# Invalid function
"{{ invalid_func() }}"               # Error: unknown function
```

### Safe Fallbacks

```yaml
# Use ternary for safe defaults
"{{ value != null ? value : 'default' }}"

# Use OR for fallbacks
"{{ value || 'default' }}"

# Check before operation
"{{ count > 0 ? total / count : 0 }}"
```

---

## Summary

This reference covered:

✅ Expression syntax and basics  
✅ All operators (arithmetic, comparison, logical)  
✅ Built-in functions (math, string, array)  
✅ Pipe filters (30+ filters)  
✅ Helper functions (date, crypto, validation)  
✅ Type system and coercion  
✅ Evaluation rules and precedence  
✅ Advanced patterns and performance tips  

For more information:
- [YAML Syntax Guide](13-yaml-syntax.md)
- [Migration Guide](15-migration-guide.md)
- [Best Practices](16-best-practices.md)
