# 2. ساختار پایه

## آناتومی یک Dynamic Route

هر route شامل metadata و logic است:

```json
{
  "name": "نام Route",
  "path": "/api/path",
  "method": "POST",
  "description": "توضیحات",
  "logic": [
    // آرایه‌ای از operations
  ]
}
```

## فیلدهای اصلی

### `name` (الزامی)

نام منحصربه‌فرد route برای شناسایی:

```json
{
  "name": "User Registration API"
}
```

### `path` (الزامی)

مسیر URL برای دسترسی:

```json
{
  "path": "/api/users/register"
}
```

**نکات مهم:**
- باید با `/` شروع شود
- می‌تواند شامل path parameters باشد: `/api/users/:id`
- Case-sensitive است

### `method` (الزامی)

متد HTTP:

```json
{
  "method": "POST"
}
```

**مقادیر مجاز:**
- `GET`
- `POST`
- `PUT`
- `PATCH`
- `DELETE`

### `description` (اختیاری)

توضیحات route:

```json
{
  "description": "ثبت‌نام کاربر جدید در سیستم"
}
```

### `logic` (الزامی)

آرایه‌ای از operations که به ترتیب اجرا می‌شوند:

```json
{
  "logic": [
    {
      "set": {
        "var": "message",
        "value": "سلام"
      }
    },
    {
      "return": {
        "value": {"message": "{{message}}"}
      }
    }
  ]
}
```

## ساختار Operation

هر operation یک JSON object با یک کلید اصلی است:

```json
{
  "operation_type": {
    "param1": "value1",
    "param2": "value2"
  }
}
```

### مثال: Set Operation

```json
{
  "set": {
    "var": "username",
    "value": "Ali"
  }
}
```

### مثال: If Operation

```json
{
  "if": {
    "condition": "{{age}} >= 18",
    "then": [
      {"set": {"var": "status", "value": "adult"}}
    ],
    "otherwise": [
      {"set": {"var": "status", "value": "minor"}}
    ]
  }
}
```

## انواع Operations

### 1. عملیات پایه

| Operation | کاربرد | مثال |
|-----------|--------|------|
| `set` | تعیین مقدار متغیر | `{"set": {"var": "x", "value": 10}}` |
| `get` | خواندن متغیر | `{"get": {"var": "x"}}` |
| `return` | بازگشت نتیجه | `{"return": {"value": "{{x}}"}}` |
| `log` | ثبت لاگ | `{"log": {"level": "info", "message": "سلام"}}` |

### 2. ساختارهای کنترلی

| Operation | کاربرد |
|-----------|--------|
| `if` | شرط If/Else |
| `switch` | Switch/Case |
| `try` | مدیریت خطا |
| `throw` | پرتاب خطا |

### 3. حلقه‌ها

| Operation | کاربرد |
|-----------|--------|
| `while` | حلقه While |
| `loop` | حلقه ForEach |
| `break` | خروج از حلقه |
| `continue` | ادامه به iteration بعدی |

### 4. توابع

| Operation | کاربرد |
|-----------|--------|
| `define_function` | تعریف تابع |
| `call_function` | فراخوانی تابع |

### 5. عملیات ریاضی

| Operation | کاربرد |
|-----------|--------|
| `math_op` | عملیات ریاضی (sum, multiply, pow, ...) |

### 6. عملیات رشته‌ای

| Operation | کاربرد |
|-----------|--------|
| `string_op` | عملیات رشته‌ای (upper, split, ...) |

### 7. عملیات تاریخ

| Operation | کاربرد |
|-----------|--------|
| `date_op` | عملیات تاریخ (now, timestamp, ...) |

### 8. Parallel

| Operation | کاربرد |
|-----------|--------|
| `parallel` | اجرای موازی tasks |

## Execution Context

هر route در یک context اجرا می‌شود که شامل:

### 1. Variables

متغیرهای تعریف‌شده در route:

```json
{
  "set": {"var": "counter", "value": 0}
}
```

### 2. Request Data

داده‌های ارسالی در request:

```json
// POST body:
{
  "username": "ali",
  "age": 25
}

// در route قابل دسترسی:
"{{username}}"  // "ali"
"{{age}}"       // 25
```

### 3. Special Variables

متغیرهای ویژه سیستم:

```json
"{{math_result}}"    // نتیجه آخرین math_op
"{{string_result}}"  // نتیجه آخرین string_op
"{{date_result}}"    // نتیجه آخرین date_op
"{{function_result}}" // نتیجه آخرین call_function
```

## Variable Resolution

متغیرها با syntax `{{variable_name}}` resolve می‌شوند:

