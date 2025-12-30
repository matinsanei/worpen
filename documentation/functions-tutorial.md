# 🎯 Functions Tutorial - Step by Step Guide

Welcome to the **Functions Tutorial**! This guide will teach you everything about functions in Worpen - from basic concepts to advanced usage.

## 📚 What You'll Learn

- What functions are and why they're useful
- How to define your first function
- How to call functions with parameters
- Advanced concepts: recursion, error handling
- Real-world examples

---

## 🔰 Chapter 1: What Are Functions?

### The Problem: Code Repetition

Imagine you have an API that needs to calculate tax in multiple places:

```yaml
# Without functions - repetitive code
logic:
  - !MathOp
    operation: multiply
    args: ["{{price}}", 0.15]
  - !Set
    var: tax1
    value: "{{math_result}}"
  
  # Later in the same route...
  - !MathOp
    operation: multiply
    args: ["{{shipping}}", 0.15]
  - !Set
    var: tax2
    value: "{{math_result}}"
```

**Problems:**
- ❌ Code duplication
- ❌ Hard to maintain
- ❌ Easy to make mistakes
- ❌ Not reusable

### The Solution: Functions

```yaml
logic:
  # Define function once
  - !DefineFunction
    name: calculate_tax
    params: [amount]
    body:
      - !MathOp
        operation: multiply
        args: ["{{amount}}", 0.15]
      - !Return
        value: "{{math_result}}"
  
  # Use function multiple times
  - !CallFunction
    name: calculate_tax
    args: ["{{price}}"]
  - !Set
    var: tax1
    value: "{{function_result}}"
  
  - !CallFunction
    name: calculate_tax
    args: ["{{shipping}}"]
  - !Set
    var: tax2
    value: "{{function_result}}"
```

**Benefits:**
- ✅ Write once, use everywhere
- ✅ Easy to maintain
- ✅ Less errors
- ✅ Clean, readable code

---

## 🚀 Chapter 2: Your First Function

### Step 1: Create a Simple Function

Let's create an API that greets users:

```yaml
name: Greeting API
path: /greet
method: GET

logic:
  # Define the greeting function
  - !DefineFunction
    name: create_greeting
    params: [name]
    body:
      - !Return
        value: "Hello, {{name}}! Welcome to Worpen!"
  
  # Call the function
  - !CallFunction
    name: create_greeting
    args: ["{{name | default('Guest')}}"]
  
  # Return the result
  - !Return
    value:
      message: "{{function_result}}"
      timestamp: "{{now()}}"
```

### Step 2: Test Your Function

```bash
# Test with name
curl "http://127.0.0.1:3000/greet?name=Alice"
# {"message": "Hello, Alice! Welcome to Worpen!", "timestamp": "2025-12-30T..."}

# Test without name (uses default)
curl "http://127.0.0.1:3000/greet"
# {"message": "Hello, Guest! Welcome to Worpen!", "timestamp": "2025-12-30T..."}
```

### Step 3: Understanding the Parts

```
!DefineFunction          # Operation type
  name: create_greeting  # Function name (your choice)
  params: [name]         # Input parameters
  body: [...]           # What the function does
```

```
!CallFunction           # Operation type
  name: create_greeting # Which function to call
  args: [...]          # Arguments to pass
```

---

## 📝 Chapter 3: Function Parameters

### Single Parameter

```yaml
- !DefineFunction
  name: square
  params: [number]
  body:
    - !MathOp
      operation: multiply
      args: ["{{number}}", "{{number}}"]
    - !Return
      value: "{{math_result}}"

# Usage
- !CallFunction
  name: square
  args: [5]  # Result: 25
```

### Multiple Parameters

```yaml
- !DefineFunction
  name: calculate_total
  params: [price, tax_rate, discount]
  body:
    - !MathOp
      operation: multiply
      args: ["{{price}}", "{{tax_rate}}"]
    - !Set
      var: tax_amount
      value: "{{math_result}}"
    
    - !MathOp
      operation: multiply
      args: ["{{price}}", "{{discount}}"]
    - !Set
      var: discount_amount
      value: "{{math_result}}"
    
    - !MathOp
      operation: sum
      args: ["{{price}}", "{{tax_amount}}"]
    - !MathOp
      operation: subtract
      args: ["{{math_result}}", "{{discount_amount}}"]
    
    - !Return
      value: "{{math_result}}"

# Usage
- !CallFunction
  name: calculate_total
  args: [100, 0.15, 0.1]  # price=100, tax_rate=15%, discount=10%
# Result: 105 (100 + 15 - 10)
```

