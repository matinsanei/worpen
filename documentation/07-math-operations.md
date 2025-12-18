# عملیات ریاضی (Math Operations)

عملیات ریاضی برای انجام محاسبات عددی استفاده می‌شوند.

## فهرست مطالب

- [عملیات پایه](#عملیات-پایه)
- [عملیات پیشرفته](#عملیات-پیشرفته)
- [نمونه‌های کاربردی](#نمونه‌های-کاربردی)

---

## عملیات پایه

### ساختار کلی

```json
{
  "math_op": {
    "operation": "نام_عملیات",
    "args": [مقدار۱, مقدار۲, ...]
  }
}
```

نتیجه در متغیر ویژه `{{math_result}}` ذخیره می‌شود.

---

### Sum (جمع)

```json
{
  "math_op": {
    "operation": "sum",
    "args": [10, 20, 30]
  }
}
// math_result = 60
```

### مثال کامل:

```json
{
  "name": "Add Numbers",
  "path": "/add",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "a",
        "value": "{{request.payload.a}}"
      }
    },
    {
      "set": {
        "var": "b",
        "value": "{{request.payload.b}}"
      }
    },
    {
      "math_op": {
        "operation": "sum",
        "args": ["{{a}}", "{{b}}"]
      }
    },
    {
      "return": {
        "value": {
          "a": "{{a}}",
          "b": "{{b}}",
          "sum": "{{math_result}}"
        }
      }
    }
  ]
}
```

---

### Subtract (تفریق)

```json
{
  "math_op": {
    "operation": "subtract",
    "args": [100, 30]
  }
}
// math_result = 70
```

---

### Multiply (ضرب)

```json
{
  "math_op": {
    "operation": "multiply",
    "args": [5, 10]
  }
}
// math_result = 50
```

---

### Divide (تقسیم)

```json
{
  "math_op": {
    "operation": "divide",
    "args": [100, 4]
  }
}
// math_result = 25
```

**نکته:** در صورت تقسیم بر صفر، خطا رخ می‌دهد.

---

### Mod (باقیمانده)

```json
{
  "math_op": {
    "operation": "mod",
    "args": [17, 5]
  }
}
// math_result = 2
```

**کاربرد:** بررسی زوج یا فرد بودن، بررسی بخش‌پذیری

---

## عملیات پیشرفته

### Power (توان)

```json
{
  "math_op": {
    "operation": "pow",
    "args": [2, 10]
  }
}
// math_result = 1024 (2^10)
```

---

### Square Root (جذر)

```json
{
  "math_op": {
    "operation": "sqrt",
    "args": [144]
  }
}
// math_result = 12
```

---

### Average (میانگین)

```json
{
  "math_op": {
    "operation": "avg",
    "args": [10, 20, 30, 40, 50]
  }
}
// math_result = 30
```

---

### Min (کمترین)

```json
{
  "math_op": {
    "operation": "min",
    "args": [45, 12, 78, 23, 56]
  }
}
// math_result = 12
```

---

### Max (بیشترین)

```json
{
  "math_op": {
    "operation": "max",
    "args": [45, 12, 78, 23, 56]
  }
}
// math_result = 78
```

---

### Abs (قدر مطلق)

```json
{
  "math_op": {
    "operation": "abs",
    "args": [-25]
  }
}
// math_result = 25
```

---

### Round (گرد کردن)

```json
{
  "math_op": {
    "operation": "round",
    "args": [3.7]
  }
}
// math_result = 4
```

---

### Floor (گرد کردن به پایین)

```json
{
  "math_op": {
    "operation": "floor",
    "args": [3.9]
  }
}
// math_result = 3
```

---

### Ceil (گرد کردن به بالا)

```json
{
  "math_op": {
    "operation": "ceil",
    "args": [3.1]
  }
}
// math_result = 4
```

---

## نمونه‌های کاربردی

### مثال ۱: محاسبه BMI (شاخص توده بدنی)

```json
{
  "name": "BMI Calculator",
  "path": "/calculate-bmi",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "weight",
        "value": "{{request.payload.weight}}"
      }
    },
    {
      "set": {
        "var": "height",
        "value": "{{request.payload.height}}"
      }
    },
    {
      "math_op": {
        "operation": "pow",
        "args": ["{{height}}", 2]
      }
    },
    {
      "set": {
        "var": "height_squared",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "divide",
        "args": ["{{weight}}", "{{height_squared}}"]
      }
    },
    {
      "set": {
        "var": "bmi",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "round",
        "args": ["{{bmi}}"]
      }
    },
    {
      "set": {
        "var": "bmi_rounded",
        "value": "{{math_result}}"
      }
    },
    {
      "if": {
        "condition": "{{bmi}} < 18.5",
        "then": [
          {"set": {"var": "status", "value": "کم‌وزن"}}
        ],
        "else": [
          {
            "if": {
              "condition": "{{bmi}} < 25",
              "then": [
                {"set": {"var": "status", "value": "نرمال"}}
              ],
              "else": [
                {
                  "if": {
                    "condition": "{{bmi}} < 30",
                    "then": [
                      {"set": {"var": "status", "value": "اضافه‌وزن"}}
                    ],
                    "else": [
                      {"set": {"var": "status", "value": "چاق"}}
                    ]
                  }
                }
              ]
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "weight": "{{weight}}",
          "height": "{{height}}",
          "bmi": "{{bmi_rounded}}",
          "status": "{{status}}"
        }
      }
    }
  ]
}
```

**درخواست:**
```json
{
  "weight": 70,
  "height": 1.75
}
```

**پاسخ:**
```json
{
  "weight": 70,
  "height": 1.75,
  "bmi": 23,
  "status": "نرمال"
}
```

---

### مثال ۲: محاسبه سود بانکی

```json
{
  "name": "Interest Calculator",
  "path": "/calculate-interest",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "principal",
        "value": "{{request.payload.principal}}"
      }
    },
    {
      "set": {
        "var": "rate",
        "value": "{{request.payload.rate}}"
      }
    },
    {
      "set": {
        "var": "years",
        "value": "{{request.payload.years}}"
      }
    },
    {
      "math_op": {
        "operation": "divide",
        "args": ["{{rate}}", 100]
      }
    },
    {
      "set": {
        "var": "rate_decimal",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "sum",
        "args": [1, "{{rate_decimal}}"]
      }
    },
    {
      "set": {
        "var": "factor",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "pow",
        "args": ["{{factor}}", "{{years}}"]
      }
    },
    {
      "set": {
        "var": "growth",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "multiply",
        "args": ["{{principal}}", "{{growth}}"]
      }
    },
    {
      "set": {
        "var": "final_amount",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{final_amount}}", "{{principal}}"]
      }
    },
    {
      "set": {
        "var": "interest",
        "value": "{{math_result}}"
      }
    },
    {
      "return": {
        "value": {
          "principal": "{{principal}}",
          "rate": "{{rate}}",
          "years": "{{years}}",
          "interest": "{{interest}}",
          "final_amount": "{{final_amount}}"
        }
      }
    }
  ]
}
```

---

### مثال ۳: محاسبه مساحت و محیط دایره

```json
{
  "name": "Circle Calculator",
  "path": "/calculate-circle",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "radius",
        "value": "{{request.payload.radius}}"
      }
    },
    {"set": {"var": "pi", "value": 3.14159}},
    {
      "math_op": {
        "operation": "multiply",
        "args": [2, "{{pi}}", "{{radius}}"]
      }
    },
    {"set": {"var": "circumference", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "pow",
        "args": ["{{radius}}", 2]
      }
    },
    {"set": {"var": "radius_squared", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "multiply",
        "args": ["{{pi}}", "{{radius_squared}}"]
      }
    },
    {"set": {"var": "area", "value": "{{math_result}}"}},
    {
      "return": {
        "value": {
          "radius": "{{radius}}",
          "circumference": "{{circumference}}",
          "area": "{{area}}"
        }
      }
    }
  ]
}
```

---

### مثال ۴: آمار آرایه اعداد

```json
{
  "name": "Array Statistics",
  "path": "/array-stats",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "numbers",
        "value": [15, 23, 8, 42, 16, 34, 11, 28]
      }
    },
    {
      "math_op": {
        "operation": "sum",
        "args": [15, 23, 8, 42, 16, 34, 11, 28]
      }
    },
    {"set": {"var": "sum", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "avg",
        "args": [15, 23, 8, 42, 16, 34, 11, 28]
      }
    },
    {"set": {"var": "average", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "min",
        "args": [15, 23, 8, 42, 16, 34, 11, 28]
      }
    },
    {"set": {"var": "min", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "max",
        "args": [15, 23, 8, 42, 16, 34, 11, 28]
      }
    },
    {"set": {"var": "max", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{max}}", "{{min}}"]
      }
    },
    {"set": {"var": "range", "value": "{{math_result}}"}},
    {
      "return": {
        "value": {
          "numbers": "{{numbers}}",
          "sum": "{{sum}}",
          "average": "{{average}}",
          "min": "{{min}}",
          "max": "{{max}}",
          "range": "{{range}}"
        }
      }
    }
  ]
}
```

---

### مثال ۵: بررسی عدد اول (Prime Number)

```json
{
  "name": "Prime Number Checker",
  "path": "/check-prime",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "number",
        "value": "{{request.payload.number}}"
      }
    },
    {
      "if": {
        "condition": "{{number}} <= 1",
        "then": [
          {
            "return": {
              "value": {
                "number": "{{number}}",
                "is_prime": false
              }
            }
          }
        ]
      }
    },
    {"set": {"var": "divisor", "value": 2}},
    {"set": {"var": "is_prime", "value": true}},
    {
      "math_op": {
        "operation": "sqrt",
        "args": ["{{number}}"]
      }
    },
    {"set": {"var": "sqrt_num", "value": "{{math_result}}"}},
    {
      "while": {
        "condition": "{{divisor}} <= {{sqrt_num}}",
        "body": [
          {
            "math_op": {
              "operation": "mod",
              "args": ["{{number}}", "{{divisor}}"]
            }
          },
          {
            "if": {
              "condition": "{{math_result}} == 0",
              "then": [
                {"set": {"var": "is_prime", "value": false}},
                {"break": {}}
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{divisor}}", 1]
            }
          },
          {"set": {"var": "divisor", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "number": "{{number}}",
          "is_prime": "{{is_prime}}"
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. Resolution متغیرها

✅ **درست:**
```json
{
  "math_op": {
    "operation": "sum",
    "args": ["{{a}}", "{{b}}"]
  }
}
```

❌ **غلط:**
```json
{
  "math_op": {
    "operation": "sum",
    "args": ["a", "b"]
  }
}
```

### 2. نتیجه همیشه عدد است

```json
{
  "math_op": {
    "operation": "sum",
    "args": [10, 20]
  }
}
// math_result = 30 (number)
```

### 3. دقت اعشار

```json
{
  "math_op": {
    "operation": "divide",
    "args": [10, 3]
  }
}
// math_result = 3.3333333...
```

### 4. تقسیم بر صفر

```json
{
  "try": {
    "actions": [
      {
        "math_op": {
          "operation": "divide",
          "args": [10, 0]
        }
      }
    ],
    "catch": [
      {
        "set": {
          "var": "error_msg",
          "value": "تقسیم بر صفر مجاز نیست"
        }
      }
    ]
  }
}
```

---

## الگوهای رایج

### 1. Chaining (زنجیره‌ای)

```json
{
  "math_op": {"operation": "sum", "args": [10, 20]}
},
{
  "set": {"var": "step1", "value": "{{math_result}}"}
},
{
  "math_op": {"operation": "multiply", "args": ["{{step1}}", 2]}
},
{
  "set": {"var": "step2", "value": "{{math_result}}"}
},
{
  "math_op": {"operation": "subtract", "args": ["{{step2}}", 5]}
},
{
  "set": {"var": "final", "value": "{{math_result}}"}
}
```

### 2. محاسبه درصد

```json
{
  "math_op": {
    "operation": "multiply",
    "args": ["{{total}}", "{{percent}}"]
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
}
```

### 3. تبدیل واحد

```json
// تبدیل کیلومتر به مایل
{
  "math_op": {
    "operation": "multiply",
    "args": ["{{km}}", 0.621371]
  }
},
{
  "set": {
    "var": "miles",
    "value": "{{math_result}}"
  }
}
```

---

## لیست کامل عملیات

| عملیات | توضیح | مثال |
|--------|-------|------|
| `sum` | جمع | `sum(10, 20, 30)` = 60 |
| `subtract` | تفریق | `subtract(100, 30)` = 70 |
| `multiply` | ضرب | `multiply(5, 10)` = 50 |
| `divide` | تقسیم | `divide(100, 4)` = 25 |
| `mod` | باقیمانده | `mod(17, 5)` = 2 |
| `pow` | توان | `pow(2, 10)` = 1024 |
| `sqrt` | جذر | `sqrt(144)` = 12 |
| `avg` | میانگین | `avg(10, 20, 30)` = 20 |
| `min` | کمترین | `min(45, 12, 78)` = 12 |
| `max` | بیشترین | `max(45, 12, 78)` = 78 |
| `abs` | قدر مطلق | `abs(-25)` = 25 |
| `round` | گرد کردن | `round(3.7)` = 4 |
| `floor` | گرد کردن به پایین | `floor(3.9)` = 3 |
| `ceil` | گرد کردن به بالا | `ceil(3.1)` = 4 |

---

## بهترین روش‌ها (Best Practices)

1. **ذخیره نتایج میانی**: نتایج را در متغیرهای با نام معنادار ذخیره کنید
2. **بررسی تقسیم بر صفر**: قبل از تقسیم، مقسوم‌علیه را بررسی کنید
3. **استفاده از Round**: برای نمایش بهتر، اعداد را Round کنید
4. **استفاده از Abs**: برای مقادیر منفی که نیاز به مثبت است
5. **استفاده از Min/Max**: برای محدود کردن مقادیر در یک بازه

---

[← قبلی: توابع](06-functions.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: عملیات رشته‌ای →](08-string-operations.md)
