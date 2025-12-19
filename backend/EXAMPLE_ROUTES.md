# 📚 Example YAML Routes

Real-world examples of YAML Dynamic Routes using expression syntax.

## Table of Contents
- [Simple Examples](#simple-examples)
- [E-commerce](#e-commerce)
- [User Management](#user-management)
- [API Integrations](#api-integrations)
- [Data Processing](#data-processing)
- [Advanced Patterns](#advanced-patterns)

---

## Simple Examples

### Hello World

```yaml
name: hello_world
path: /hello
method: GET

response:
  message: Hello, World!
  timestamp: "{{now()}}"
```

### Echo Service

```yaml
name: echo
path: /echo
method: POST

response:
  you_said: "{{message}}"
  length: "{{message | length}}"
  uppercase: "{{message | upper}}"
  reversed: "{{message | reverse}}"
```

### Calculator

```yaml
name: calculator
path: /calculate
method: POST

response:
  operation: add
  a: "{{a}}"
  b: "{{b}}"
  result: "{{a + b}}"
  doubled: "{{(a + b) * 2}}"
  squared: "{{(a + b) ** 2}}"
```

---

## E-commerce

### Product Price Calculator

```yaml
name: product_price
path: /api/products/price
method: POST

response:
  product:
    id: "{{product_id}}"
    name: "{{product_name}}"
    base_price: "{{price}}"
    quantity: "{{quantity}}"
  
  pricing:
    subtotal: "{{price * quantity}}"
    discount: "{{quantity >= 10 ? price * quantity * 0.1 : 0}}"
    after_discount: "{{price * quantity * (quantity >= 10 ? 0.9 : 1)}}"
    tax: "{{price * quantity * (quantity >= 10 ? 0.9 : 1) * 0.15}}"
    shipping: "{{quantity > 5 ? 0 : 10}}"
    total: "{{price * quantity * (quantity >= 10 ? 0.9 : 1) * 1.15 + (quantity > 5 ? 0 : 10)}}"
  
  info:
    free_shipping: "{{quantity > 5}}"
    bulk_discount_applied: "{{quantity >= 10}}"
    savings: "{{quantity >= 10 ? price * quantity * 0.1 : 0}}"
```

### Checkout with Tiered Pricing

```yaml
name: checkout
path: /api/checkout
method: POST

response:
  order:
    id: "{{order_id}}"
    customer_email: "{{email | trim | lower}}"
    items_count: "{{items | length}}"
  
  pricing:
    subtotal: "{{subtotal}}"
    
    # Tiered discount based on customer level
    discount_rate: "{{
      customer_level == 'platinum' ? 0.25 :
      (customer_level == 'gold' ? 0.2 :
      (customer_level == 'silver' ? 0.15 :
      (customer_level == 'bronze' ? 0.1 : 0.05)))
    }}"
    
    discount_amount: "{{subtotal * (
      customer_level == 'platinum' ? 0.25 :
      (customer_level == 'gold' ? 0.2 :
      (customer_level == 'silver' ? 0.15 :
      (customer_level == 'bronze' ? 0.1 : 0.05)))
    )}}"
    
    after_discount: "{{subtotal - (subtotal * (
      customer_level == 'platinum' ? 0.25 :
      (customer_level == 'gold' ? 0.2 :
      (customer_level == 'silver' ? 0.15 :
      (customer_level == 'bronze' ? 0.1 : 0.05)))
    ))}}"
    
    tax: "{{(subtotal - (subtotal * (
      customer_level == 'platinum' ? 0.25 :
      (customer_level == 'gold' ? 0.2 :
      (customer_level == 'silver' ? 0.15 :
      (customer_level == 'bronze' ? 0.1 : 0.05)))
    ))) * 0.1}}"
    
    shipping: "{{subtotal > 100 ? 0 : 15}}"
    
    total: "{{(subtotal - (subtotal * (
      customer_level == 'platinum' ? 0.25 :
      (customer_level == 'gold' ? 0.2 :
      (customer_level == 'silver' ? 0.15 :
      (customer_level == 'bronze' ? 0.1 : 0.05)))
    ))) * 1.1 + (subtotal > 100 ? 0 : 15)}}"
  
  benefits:
    free_shipping: "{{subtotal > 100}}"
    loyalty_points_earned: "{{(subtotal / 10) | floor}}"
    next_tier: "{{
      customer_level == 'platinum' ? 'Maximum tier reached!' :
      (customer_level == 'gold' ? 'Upgrade to Platinum at $5000' :
      (customer_level == 'silver' ? 'Upgrade to Gold at $2000' :
      (customer_level == 'bronze' ? 'Upgrade to Silver at $500' :
      'Upgrade to Bronze at $100')))
    }}"
```

### Shopping Cart Summary

```yaml
name: cart_summary
path: /api/cart/summary
method: POST

response:
  cart:
    items: "{{items}}"
    item_count: "{{items | length}}"
    first_item: "{{items | first}}"
    last_item: "{{items | last}}"
  
  totals:
    subtotal: "{{subtotal}}"
    estimated_tax: "{{subtotal * 0.1}}"
    estimated_total: "{{subtotal * 1.1}}"
  
  recommendations:
    add_more_for_free_shipping: "{{subtotal < 50 ? 50 - subtotal : 0}}"
    eligible_for_discount: "{{subtotal > 100}}"
    savings_potential: "{{subtotal > 100 ? subtotal * 0.15 : 0}}"
```

---

## User Management

### User Registration

```yaml
name: user_registration
path: /api/users/register
method: POST

response:
  success: true
  user:
    email: "{{email | trim | lower}}"
    username: "{{username | trim}}"
    display_name: "{{first_name | trim | capitalize}} {{last_name | trim | capitalize}}"
    age: "{{age}}"
    is_adult: "{{age >= 18}}"
    account_type: "{{is_premium ? 'Premium' : 'Free'}}"
    verification_required: "{{age < 18}}"
```

### User Profile

```yaml
name: user_profile
path: /api/users/{id}
method: GET

response:
  status: success
  user:
    id: "{{user_id}}"
    email: "{{email | trim | lower}}"
    username: "{{username}}"
    full_name: "{{first_name | capitalize}} {{last_name | capitalize}}"
    member_since: "{{created_at}}"
    
  stats:
    total_orders: "{{orders | length}}"
    total_spent: "{{total_spent}}"
    average_order: "{{orders | length > 0 ? total_spent / (orders | length) | round : 0}}"
    loyalty_tier: "{{
      orders | length > 20 ? 'platinum' :
      (orders | length > 10 ? 'gold' :
      (orders | length > 5 ? 'silver' : 'bronze'))
    }}"
  
  preferences:
    newsletter: "{{newsletter_enabled}}"
    notifications: "{{notifications_enabled}}"
```

### Password Strength Checker

```yaml
name: password_check
path: /api/auth/check-password
method: POST

response:
  password_length: "{{password | length}}"
  
  strength:
    is_long_enough: "{{(password | length) >= 8}}"
    is_very_strong: "{{(password | length) >= 16}}"
    
  rating: "{{
    (password | length) >= 16 ? 'Very Strong' :
    ((password | length) >= 12 ? 'Strong' :
    ((password | length) >= 8 ? 'Medium' : 'Weak'))
  }}"
  
  meets_requirements: "{{(password | length) >= 8}}"
```

---

## API Integrations

### Weather API Response

```yaml
name: weather
path: /api/weather
method: GET

response:
  location:
    city: "{{city_name | capitalize}}"
    country: "{{country_code | upper}}"
  
  temperature:
    celsius: "{{temp_c}}"
    fahrenheit: "{{temp_c * 1.8 + 32 | round}}"
    feels_like: "{{feels_like_c}}"
    
  conditions:
    description: "{{weather_desc | capitalize}}"
    is_hot: "{{temp_c > 30}}"
    is_cold: "{{temp_c < 10}}"
    is_comfortable: "{{temp_c >= 18 && temp_c <= 25}}"
    
  advice: "{{
    temp_c > 30 ? 'Stay hydrated! It\\'s hot today.' :
    (temp_c < 10 ? 'Bundle up! It\\'s cold outside.' :
    'Perfect weather for outdoor activities!')
  }}"
```

### Currency Converter

```yaml
name: currency_converter
path: /api/convert
method: POST

response:
  from:
    amount: "{{amount}}"
    currency: "{{from_currency | upper}}"
  
  to:
    amount: "{{amount * exchange_rate | round}}"
    currency: "{{to_currency | upper}}"
  
  exchange:
    rate: "{{exchange_rate}}"
    inverse_rate: "{{1 / exchange_rate | round}}"
    
  info:
    is_favorable: "{{exchange_rate > 1}}"
    markup_percent: "{{((exchange_rate / base_rate - 1) * 100) | round}}%"
```

### Geolocation Distance

```yaml
name: geo_distance
path: /api/geo/distance
method: POST

response:
  from:
    lat: "{{lat1}}"
    lon: "{{lon1}}"
    name: "{{location1_name | capitalize}}"
  
  to:
    lat: "{{lat2}}"
    lon: "{{lon2}}"
    name: "{{location2_name | capitalize}}"
  
  distance:
    kilometers: "{{distance_km}}"
    miles: "{{distance_km * 0.621371 | round}}"
    meters: "{{distance_km * 1000 | round}}"
    
  travel:
    is_nearby: "{{distance_km < 10}}"
    is_far: "{{distance_km > 100}}"
    driving_time_hours: "{{distance_km / 60 | round}}"
```

---

## Data Processing

### Array Statistics

```yaml
name: array_stats
path: /api/stats
method: POST

response:
  input:
    numbers: "{{numbers}}"
    count: "{{numbers | length}}"
  
  values:
    first: "{{numbers | first}}"
    last: "{{numbers | last}}"
    max: "{{max(numbers)}}"
    min: "{{min(numbers)}}"
  
  operations:
    sorted: "{{numbers | sort}}"
    reversed: "{{numbers | reverse}}"
    unique: "{{numbers | unique}}"
```

### String Manipulation

```yaml
name: string_processor
path: /api/strings/process
method: POST

response:
  original: "{{text}}"
  
  transformations:
    uppercase: "{{text | upper}}"
    lowercase: "{{text | lower}}"
    trimmed: "{{text | trim}}"
    capitalized: "{{text | trim | capitalize}}"
    reversed: "{{text | reverse}}"
    
  info:
    length: "{{text | length}}"
    trimmed_length: "{{text | trim | length}}"
    is_empty: "{{text | trim | length == 0}}"
    has_content: "{{text | trim | length > 0}}"
    
  words:
    split: "{{text | trim | split(' ')}}"
    word_count: "{{text | trim | split(' ') | length}}"
```

### CSV to Array

```yaml
name: csv_parser
path: /api/parse/csv
method: POST

response:
  original: "{{csv_string}}"
  
  parsed:
    items: "{{csv_string | split(',')}}"
    count: "{{csv_string | split(',') | length}}"
    first: "{{csv_string | split(',') | first | trim}}"
    last: "{{csv_string | split(',') | last | trim}}"
    
  processed:
    trimmed: "{{csv_string | split(',') | unique}}"
    sorted: "{{csv_string | split(',') | sort}}"
    unique: "{{csv_string | split(',') | unique}}"
    joined: "{{csv_string | split(',') | unique | sort | join('; ')}}"
```

---

## Advanced Patterns

### Multi-Step Calculation

```yaml
name: loan_calculator
path: /api/loans/calculate
method: POST

response:
  loan:
    principal: "{{principal}}"
    annual_rate: "{{annual_rate}}"
    years: "{{years}}"
  
  calculated:
    monthly_rate: "{{annual_rate / 12 / 100}}"
    num_payments: "{{years * 12}}"
    
    # Monthly payment formula: P * (r * (1 + r)^n) / ((1 + r)^n - 1)
    monthly_payment: "{{(principal * (annual_rate / 12 / 100) * ((1 + (annual_rate / 12 / 100)) ** (years * 12))) / (((1 + (annual_rate / 12 / 100)) ** (years * 12)) - 1) | round}}"
    
    total_paid: "{{((principal * (annual_rate / 12 / 100) * ((1 + (annual_rate / 12 / 100)) ** (years * 12))) / (((1 + (annual_rate / 12 / 100)) ** (years * 12)) - 1) | round) * years * 12}}"
    
    total_interest: "{{(((principal * (annual_rate / 12 / 100) * ((1 + (annual_rate / 12 / 100)) ** (years * 12))) / (((1 + (annual_rate / 12 / 100)) ** (years * 12)) - 1) | round) * years * 12) - principal}}"
    
  affordability:
    is_affordable: "{{monthly_income * 0.3 > ((principal * (annual_rate / 12 / 100) * ((1 + (annual_rate / 12 / 100)) ** (years * 12))) / (((1 + (annual_rate / 12 / 100)) ** (years * 12)) - 1) | round)}}"
    debt_to_income_ratio: "{{(((principal * (annual_rate / 12 / 100) * ((1 + (annual_rate / 12 / 100)) ** (years * 12))) / (((1 + (annual_rate / 12 / 100)) ** (years * 12)) - 1) | round) / monthly_income * 100) | round}}%"
```

### Nested Conditional Logic

```yaml
name: insurance_quote
path: /api/insurance/quote
method: POST

response:
  applicant:
    age: "{{age}}"
    gender: "{{gender | lower}}"
    smoker: "{{is_smoker}}"
    health: "{{health_rating | lower}}"
  
  base_premium: "{{
    age < 25 ? 150 :
    (age < 35 ? 120 :
    (age < 45 ? 100 :
    (age < 55 ? 130 :
    (age < 65 ? 180 : 250))))
  }}"
  
  multipliers:
    smoker_multiplier: "{{is_smoker ? 1.5 : 1.0}}"
    health_multiplier: "{{
      health_rating == 'excellent' ? 0.8 :
      (health_rating == 'good' ? 1.0 :
      (health_rating == 'fair' ? 1.3 :
      (health_rating == 'poor' ? 1.8 : 1.5)))
    }}"
  
  monthly_premium: "{{
    (age < 25 ? 150 :
    (age < 35 ? 120 :
    (age < 45 ? 100 :
    (age < 55 ? 130 :
    (age < 65 ? 180 : 250))))) *
    (is_smoker ? 1.5 : 1.0) *
    (health_rating == 'excellent' ? 0.8 :
    (health_rating == 'good' ? 1.0 :
    (health_rating == 'fair' ? 1.3 :
    (health_rating == 'poor' ? 1.8 : 1.5)))) | round
  }}"
  
  annual_premium: "{{
    ((age < 25 ? 150 :
    (age < 35 ? 120 :
    (age < 45 ? 100 :
    (age < 55 ? 130 :
    (age < 65 ? 180 : 250))))) *
    (is_smoker ? 1.5 : 1.0) *
    (health_rating == 'excellent' ? 0.8 :
    (health_rating == 'good' ? 1.0 :
    (health_rating == 'fair' ? 1.3 :
    (health_rating == 'poor' ? 1.8 : 1.5)))) | round) * 12
  }}"
  
  risk_assessment: "{{
    is_smoker && age > 50 && health_rating != 'excellent' ? 'High Risk' :
    (is_smoker || age > 60 ? 'Medium Risk' :
    (health_rating == 'poor' ? 'Medium Risk' : 'Low Risk'))
  }}"
```

### Dynamic Notification Builder

```yaml
name: notification_builder
path: /api/notifications/build
method: POST

response:
  user:
    name: "{{user_name | trim | capitalize}}"
    email: "{{email | trim | lower}}"
  
  notification:
    type: "{{notification_type | lower}}"
    
    subject: "{{
      notification_type == 'order_shipped' ? 'Your order has been shipped!' :
      (notification_type == 'order_delivered' ? 'Your order has been delivered!' :
      (notification_type == 'password_reset' ? 'Password Reset Request' :
      (notification_type == 'account_created' ? 'Welcome to our platform!' :
      'System Notification')))
    }}"
    
    message: "{{
      notification_type == 'order_shipped' ? 'Hello {{user_name | capitalize}}, your order #{{order_id}} has been shipped and will arrive in {{delivery_days}} days.' :
      (notification_type == 'order_delivered' ? 'Hello {{user_name | capitalize}}, your order #{{order_id}} has been delivered. Enjoy your purchase!' :
      (notification_type == 'password_reset' ? 'Hello {{user_name | capitalize}}, click the link below to reset your password. This link expires in 24 hours.' :
      (notification_type == 'account_created' ? 'Welcome {{user_name | capitalize}}! Thank you for joining us. Your account is now active.' :
      'Hello {{user_name | capitalize}}, this is a system notification.')))
    }}"
    
    priority: "{{
      notification_type == 'password_reset' ? 'high' :
      (notification_type == 'order_delivered' ? 'medium' :
      (notification_type == 'account_created' ? 'medium' : 'low'))
    }}"
    
    send_email: "{{
      notification_type == 'password_reset' || 
      notification_type == 'order_shipped' ||
      notification_type == 'account_created'
    }}"
    
    send_sms: "{{notification_type == 'password_reset'}}"
```

---

## Tips & Best Practices

### 1. Use Comments for Complex Logic

```yaml
name: complex_calculation
path: /api/calculate
method: POST

response:
  # Calculate discount based on customer tier
  discount_rate: "{{
    tier == 'platinum' ? 0.25 :    # 25% for platinum
    (tier == 'gold' ? 0.2 :         # 20% for gold
    (tier == 'silver' ? 0.15 : 0))  # 15% for silver, 0% for others
  }}"
```

### 2. Break Down Complex Expressions

```yaml
# ✅ Good: Clear and maintainable
response:
  subtotal: "{{price * quantity}}"
  discount: "{{subtotal * discount_rate}}"
  after_discount: "{{subtotal - discount}}"
  tax: "{{after_discount * 0.1}}"
  total: "{{after_discount + tax}}"

# ❌ Avoid: All in one expression
response:
  total: "{{price * quantity * (1 - discount_rate) * 1.1}}"
```

### 3. Normalize Input Early

```yaml
response:
  # Normalize inputs first
  email_clean: "{{email | trim | lower}}"
  username_clean: "{{username | trim}}"
  
  # Use normalized values
  user:
    email: "{{email_clean}}"
    username: "{{username_clean}}"
```

### 4. Use Descriptive Field Names

```yaml
# ✅ Good
response:
  monthly_payment: "{{loan_amount * monthly_rate}}"
  total_with_tax: "{{subtotal * 1.1}}"
  
# ❌ Avoid
response:
  result1: "{{a * b}}"
  result2: "{{x * 1.1}}"
```

---

## Performance Considerations

- ✅ **Cache expensive calculations** (calculate once, reuse)
- ✅ **Use short-circuit operators** (`&&`, `||`)
- ✅ **Avoid deep nesting** (max 2-3 levels of ternary)
- ✅ **Filter chains are efficient** (pipe operations)
- ❌ **Don't repeat complex expressions** (calculate once)

---

## Next Steps

- Read [EXPRESSION_GUIDE.md](./EXPRESSION_GUIDE.md) for complete syntax reference
- Check [documentation/](../documentation/) for more advanced topics
- Try [examples/](../examples/) for simple starter routes
- Join community for support and best practices

Happy routing! 🚀