### No Parameters

```yaml
- !DefineFunction
  name: get_current_time
  params: []
  body:
    - !Return
      value: "{{now()}}"

# Usage
- !CallFunction
  name: get_current_time
  args: []
```

---

## 🔄 Chapter 4: Advanced Functions

### Conditional Logic in Functions

```yaml
- !DefineFunction
  name: get_user_status
  params: [score, is_premium]
  body:
    - !If
      condition: "{{is_premium}}"
      then:
        - !Return
          value: "Premium User"
      else:
        - !If
          condition: "{{score}} >= 90"
          then:
            - !Return
              value: "Gold Member"
          else:
            - !If
              condition: "{{score}} >= 70"
              then:
                - !Return
                  value: "Silver Member"
              else:
                - !Return
                  value: "Bronze Member"

# Usage
- !CallFunction
  name: get_user_status
  args: [85, false]
# Result: "Silver Member"
```

### Functions Calling Other Functions

```yaml
logic:
  # Define helper function
  - !DefineFunction
    name: format_currency
    params: [amount]
    body:
      - !Return
        value: "${{amount | round(2)}}"
  
  # Define main function
  - !DefineFunction
    name: create_invoice
    params: [items, tax_rate]
    body:
      # Calculate subtotal
      - !Set
        var: subtotal
        value: "{{items | sum}}"
      
      # Calculate tax
      - !MathOp
        operation: multiply
        args: ["{{subtotal}}", "{{tax_rate}}"]
      - !Set
        var: tax
        value: "{{math_result}}"
      
      # Calculate total
      - !MathOp
        operation: sum
        args: ["{{subtotal}}", "{{tax}}"]
      - !Set
        var: total
        value: "{{math_result}}"
      
      # Format amounts using helper function
      - !CallFunction
        name: format_currency
        args: ["{{subtotal}}"]
      - !Set
        var: formatted_subtotal
        value: "{{function_result}}"
      
      - !CallFunction
        name: format_currency
        args: ["{{tax}}"]
      - !Set
        var: formatted_tax
        value: "{{function_result}}"
      
      - !CallFunction
        name: format_currency
        args: ["{{total}}"]
      - !Set
        var: formatted_total
        value: "{{function_result}}"
      
      # Return formatted invoice
      - !Return
        value:
          subtotal: "{{formatted_subtotal}}"
          tax: "{{formatted_tax}}"
          total: "{{formatted_total}}"
  
  # Use the function
  - !CallFunction
    name: create_invoice
    args: ["{{items}}", 0.15]
  
  - !Return
    value: "{{function_result}}"
```

---

## 🔁 Chapter 5: Recursion (Advanced)

Recursion is when a function calls itself. It's perfect for problems that can be broken down into smaller versions of the same problem.

### Factorial Example

```yaml
logic:
  - !DefineFunction
    name: factorial
    params: [n]
    body:
      - !If
        condition: "{{n}} <= 1"
        then:
          - !Return
            value: 1
        else:
          # Call factorial with (n-1)
          - !CallFunction
            name: factorial
            args: ["{{n}} - 1"]
          # Multiply n by result
          - !MathOp
            operation: multiply
            args: ["{{n}}", "{{function_result}}"]
          - !Return
            value: "{{math_result}}"
  
  # Calculate 5!
  - !CallFunction
    name: factorial
    args: [5]
  - !Return
    value:
      number: 5
      factorial: "{{function_result}}"  # Result: 120
```

### Fibonacci Example

```yaml
logic:
  - !DefineFunction
    name: fibonacci
    params: [n]
    body:
      - !If
        condition: "{{n}} <= 1"
        then:
          - !Return
            value: "{{n}}"
        else:
          # Calculate fibonacci(n-1)
          - !CallFunction
            name: fibonacci
            args: ["{{n}} - 1"]
          - !Set
            var: fib_n1
            value: "{{function_result}}"
          
          # Calculate fibonacci(n-2)
          - !CallFunction
            name: fibonacci
            args: ["{{n}} - 2"]
          - !Set
            var: fib_n2
            value: "{{function_result}}"
          
          # Return sum
          - !MathOp
            operation: sum
            args: ["{{fib_n1}}", "{{fib_n2}}"]
          - !Return
            value: "{{math_result}}"
  
  # Calculate fibonacci(10)
  - !CallFunction
    name: fibonacci
    args: [10]
  - !Return
    value:
      fibonacci_10: "{{function_result}}"  # Result: 55
```

