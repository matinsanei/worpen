# توابع سفارشی (Custom Functions)

توابع به شما امکان می‌دهند تا کدهای تکراری را در یک تابع قرار دهید و در جاهای مختلف فراخوانی کنید.

## فهرست مطالب

- [تعریف تابع](#تعریف-تابع)
- [فراخوانی تابع](#فراخوانی-تابع)
- [پارامترها و Return](#پارامترها-و-return)
- [نمونه‌های پیشرفته](#نمونه‌های-پیشرفته)

---

## تعریف تابع

### ساختار پایه

```json
{
  "define_function": {
    "name": "نام_تابع",
    "params": ["پارامتر۱", "پارامتر۲"],
    "body": [
      // عملیات‌های تابع
    ]
  }
}
```

### مثال ۱: تابع ساده بدون پارامتر

```json
{
  "name": "Greeting Function",
  "path": "/greeting",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "say_hello",
        "params": [],
        "body": [
          {
            "set": {
              "var": "message",
              "value": "سلام! خوش آمدید"
            }
          },
          {
            "return": {
              "value": "{{message}}"
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "say_hello",
        "args": []
      }
    },
    {
      "return": {
        "value": {
          "greeting": "{{function_result}}"
        }
      }
    }
  ]
}
```

### مثال ۲: تابع با پارامترها

```json
{
  "name": "Add Two Numbers",
  "path": "/add",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "add",
        "params": ["a", "b"],
        "body": [
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{a}}", "{{b}}"]
            }
          },
          {
            "return": {
              "value": "{{math_result}}"
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "add",
        "args": [10, 20]
      }
    },
    {
      "return": {
        "value": {
          "result": "{{function_result}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "result": 30
}
```

---

## فراخوانی تابع

### ساختار پایه

```json
{
  "call_function": {
    "name": "نام_تابع",
    "args": ["مقدار۱", "مقدار۲"]
  }
}
```

### دسترسی به نتیجه تابع

نتیجه تابع در متغیر ویژه `{{function_result}}` ذخیره می‌شود.

```json
{
  "call_function": {
    "name": "calculate",
    "args": [5, 10]
  }
},
{
  "set": {
    "var": "result",
    "value": "{{function_result}}"
  }
}
```

### مثال: فراخوانی چندباره

```json
{
  "name": "Multiple Function Calls",
  "path": "/multi-call",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "square",
        "params": ["x"],
        "body": [
          {
            "math_op": {
              "operation": "multiply",
              "args": ["{{x}}", "{{x}}"]
            }
          },
          {
            "return": {
              "value": "{{math_result}}"
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "square",
        "args": [5]
      }
    },
    {
      "set": {
        "var": "square_5",
        "value": "{{function_result}}"
      }
    },
    {
      "call_function": {
        "name": "square",
        "args": [10]
      }
    },
    {
      "set": {
        "var": "square_10",
        "value": "{{function_result}}"
      }
    },
    {
      "return": {
        "value": {
          "square_5": "{{square_5}}",
          "square_10": "{{square_10}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "square_5": 25,
  "square_10": 100
}
```

---

## پارامترها و Return

### مثال ۱: تابع با چند پارامتر

```json
{
  "name": "Calculate Rectangle Area",
  "path": "/rectangle-area",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "calculate_area",
        "params": ["width", "height"],
        "body": [
          {
            "math_op": {
              "operation": "multiply",
              "args": ["{{width}}", "{{height}}"]
            }
          },
          {
            "return": {
              "value": "{{math_result}}"
            }
          }
        ]
      }
    },
    {
      "set": {
        "var": "width",
        "value": "{{request.payload.width}}"
      }
    },
    {
      "set": {
        "var": "height",
        "value": "{{request.payload.height}}"
      }
    },
    {
      "call_function": {
        "name": "calculate_area",
        "args": ["{{width}}", "{{height}}"]
      }
    },
    {
      "return": {
        "value": {
          "width": "{{width}}",
          "height": "{{height}}",
          "area": "{{function_result}}"
        }
      }
    }
  ]
}
```

### مثال ۲: تابع با Return مشروط

```json
{
  "name": "Check Even or Odd",
  "path": "/check-even-odd",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "is_even",
        "params": ["number"],
        "body": [
          {
            "math_op": {
              "operation": "mod",
              "args": ["{{number}}", 2]
            }
          },
          {
            "if": {
              "condition": "{{math_result}} == 0",
              "then": [
                {
                  "return": {
                    "value": true
                  }
                }
              ],
              "else": [
                {
                  "return": {
                    "value": false
                  }
                }
              ]
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "is_even",
        "args": ["{{request.payload.number}}"]
      }
    },
    {
      "return": {
        "value": {
          "number": "{{request.payload.number}}",
          "is_even": "{{function_result}}"
        }
      }
    }
  ]
}
```

### مثال ۳: Return Object

```json
{
  "name": "User Info Function",
  "path": "/user-info",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "get_user_info",
        "params": ["user_id", "name", "age"],
        "body": [
          {
            "set": {
              "var": "user",
              "value": {
                "id": "{{user_id}}",
                "name": "{{name}}",
                "age": "{{age}}",
                "created_at": "2024-01-01"
              }
            }
          },
          {
            "return": {
              "value": "{{user}}"
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "get_user_info",
        "args": [123, "علی", 25]
      }
    },
    {
      "return": {
        "value": {
          "user": "{{function_result}}"
        }
      }
    }
  ]
}
```

---

## نمونه‌های پیشرفته

### مثال ۱: محاسبه Fibonacci با تابع

```json
{
  "name": "Fibonacci Function",
  "path": "/fibonacci-func",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "fibonacci",
        "params": ["n"],
        "body": [
          {
            "if": {
              "condition": "{{n}} <= 1",
              "then": [
                {
                  "return": {
                    "value": "{{n}}"
                  }
                }
              ]
            }
          },
          {"set": {"var": "a", "value": 0}},
          {"set": {"var": "b", "value": 1}},
          {"set": {"var": "i", "value": 2}},
          {
            "while": {
              "condition": "{{i}} <= {{n}}",
              "body": [
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{a}}", "{{b}}"]
                  }
                },
                {"set": {"var": "temp", "value": "{{math_result}}"}},
                {"set": {"var": "a", "value": "{{b}}"}},
                {"set": {"var": "b", "value": "{{temp}}"}},
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{i}}", 1]
                  }
                },
                {"set": {"var": "i", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "return": {
              "value": "{{b}}"
            }
          }
        ]
      }
    },
    {
      "set": {
        "var": "n",
        "value": "{{request.payload.n}}"
      }
    },
    {
      "call_function": {
        "name": "fibonacci",
        "args": ["{{n}}"]
      }
    },
    {
      "return": {
        "value": {
          "n": "{{n}}",
          "fibonacci": "{{function_result}}"
        }
      }
    }
  ]
}
```

### مثال ۲: محاسبه Factorial با تابع

```json
{
  "name": "Factorial Function",
  "path": "/factorial-func",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "factorial",
        "params": ["n"],
        "body": [
          {
            "if": {
              "condition": "{{n}} <= 1",
              "then": [
                {
                  "return": {
                    "value": 1
                  }
                }
              ]
            }
          },
          {"set": {"var": "result", "value": 1}},
          {"set": {"var": "i", "value": 2}},
          {
            "while": {
              "condition": "{{i}} <= {{n}}",
              "body": [
                {
                  "math_op": {
                    "operation": "multiply",
                    "args": ["{{result}}", "{{i}}"]
                  }
                },
                {"set": {"var": "result", "value": "{{math_result}}"}},
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{i}}", 1]
                  }
                },
                {"set": {"var": "i", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "return": {
              "value": "{{result}}"
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "factorial",
        "args": [5]
      }
    },
    {
      "return": {
        "value": {
          "factorial_5": "{{function_result}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "factorial_5": 120
}
```

### مثال ۳: تابع با توابع دیگر

```json
{
  "name": "Math Utilities",
  "path": "/math-utils",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "square",
        "params": ["x"],
        "body": [
          {
            "math_op": {
              "operation": "multiply",
              "args": ["{{x}}", "{{x}}"]
            }
          },
          {
            "return": {
              "value": "{{math_result}}"
            }
          }
        ]
      }
    },
    {
      "define_function": {
        "name": "sum_of_squares",
        "params": ["a", "b"],
        "body": [
          {
            "call_function": {
              "name": "square",
              "args": ["{{a}}"]
            }
          },
          {"set": {"var": "square_a", "value": "{{function_result}}"}},
          {
            "call_function": {
              "name": "square",
              "args": ["{{b}}"]
            }
          },
          {"set": {"var": "square_b", "value": "{{function_result}}"}},
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{square_a}}", "{{square_b}}"]
            }
          },
          {
            "return": {
              "value": "{{math_result}}"
            }
          }
        ]
      }
    },
    {
      "call_function": {
        "name": "sum_of_squares",
        "args": [3, 4]
      }
    },
    {
      "return": {
        "value": {
          "result": "{{function_result}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "result": 25
}
```

### مثال ۴: تابع برای پردازش کاربر

```json
{
  "name": "User Processing Function",
  "path": "/process-user",
  "method": "POST",
  "logic": [
    {
      "define_function": {
        "name": "calculate_user_score",
        "params": ["age", "activity_count", "is_premium"],
        "body": [
          {"set": {"var": "score", "value": 0}},
          {
            "if": {
              "condition": "{{age}} >= 18",
              "then": [
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{score}}", 10]
                  }
                },
                {"set": {"var": "score", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{score}}", "{{activity_count}}"]
            }
          },
          {"set": {"var": "score", "value": "{{math_result}}"}},
          {
            "if": {
              "condition": "{{is_premium}} == true",
              "then": [
                {
                  "math_op": {
                    "operation": "multiply",
                    "args": ["{{score}}", 2]
                  }
                },
                {"set": {"var": "score", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "return": {
              "value": "{{score}}"
            }
          }
        ]
      }
    },
    {
      "set": {
        "var": "user",
        "value": "{{request.payload.user}}"
      }
    },
    {
      "call_function": {
        "name": "calculate_user_score",
        "args": ["{{user.age}}", "{{user.activity_count}}", "{{user.is_premium}}"]
      }
    },
    {
      "return": {
        "value": {
          "user": "{{user}}",
          "score": "{{function_result}}"
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. تعریف قبل از فراخوانی

✅ **درست:**
```json
[
  {
    "define_function": {
      "name": "my_func",
      "params": ["x"],
      "body": [...]
    }
  },
  {
    "call_function": {
      "name": "my_func",
      "args": [5]
    }
  }
]
```

❌ **غلط:**
```json
[
  {
    "call_function": {
      "name": "my_func",
      "args": [5]
    }
  },
  {
    "define_function": {
      "name": "my_func",
      "params": ["x"],
      "body": [...]
    }
  }
]
```

### 2. دسترسی به نتیجه تابع

```json
{
  "call_function": {
    "name": "add",
    "args": [5, 10]
  }
},
{
  "log": {
    "message": "Result: {{function_result}}"
  }
}
```

### 3. تعداد پارامترها

تعداد آرگومان‌ها باید با تعداد پارامترها برابر باشد:

```json
// تعریف با 2 پارامتر
{
  "define_function": {
    "name": "add",
    "params": ["a", "b"],
    "body": [...]
  }
}

// فراخوانی با 2 آرگومان
{
  "call_function": {
    "name": "add",
    "args": [5, 10]
  }
}
```

### 4. Return زودهنگام

می‌توانید در هر جای تابع Return کنید:

```json
{
  "define_function": {
    "name": "check_value",
    "params": ["x"],
    "body": [
      {
        "if": {
          "condition": "{{x}} < 0",
          "then": [
            {
              "return": {
                "value": "invalid"
              }
            }
          ]
        }
      },
      // اگر x >= 0 باشد، به اینجا می‌رسد
      {
        "return": {
          "value": "valid"
        }
      }
    ]
  }
}
```

---

## الگوهای رایج

### 1. تابع Validator

```json
{
  "define_function": {
    "name": "validate_email",
    "params": ["email"],
    "body": [
      {
        "if": {
          "condition": "{{email}} == ''",
          "then": [
            {"return": {"value": false}}
          ]
        }
      },
      {
        "return": {"value": true}
      }
    ]
  }
}
```

### 2. تابع Calculator

```json
{
  "define_function": {
    "name": "calculate_price",
    "params": ["base_price", "quantity", "discount"],
    "body": [
      {"math_op": {"operation": "multiply", "args": ["{{base_price}}", "{{quantity}}"]}},
      {"set": {"var": "subtotal", "value": "{{math_result}}"}},
      {"math_op": {"operation": "multiply", "args": ["{{subtotal}}", "{{discount}}"]}},
      {"math_op": {"operation": "divide", "args": ["{{math_result}}", 100]}},
      {"set": {"var": "discount_amount", "value": "{{math_result}}"}},
      {"math_op": {"operation": "subtract", "args": ["{{subtotal}}", "{{discount_amount}}"]}},
      {"return": {"value": "{{math_result}}"}}
    ]
  }
}
```

### 3. تابع Formatter

```json
{
  "define_function": {
    "name": "format_user_name",
    "params": ["first_name", "last_name"],
    "body": [
      {
        "string_op": {
          "operation": "upper",
          "value": "{{first_name}}"
        }
      },
      {"set": {"var": "first_upper", "value": "{{string_result}}"}},
      {
        "string_op": {
          "operation": "upper",
          "value": "{{last_name}}"
        }
      },
      {"set": {"var": "last_upper", "value": "{{string_result}}"}},
      {"return": {"value": "{{first_upper}} {{last_upper}}"}}
    ]
  }
}
```

---

## بهترین روش‌ها (Best Practices)

1. **نام‌گذاری معنادار**: از نام‌های واضح برای توابع استفاده کنید
2. **یک مسئولیت**: هر تابع باید یک کار مشخص انجام دهد
3. **Return صریح**: همیشه مقدار return کنید
4. **Validation ورودی**: پارامترها را در ابتدای تابع بررسی کنید
5. **توابع کوچک**: توابع کوچک خواناتر و قابل نگهداری‌تر هستند

---

[← قبلی: حلقه‌ها](05-loops.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: عملیات ریاضی →](07-math-operations.md)
