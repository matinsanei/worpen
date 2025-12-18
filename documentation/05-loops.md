# حلقه‌ها (Loops)

حلقه‌ها به شما امکان می‌دهند تا یک سری عملیات را چندین بار تکرار کنید.

## فهرست مطالب

- [While Loop](#while-loop)
- [ForEach Loop](#foreach-loop)
- [Break و Continue](#break-و-continue)
- [نمونه‌های پیشرفته](#نمونه‌های-پیشرفته)

---

## While Loop

حلقه While تا زمانی که شرط true باشد، ادامه می‌یابد.

### ساختار پایه

```json
{
  "while": {
    "condition": "شرط",
    "body": [
      // عملیات‌هایی که تکرار می‌شوند
    ]
  }
}
```

### مثال ۱: شمارش ساده

```json
{
  "name": "Simple Counter",
  "path": "/count",
  "method": "POST",
  "logic": [
    {"set": {"var": "counter", "value": 0}},
    {"set": {"var": "sum", "value": 0}},
    {
      "while": {
        "condition": "{{counter}} < 10",
        "body": [
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{sum}}", "{{counter}}"]
            }
          },
          {"set": {"var": "sum", "value": "{{math_result}}"}},
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{counter}}", 1]
            }
          },
          {"set": {"var": "counter", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "counter": "{{counter}}",
          "sum": "{{sum}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "counter": 10,
  "sum": 45
}
```

### مثال ۲: محاسبه Fibonacci

```json
{
  "name": "Fibonacci Calculator",
  "path": "/fibonacci",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "n",
        "value": "{{request.payload.n}}"
      }
    },
    {
      "if": {
        "condition": "{{n}} <= 1",
        "then": [
          {"return": {"value": {"result": "{{n}}"}}}
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
        "value": {
          "n": "{{n}}",
          "result": "{{b}}"
        }
      }
    }
  ]
}
```

**درخواست:**
```json
{
  "n": 8
}
```

**پاسخ:**
```json
{
  "n": 8,
  "result": 21
}
```

### مثال ۳: محاسبه Factorial

```json
{
  "name": "Factorial Calculator",
  "path": "/factorial",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "n",
        "value": "{{request.payload.n}}"
      }
    },
    {
      "if": {
        "condition": "{{n}} <= 1",
        "then": [
          {"return": {"value": {"result": 1}}}
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
        "value": {
          "n": "{{n}}",
          "result": "{{result}}"
        }
      }
    }
  ]
}
```

---

## ForEach Loop

حلقه ForEach برای پیمایش آرایه‌ها و اجرای عملیات روی هر عنصر استفاده می‌شود.

### ساختار پایه

```json
{
  "foreach": {
    "collection": "{{array_variable}}",
    "item_var": "item",
    "index_var": "index",
    "body": [
      // عملیات‌هایی که روی هر عنصر اجرا می‌شوند
    ]
  }
}
```

### مثال ۱: محاسبه مجموع آرایه

```json
{
  "name": "Array Sum",
  "path": "/array-sum",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "numbers",
        "value": [10, 20, 30, 40, 50]
      }
    },
    {"set": {"var": "sum", "value": 0}},
    {
      "foreach": {
        "collection": "{{numbers}}",
        "item_var": "num",
        "index_var": "i",
        "body": [
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{sum}}", "{{num}}"]
            }
          },
          {"set": {"var": "sum", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "numbers": "{{numbers}}",
          "sum": "{{sum}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "numbers": [10, 20, 30, 40, 50],
  "sum": 150
}
```

### مثال ۲: پردازش لیست کاربران

```json
{
  "name": "Process Users",
  "path": "/process-users",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "users",
        "value": [
          {"name": "علی", "age": 25, "active": true},
          {"name": "سارا", "age": 30, "active": false},
          {"name": "رضا", "age": 22, "active": true}
        ]
      }
    },
    {"set": {"var": "active_users", "value": []}},
    {"set": {"var": "total_age", "value": 0}},
    {
      "foreach": {
        "collection": "{{users}}",
        "item_var": "user",
        "index_var": "idx",
        "body": [
          {
            "if": {
              "condition": "{{user.active}} == true",
              "then": [
                {
                  "set": {
                    "var": "active_users",
                    "value": "{{active_users}}"
                  }
                }
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{total_age}}", "{{user.age}}"]
            }
          },
          {"set": {"var": "total_age", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "total_users": 3,
          "total_age": "{{total_age}}",
          "active_count": 2
        }
      }
    }
  ]
}
```

### مثال ۳: محاسبه ارزش موجودی انبار

```json
{
  "name": "Inventory Value Calculator",
  "path": "/inventory-value",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "items",
        "value": [
          {"name": "Apple", "quantity": 5, "price": 3},
          {"name": "Banana", "quantity": 8, "price": 2},
          {"name": "Orange", "quantity": 6, "price": 5}
        ]
      }
    },
    {"set": {"var": "total_value", "value": 0}},
    {
      "foreach": {
        "collection": "{{items}}",
        "item_var": "item",
        "index_var": "i",
        "body": [
          {
            "math_op": {
              "operation": "multiply",
              "args": ["{{item.quantity}}", "{{item.price}}"]
            }
          },
          {"set": {"var": "item_value", "value": "{{math_result}}"}},
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{total_value}}", "{{item_value}}"]
            }
          },
          {"set": {"var": "total_value", "value": "{{math_result}}"}},
          {
            "log": {
              "message": "{{item.name}}: {{item.quantity}} x {{item.price}} = {{item_value}}",
              "level": "info"
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "items": "{{items}}",
          "total_inventory_value": "{{total_value}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "items": [
    {"name": "Apple", "quantity": 5, "price": 3},
    {"name": "Banana", "quantity": 8, "price": 2},
    {"name": "Orange", "quantity": 6, "price": 5}
  ],
  "total_inventory_value": 61
}
```

---

## Break و Continue

### Break

`break` برای خروج زودهنگام از حلقه استفاده می‌شود.

```json
{
  "name": "Find First Match",
  "path": "/find-first",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "numbers",
        "value": [1, 5, 10, 15, 20, 25]
      }
    },
    {"set": {"var": "found", "value": false}},
    {"set": {"var": "target", "value": 15}},
    {
      "foreach": {
        "collection": "{{numbers}}",
        "item_var": "num",
        "index_var": "i",
        "body": [
          {
            "if": {
              "condition": "{{num}} == {{target}}",
              "then": [
                {"set": {"var": "found", "value": true}},
                {"set": {"var": "found_index", "value": "{{i}}"}},
                {"break": {}}
              ]
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "found": "{{found}}",
          "index": "{{found_index}}",
          "value": "{{target}}"
        }
      }
    }
  ]
}
```

### Continue

`continue` برای پرش به تکرار بعدی حلقه استفاده می‌شود.

```json
{
  "name": "Sum Only Even Numbers",
  "path": "/sum-even",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "numbers",
        "value": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
      }
    },
    {"set": {"var": "sum", "value": 0}},
    {
      "foreach": {
        "collection": "{{numbers}}",
        "item_var": "num",
        "index_var": "i",
        "body": [
          {
            "math_op": {
              "operation": "mod",
              "args": ["{{num}}", 2]
            }
          },
          {
            "if": {
              "condition": "{{math_result}} != 0",
              "then": [
                {"continue": {}}
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{sum}}", "{{num}}"]
            }
          },
          {"set": {"var": "sum", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "numbers": "{{numbers}}",
          "even_sum": "{{sum}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "numbers": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  "even_sum": 30
}
```

---

## نمونه‌های پیشرفته

### مثال ۱: پیدا کردن اعداد اول

```json
{
  "name": "Find Prime Numbers",
  "path": "/find-primes",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "max",
        "value": "{{request.payload.max}}"
      }
    },
    {"set": {"var": "primes", "value": []}},
    {"set": {"var": "num", "value": 2}},
    {
      "while": {
        "condition": "{{num}} <= {{max}}",
        "body": [
          {"set": {"var": "is_prime", "value": true}},
          {"set": {"var": "divisor", "value": 2}},
          {
            "while": {
              "condition": "{{divisor}} * {{divisor}} <= {{num}}",
              "body": [
                {
                  "math_op": {
                    "operation": "mod",
                    "args": ["{{num}}", "{{divisor}}"]
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
            "if": {
              "condition": "{{is_prime}} == true",
              "then": [
                {
                  "log": {
                    "message": "Found prime: {{num}}",
                    "level": "info"
                  }
                }
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{num}}", 1]
            }
          },
          {"set": {"var": "num", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "max": "{{max}}",
          "prime_count": "counted"
        }
      }
    }
  ]
}
```

### مثال ۲: جمع اعداد فرد در محدوده

```json
{
  "name": "Sum Odd Numbers in Range",
  "path": "/sum-odd-range",
  "method": "POST",
  "logic": [
    {"set": {"var": "start", "value": 1}},
    {"set": {"var": "end", "value": 20}},
    {"set": {"var": "sum", "value": 0}},
    {"set": {"var": "counter", "value": "{{start}}"}},
    {
      "while": {
        "condition": "{{counter}} <= {{end}}",
        "body": [
          {
            "math_op": {
              "operation": "mod",
              "args": ["{{counter}}", 2]
            }
          },
          {
            "if": {
              "condition": "{{math_result}} == 1",
              "then": [
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{sum}}", "{{counter}}"]
                  }
                },
                {"set": {"var": "sum", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{counter}}", 1]
            }
          },
          {"set": {"var": "counter", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "start": "{{start}}",
          "end": "{{end}}",
          "sum_odd": "{{sum}}"
        }
      }
    }
  ]
}
```

### مثال ۳: پردازش تودرتو (Nested Loops)

```json
{
  "name": "Matrix Multiplication",
  "path": "/matrix-sum",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "matrix",
        "value": [
          [1, 2, 3],
          [4, 5, 6],
          [7, 8, 9]
        ]
      }
    },
    {"set": {"var": "total_sum", "value": 0}},
    {
      "foreach": {
        "collection": "{{matrix}}",
        "item_var": "row",
        "index_var": "i",
        "body": [
          {"set": {"var": "row_sum", "value": 0}},
          {
            "foreach": {
              "collection": "{{row}}",
              "item_var": "cell",
              "index_var": "j",
              "body": [
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{row_sum}}", "{{cell}}"]
                  }
                },
                {"set": {"var": "row_sum", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{total_sum}}", "{{row_sum}}"]
            }
          },
          {"set": {"var": "total_sum", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "matrix": "{{matrix}}",
          "total_sum": "{{total_sum}}"
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. جلوگیری از حلقه بی‌نهایت

✅ **درست:**
```json
{
  "while": {
    "condition": "{{counter}} < 10",
    "body": [
      {"math_op": {"operation": "sum", "args": ["{{counter}}", 1]}},
      {"set": {"var": "counter", "value": "{{math_result}}"}}
    ]
  }
}
```

❌ **غلط (حلقه بی‌نهایت):**
```json
{
  "while": {
    "condition": "{{counter}} < 10",
    "body": [
      // counter را افزایش نداده‌ایم!
      {"log": {"message": "در حال اجرا..."}}
    ]
  }
}
```

### 2. دسترسی به عناصر آرایه در ForEach

```json
{
  "foreach": {
    "collection": "{{items}}",
    "item_var": "item",
    "index_var": "i",
    "body": [
      {"log": {"message": "Index: {{i}}, Value: {{item}}"}},
      {"log": {"message": "Name: {{item.name}}"}}
    ]
  }
}
```

### 3. Break فقط در حلقه کار می‌کند

```json
// Break باید داخل While یا ForEach باشد
{
  "while": {
    "condition": "true",
    "body": [
      {
        "if": {
          "condition": "{{found}} == true",
          "then": [
            {"break": {}}
          ]
        }
      }
    ]
  }
}
```

### 4. Continue به تکرار بعدی می‌رود

```json
{
  "foreach": {
    "collection": "{{items}}",
    "item_var": "item",
    "body": [
      {
        "if": {
          "condition": "{{item}} < 0",
          "then": [
            {"continue": {}}
          ]
        }
      },
      // این خط برای اعداد منفی اجرا نمی‌شود
      {"log": {"message": "Processing: {{item}}"}}
    ]
  }
}
```

---

## الگوهای رایج

### 1. پیمایش آرایه با فیلتر

```json
{
  "foreach": {
    "collection": "{{items}}",
    "item_var": "item",
    "body": [
      {
        "if": {
          "condition": "{{item.active}} == true",
          "then": [
            // پردازش فقط آیتم‌های فعال
          ]
        }
      }
    ]
  }
}
```

### 2. محاسبه مجموع با شرط

```json
{
  "foreach": {
    "collection": "{{numbers}}",
    "item_var": "num",
    "body": [
      {
        "if": {
          "condition": "{{num}} > 10",
          "then": [
            {"math_op": {"operation": "sum", "args": ["{{sum}}", "{{num}}"]}},
            {"set": {"var": "sum", "value": "{{math_result}}"}}
          ]
        }
      }
    ]
  }
}
```

### 3. جستجو و خروج زودهنگام

```json
{
  "foreach": {
    "collection": "{{items}}",
    "item_var": "item",
    "body": [
      {
        "if": {
          "condition": "{{item.id}} == {{target_id}}",
          "then": [
            {"set": {"var": "found_item", "value": "{{item}}"}},
            {"break": {}}
          ]
        }
      }
    ]
  }
}
```

---

## بهترین روش‌ها (Best Practices)

1. **همیشه شرط خروج داشته باشید**: در While حتماً شرطی که false شود داشته باشید
2. **از Break برای بهینه‌سازی استفاده کنید**: اگر چیزی را پیدا کردید، حلقه را break کنید
3. **متغیرهای شمارنده را به‌روز کنید**: در While فراموش نکنید که counter را افزایش دهید
4. **از ForEach برای آرایه‌ها استفاده کنید**: ForEach خواناتر و ساده‌تر از While است
5. **از Continue برای رد کردن موارد استفاده کنید**: به جای If تودرتو، از Continue استفاده کنید

---

[← قبلی: ساختارهای کنترلی](04-control-flow.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: توابع →](06-functions.md)