---

## 🎯 Chapter 6: Real-World Examples

### User Authentication Function

```yaml
logic:
  - !DefineFunction
    name: authenticate_user
    params: [email, password]
    body:
      # Find user by email
      - !QueryDb
        query: "SELECT id, password_hash, is_active FROM users WHERE email = ?"
        params: ["{{email | lower}}"]
      
      - !If
        condition: "{{db_result | length}} == 0"
        then:
          - !Return
            value:
              success: false
              error: "User not found"
        else: []
      
      - !Set
        var: user
        value: "{{db_result[0]}}"
      
      # Check if user is active
      - !If
        condition: "{{user.is_active}} != true"
        then:
          - !Return
            value:
              success: false
              error: "Account is disabled"
        else: []
      
      # Verify password
      - !Set
        var: hashed_input
        value: "{{hash(password, 'sha256')}}"
      
      - !If
        condition: "{{hashed_input}} == {{user.password_hash}}"
        then:
          - !Return
            value:
              success: true
              user_id: "{{user.id}}"
        else:
          - !Return
            value:
              success: false
              error: "Invalid password"
  
  # Use the function
  - !CallFunction
    name: authenticate_user
    args: ["{{email}}", "{{password}}"]
  
  - !Return
    value: "{{function_result}}"
```

### Shopping Cart Calculator

```yaml
logic:
  - !DefineFunction
    name: calculate_cart_total
    params: [items, tax_rate, shipping_rate]
    body:
      # Calculate subtotal
      - !Set
        var: subtotal
        value: "{{items | map('price * quantity') | sum}}"
      
      # Calculate tax
      - !MathOp
        operation: multiply
        args: ["{{subtotal}}", "{{tax_rate}}"]
      - !Set
        var: tax
        value: "{{math_result}}"
      
      # Calculate shipping (free over $50)
      - !If
        condition: "{{subtotal}} >= 50"
        then:
          - !Set
            var: shipping
            value: 0
        else:
          - !MathOp
            operation: multiply
            args: ["{{items | length}}", "{{shipping_rate}}"]
          - !Set
            var: shipping
            value: "{{math_result}}"
      
      # Calculate total
      - !MathOp
        operation: sum
        args: ["{{subtotal}}", "{{tax}}", "{{shipping}}"]
      
      - !Return
        value:
          subtotal: "{{subtotal}}"
          tax: "{{tax}}"
          shipping: "{{shipping}}"
          total: "{{math_result}}"
  
  # Use the function
  - !CallFunction
    name: calculate_cart_total
    args: ["{{cart_items}}", 0.15, 5.99]
  
  - !Return
    value: "{{function_result}}"
```

---

## 🧪 Chapter 7: Testing Functions

### Test Individual Functions

Create separate test routes for each function:

```yaml
# Test route for factorial function
name: Test Factorial
path: /test/factorial/:n
method: GET

logic:
  - !DefineFunction
    name: factorial
    params: [n]
    body:
      - !If
        condition: "{{n}} <= 1"
        then:
          - !Return
            value: 1
        else:
          - !CallFunction
            name: factorial
            args: ["{{n}} - 1"]
          - !MathOp
            operation: multiply
            args: ["{{n}}", "{{function_result}}"]
          - !Return
            value: "{{math_result}}"
  
  - !CallFunction
    name: factorial
    args: ["{{n}}"]
  
  - !Return
    value:
      input: "{{n}}"
      result: "{{function_result}}"
```

### Function Debugging Tips

1. **Add Logging**: Use `!Log` operations in functions
2. **Test Small**: Test with simple inputs first
3. **Check Parameters**: Log parameters at function start
4. **Isolate Issues**: Test functions independently

---

## 🎉 Congratulations!

You now know how to:
- ✅ Create reusable functions
- ✅ Pass parameters to functions
- ✅ Call functions from anywhere
- ✅ Use recursion for complex algorithms
- ✅ Build real-world applications with functions

**Next Steps:**
- Read the [Complete Functions Guide](06-functions.md)
- Explore [Expression Syntax](../backend/EXPRESSION_GUIDE.md)
- Try the [Advanced Examples](../DYNAMIC_ROUTES_COMPLETE_GUIDE.md)

**Happy coding with Worpen! 🚀**