# 3. متغیرها و عملیات پایه

## تعریف متغیر (Set)

### Syntax

```json
{
  "set": {
    "var": "variable_name",
    "value": value
  }
}
```

### انواع مقدار

#### 1. Number

```json
{
  "set": {"var": "age", "value": 25}
}
```

#### 2. String

```json
{
  "set": {"var": "name", "value": "علی"}
}
```

#### 3. Boolean

```json
{
  "set": {"var": "is_active", "value": true}
}
```

#### 4. Array

```json
{
  "set": {
    "var": "numbers",
    "value": [1, 2, 3, 4, 5]
  }
}
```

#### 5. Object

```json
{
  "set": {
    "var": "user",
    "value": {
      "name": "علی",
      "age": 25,
      "email": "ali@example.com"
    }
  }
}
```

#### 6. Null

```json
{
  "set": {"var": "result", "value": null}
}
```

### استفاده از متغیرهای دیگر

```json
{
  "logic": [
    {"set": {"var": "first_name", "value": "علی"}},
    {"set": {"var": "last_name", "value": "احمدی"}},
    {
      "set": {
        "var": "full_name",
        "value": "{{first_name}} {{last_name}}"
      }
    }
  ]
}
// full_name = "علی احمدی"
```

## خواندن متغیر (Get)

```json
{
  "get": {
    "var": "username"
  }
}
```

**نکته:** معمولاً نیازی به `get` نیست، فقط از `{{variable}}` استفاده کنید.

## Variable Resolution

### Syntax پایه

```json
"{{variable_name}}"
```

### دسترسی به Request Data

```json
// POST /api/route
// Body: {"username": "ali", "age": 25}

{
  "set": {
    "var": "user",
    "value": "{{username}}"
  }
}
// user = "ali"
```

### Nested Path Access

```json
{
  "set": {
    "var": "user",
    "value": {
      "profile": {
        "name": "علی",
        "age": 25
      }
    }
  },
  "log": {
    "message": "نام: {{user.profile.name}}"
  }
}
// لاگ: "نام: علی"
```

### Array Index Access

```json
{
  "set": {
    "var": "numbers",
    "value": [10, 20, 30]
  },
  "log": {
    "message": "اولین عدد: {{numbers.0}}"
  }
}
// لاگ: "اولین عدد: 10"
```

### متغیرهای ویژه

```json
// بعد از math_op
"{{math_result}}"

// بعد از string_op
"{{string_result}}"

// بعد از date_op
"{{date_result}}"

// بعد از call_function
"{{function_result}}"
```

## Return

```json
{
  "return": {
    "value": expression
  }
}
```

### مثال‌ها

#### Return ساده

```json
{
  "return": {
    "value": "سلام دنیا"
  }
}
```

#### Return Object

```json
{
  "return": {
    "value": {
      "status": "success",
      "message": "عملیات موفق"
    }
  }
}
```

#### Return با متغیرها

```json
{
  "logic": [
    {"set": {"var": "name", "value": "علی"}},
    {"set": {"var": "age", "value": 25}},
    {
      "return": {
        "value": {
          "name": "{{name}}",
          "age": "{{age}}"
        }
      }
    }
  ]
}
```

**نکته:** `return` execution را متوقف می‌کند و بقیه operations اجرا نمی‌شوند.

## Log

```json
{
  "log": {
    "level": "info|warn|error|debug",
    "message": "پیام لاگ"
  }
}
```

### سطوح Log

| Level | کاربرد |
|-------|--------|
| `debug` | اطلاعات debugging |
| `info` | اطلاعات عمومی |
| `warn` | هشدارها |
| `error` | خطاها |

### مثال

```json
{
  "logic": [
    {
      "log": {
        "level": "info",
        "message": "شروع پردازش"
      }
    },
    {"set": {"var": "counter", "value": 0}},
    {
      "log": {
        "level": "debug",
        "message": "Counter = {{counter}}"
      }
    },
    {
      "log": {
        "level": "warn",
        "message": "هشدار: مقدار کم است"
      }
    }
  ]
}
```

## Sleep (توقف موقت)

```json
{
  "sleep": {
    "duration_ms": 1000
  }
}
```

**نکته:** `duration_ms` به میلی‌ثانیه است (1000ms = 1 ثانیه)

### مثال

```json
{
  "logic": [
    {
      "log": {
        "message": "شروع"
      }
    },
    {
      "sleep": {
        "duration_ms": 2000
      }
    },
    {
      "log": {
        "message": "بعد از 2 ثانیه"
      }
    }
  ]
}
```

## مثال کامل: سیستم امتیازدهی