### ساده

```json
{
  "set": {"var": "name", "value": "Ali"},
  "return": {"value": "{{name}}"}
}
// خروجی: "Ali"
```

### Nested Path

```json
{
  "set": {
    "var": "user",
    "value": {"name": "Ali", "age": 25}
  },
  "return": {
    "value": "{{user.name}}"
  }
}
// خروجی: "Ali"
```

### در String

```json
{
  "set": {"var": "name", "value": "Ali"},
  "log": {
    "message": "سلام {{name}}!"
  }
}
// لاگ: "سلام Ali!"
```

## Response Structure

پاسخ همیشه به این فرمت است:

```json
{
  "success": true,
  "result": {
    // نتیجه return
  },
  "error": null,
  "execution_time_ms": 42,
  "steps_executed": [
    "Route found: My Route (/api/test)",
    "Execution context created",
    "Step 1: Executing operation",
    "Set variable 'x' = 10",
    "Step 2: Executing operation",
    "Returning value: {\"x\":10}"
  ]
}
```

## مثال کامل: محاسبه قیمت با تخفیف

```json
{
  "name": "Calculate Price",
  "path": "/api/calculate-price",
  "method": "POST",
  "description": "محاسبه قیمت نهایی با تخفیف",
  "logic": [
    {
      "log": {
        "level": "info",
        "message": "شروع محاسبه قیمت"
      }
    },
    {
      "set": {
        "var": "base_price",
        "value": "{{price}}"
      }
    },
    {
      "set": {
        "var": "quantity",
        "value": "{{quantity}}"
      }
    },
    {
      "math_op": {
        "operation": "multiply",
        "args": ["{{base_price}}", "{{quantity}}"]
      }
    },
    {
      "set": {
        "var": "subtotal",
        "value": "{{math_result}}"
      }
    },
    {
      "if": {
        "condition": "{{subtotal}} > 1000",
        "then": [
          {
            "set": {
              "var": "discount_percent",
              "value": 20
            }
          }
        ],
        "otherwise": [
          {
            "if": {
              "condition": "{{subtotal}} > 500",
              "then": [
                {"set": {"var": "discount_percent", "value": 10}}
              ],
              "otherwise": [
                {"set": {"var": "discount_percent", "value": 0}}
              ]
            }
          }
        ]
      }
    },
    {
      "math_op": {
        "operation": "multiply",
        "args": ["{{subtotal}}", "{{discount_percent}}"]
      }
    },
    {
      "math_op": {
        "operation": "divide",
        "args": ["{{math_result}}", 100]
      }
    },
    {
      "set": {
        "var": "discount_amount",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{subtotal}}", "{{discount_amount}}"]
      }
    },
    {
      "set": {
        "var": "final_price",
        "value": "{{math_result}}"
      }
    },
    {
      "return": {
        "value": {
          "base_price": "{{base_price}}",
          "quantity": "{{quantity}}",
          "subtotal": "{{subtotal}}",
          "discount_percent": "{{discount_percent}}",
          "discount_amount": "{{discount_amount}}",
          "final_price": "{{final_price}}"
        }
      }
    }
  ]
}
```

### استفاده:

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes/{route_id}/execute \
  -H "Content-Type: application/json" \
  -d '{"price": 150, "quantity": 8}'
```

### خروجی:

```json
{
  "success": true,
  "result": {
    "base_price": "150",
    "quantity": "8",
    "subtotal": "1200.0",
    "discount_percent": "20",
    "discount_amount": "240.0",
    "final_price": "960.0"
  }
}
```

## Best Practices

### 1. نام‌گذاری واضح

```json
// ❌ بد
{"var": "x"}

// ✅ خوب
{"var": "user_age"}
```

### 2. استفاده از Log برای Debug

```json
{
  "log": {
    "level": "debug",
    "message": "مقدار counter: {{counter}}"
  }
}
```

### 3. مدیریت خطا

```json
{
  "try": {
    "body": [
      // عملیات
    ],
    "catch": [
      {
        "log": {
          "level": "error",
          "message": "خطا: {{error.message}}"
        }
      }
    ]
  }
}
```

### 4. تقسیم منطق پیچیده

```json
// به جای یک operation بزرگ، از توابع استفاده کنید
{
  "define_function": {
    "name": "calculate_discount",
    "params": ["amount"],
    "body": [...]
  }
}
```

## مراحل بعدی

- [متغیرها و عملیات پایه](03-variables-basics.md)
- [ساختارهای کنترلی](04-control-flow.md)
- [نمونه‌های کامل](12-complete-examples.md)