```json
{
  "name": "Calculate Score",
  "path": "/api/score",
  "method": "POST",
  "description": "محاسبه امتیاز کاربر",
  "logic": [
    {
      "log": {
        "level": "info",
        "message": "شروع محاسبه امتیاز برای {{username}}"
      }
    },
    {
      "set": {
        "var": "username",
        "value": "{{username}}"
      }
    },
    {
      "set": {
        "var": "tasks_completed",
        "value": "{{tasks_completed}}"
      }
    },
    {
      "set": {
        "var": "bonus_points",
        "value": "{{bonus_points}}"
      }
    },
    {
      "math_op": {
        "operation": "multiply",
        "args": ["{{tasks_completed}}", 10]
      }
    },
    {
      "set": {
        "var": "base_score",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "sum",
        "args": ["{{base_score}}", "{{bonus_points}}"]
      }
    },
    {
      "set": {
        "var": "total_score",
        "value": "{{math_result}}"
      }
    },
    {
      "if": {
        "condition": "{{total_score}} >= 100",
        "then": [
          {"set": {"var": "rank", "value": "Gold"}},
          {"set": {"var": "reward", "value": 1000}}
        ],
        "otherwise": [
          {
            "if": {
              "condition": "{{total_score}} >= 50",
              "then": [
                {"set": {"var": "rank", "value": "Silver"}},
                {"set": {"var": "reward", "value": 500}}
              ],
              "otherwise": [
                {"set": {"var": "rank", "value": "Bronze"}},
                {"set": {"var": "reward", "value": 100}}
              ]
            }
          }
        ]
      }
    },
    {
      "log": {
        "level": "info",
        "message": "کاربر {{username}}: امتیاز={{total_score}}, رتبه={{rank}}"
      }
    },
    {
      "return": {
        "value": {
          "username": "{{username}}",
          "tasks_completed": "{{tasks_completed}}",
          "bonus_points": "{{bonus_points}}",
          "base_score": "{{base_score}}",
          "total_score": "{{total_score}}",
          "rank": "{{rank}}",
          "reward": "{{reward}}"
        }
      }
    }
  ]
}
```

### استفاده

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes/{route_id}/execute \
  -H "Content-Type: application/json" \
  -d '{
    "username": "ali123",
    "tasks_completed": 12,
    "bonus_points": 25
  }'
```

### خروجی

```json
{
  "success": true,
  "result": {
    "username": "ali123",
    "tasks_completed": "12",
    "bonus_points": "25",
    "base_score": "120.0",
    "total_score": "145.0",
    "rank": "Gold",
    "reward": "1000"
  }
}
```

## نکات مهم

### 1. Type Coercion

سیستم automatically تبدیل type می‌کند:

```json
{
  "set": {"var": "age", "value": "25"}  // String
}
// در math_op به Number تبدیل می‌شود
```

### 2. Null Safety

```json
{
  "set": {"var": "maybe_null", "value": null}
}
// استفاده از {{maybe_null}} خطا نمی‌دهد
// فقط "null" برمی‌گرداند
```

### 3. Scope

متغیرها در تمام route قابل دسترسی هستند:

```json
{
  "logic": [
    {"set": {"var": "x", "value": 10}},
    {
      "if": {
        "condition": "{{x}} > 5",
        "then": [
          {"set": {"var": "y", "value": 20}}
        ]
      }
    },
    // y اینجا هم قابل دسترسی است
    {"log": {"message": "y = {{y}}"}}
  ]
}
```

### 4. Shadowing

می‌توانید متغیر را دوباره set کنید:

```json
{
  "set": {"var": "counter", "value": 0}
},
{
  "set": {"var": "counter", "value": 10}
}
// counter حالا 10 است
```

## Common Patterns

### 1. Configuration Object

```json
{
  "set": {
    "var": "config",
    "value": {
      "max_retries": 3,
      "timeout_ms": 5000,
      "api_key": "secret"
    }
  }
}
```

### 2. Counter

```json
{
  "set": {"var": "counter", "value": 0}
},
{
  "while": {
    "condition": "{{counter}} < 10",
    "body": [
      {
        "math_op": {
          "operation": "sum",
          "args": ["{{counter}}", 1]
        }
      },
      {"set": {"var": "counter", "value": "{{math_result}}"}}
    ]
  }
}
```

### 3. Flag Variables

```json
{
  "set": {"var": "found", "value": false}
},
{
  "if": {
    "condition": "{{item.id}} == {{target_id}}",
    "then": [
      {"set": {"var": "found", "value": true}}
    ]
  }
}
```

## مراحل بعدی

- [ساختارهای کنترلی](04-control-flow.md) - If/Else, Switch
- [حلقه‌ها](05-loops.md) - While, ForEach
- [عملیات ریاضی](07-math-operations.md)
