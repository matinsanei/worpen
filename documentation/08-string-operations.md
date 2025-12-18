# عملیات رشته‌ای (String Operations)

عملیات رشته‌ای برای پردازش و دستکاری متن استفاده می‌شوند.

## فهرست مطالب

- [عملیات پایه](#عملیات-پایه)
- [عملیات پیشرفته](#عملیات-پیشرفته)
- [نمونه‌های کاربردی](#نمونه‌های-کاربردی)

---

## عملیات پایه

### ساختار کلی

```json
{
  "string_op": {
    "operation": "نام_عملیات",
    "value": "متن",
    "args": ["آرگومان‌ها"]
  }
}
```

نتیجه در متغیر ویژه `{{string_result}}` ذخیره می‌شود.

---

### Upper (تبدیل به حروف بزرگ)

```json
{
  "string_op": {
    "operation": "upper",
    "value": "hello world"
  }
}
// string_result = "HELLO WORLD"
```

**مثال کامل:**
```json
{
  "name": "Convert to Uppercase",
  "path": "/to-upper",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "text",
        "value": "{{request.payload.text}}"
      }
    },
    {
      "string_op": {
        "operation": "upper",
        "value": "{{text}}"
      }
    },
    {
      "return": {
        "value": {
          "original": "{{text}}",
          "uppercase": "{{string_result}}"
        }
      }
    }
  ]
}
```

---

### Lower (تبدیل به حروف کوچک)

```json
{
  "string_op": {
    "operation": "lower",
    "value": "HELLO WORLD"
  }
}
// string_result = "hello world"
```

---

### Trim (حذف فضای خالی)

```json
{
  "string_op": {
    "operation": "trim",
    "value": "  hello  "
  }
}
// string_result = "hello"
```

---

### Length (طول رشته)

```json
{
  "string_op": {
    "operation": "length",
    "value": "Hello"
  }
}
// string_result = 5
```

---

### Concat (الحاق رشته‌ها)

```json
{
  "string_op": {
    "operation": "concat",
    "value": "Hello",
    "args": [" ", "World", "!"]
  }
}
// string_result = "Hello World!"
```

**مثال:**
```json
{
  "name": "Concatenate Strings",
  "path": "/concat",
  "method": "POST",
  "logic": [
    {"set": {"var": "first_name", "value": "علی"}},
    {"set": {"var": "last_name", "value": "احمدی"}},
    {
      "string_op": {
        "operation": "concat",
        "value": "{{first_name}}",
        "args": [" ", "{{last_name}}"]
      }
    },
    {
      "return": {
        "value": {
          "full_name": "{{string_result}}"
        }
      }
    }
  ]
}
```

---

## عملیات پیشرفته

### Split (تقسیم رشته)

```json
{
  "string_op": {
    "operation": "split",
    "value": "apple,banana,orange",
    "args": [","]
  }
}
// string_result = ["apple", "banana", "orange"]
```

**مثال کامل:**
```json
{
  "name": "Split String",
  "path": "/split",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "text",
        "value": "John,Doe,john@example.com,30"
      }
    },
    {
      "string_op": {
        "operation": "split",
        "value": "{{text}}",
        "args": [","]
      }
    },
    {
      "set": {
        "var": "parts",
        "value": "{{string_result}}"
      }
    },
    {
      "return": {
        "value": {
          "first_name": "{{parts.0}}",
          "last_name": "{{parts.1}}",
          "email": "{{parts.2}}",
          "age": "{{parts.3}}"
        }
      }
    }
  ]
}
```

---

### Replace (جایگزینی)

```json
{
  "string_op": {
    "operation": "replace",
    "value": "Hello World",
    "args": ["World", "Rust"]
  }
}
// string_result = "Hello Rust"
```

**مثال:**
```json
{
  "name": "Replace Text",
  "path": "/replace",
  "method": "POST",
  "logic": [
    {"set": {"var": "text", "value": "I love JavaScript"}},
    {
      "string_op": {
        "operation": "replace",
        "value": "{{text}}",
        "args": ["JavaScript", "Rust"]
      }
    },
    {
      "return": {
        "value": {
          "original": "{{text}}",
          "replaced": "{{string_result}}"
        }
      }
    }
  ]
}
```

---

### Substring (زیررشته)

```json
{
  "string_op": {
    "operation": "substring",
    "value": "Hello World",
    "args": [0, 5]
  }
}
// string_result = "Hello"
```

---

### Contains (بررسی وجود)

```json
{
  "string_op": {
    "operation": "contains",
    "value": "Hello World",
    "args": ["World"]
  }
}
// string_result = true
```

---

### Starts With (شروع با)

```json
{
  "string_op": {
    "operation": "starts_with",
    "value": "Hello World",
    "args": ["Hello"]
  }
}
// string_result = true
```

---

### Ends With (پایان با)

```json
{
  "string_op": {
    "operation": "ends_with",
    "value": "Hello World",
    "args": ["World"]
  }
}
// string_result = true
```

---

### Reverse (معکوس کردن)

```json
{
  "string_op": {
    "operation": "reverse",
    "value": "Hello"
  }
}
// string_result = "olleH"
```

---

## نمونه‌های کاربردی

### مثال ۱: Validation ایمیل

```json
{
  "name": "Email Validator",
  "path": "/validate-email",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "email",
        "value": "{{request.payload.email}}"
      }
    },
    {
      "string_op": {
        "operation": "trim",
        "value": "{{email}}"
      }
    },
    {"set": {"var": "email", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "lower",
        "value": "{{email}}"
      }
    },
    {"set": {"var": "email_lower", "value": "{{string_result}}"}},
    {"set": {"var": "is_valid", "value": false}},
    {
      "string_op": {
        "operation": "contains",
        "value": "{{email_lower}}",
        "args": ["@"]
      }
    },
    {
      "if": {
        "condition": "{{string_result}} == true",
        "then": [
          {
            "string_op": {
              "operation": "contains",
              "value": "{{email_lower}}",
              "args": ["."]
            }
          },
          {
            "if": {
              "condition": "{{string_result}} == true",
              "then": [
                {"set": {"var": "is_valid", "value": true}}
              ]
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "email": "{{email}}",
          "is_valid": "{{is_valid}}"
        }
      }
    }
  ]
}
```

---

### مثال ۲: پردازش نام کامل

```json
{
  "name": "Parse Full Name",
  "path": "/parse-name",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "full_name",
        "value": "{{request.payload.full_name}}"
      }
    },
    {
      "string_op": {
        "operation": "trim",
        "value": "{{full_name}}"
      }
    },
    {"set": {"var": "full_name", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "split",
        "value": "{{full_name}}",
        "args": [" "]
      }
    },
    {"set": {"var": "parts", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "upper",
        "value": "{{parts.0}}"
      }
    },
    {"set": {"var": "first_name_upper", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "upper",
        "value": "{{parts.1}}"
      }
    },
    {"set": {"var": "last_name_upper", "value": "{{string_result}}"}},
    {
      "return": {
        "value": {
          "original": "{{full_name}}",
          "first_name": "{{parts.0}}",
          "last_name": "{{parts.1}}",
          "first_name_upper": "{{first_name_upper}}",
          "last_name_upper": "{{last_name_upper}}"
        }
      }
    }
  ]
}
```

---

### مثال ۳: تولید Slug از عنوان

```json
{
  "name": "Generate Slug",
  "path": "/generate-slug",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "title",
        "value": "{{request.payload.title}}"
      }
    },
    {
      "string_op": {
        "operation": "trim",
        "value": "{{title}}"
      }
    },
    {"set": {"var": "slug", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "lower",
        "value": "{{slug}}"
      }
    },
    {"set": {"var": "slug", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "replace",
        "value": "{{slug}}",
        "args": [" ", "-"]
      }
    },
    {"set": {"var": "slug", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "replace",
        "value": "{{slug}}",
        "args": ["--", "-"]
      }
    },
    {"set": {"var": "slug", "value": "{{string_result}}"}},
    {
      "return": {
        "value": {
          "title": "{{title}}",
          "slug": "{{slug}}"
        }
      }
    }
  ]
}
```

**درخواست:**
```json
{
  "title": "Hello  World  From  Rust"
}
```

**پاسخ:**
```json
{
  "title": "Hello  World  From  Rust",
  "slug": "hello-world-from-rust"
}
```

---

### مثال ۴: ماسک کردن شماره کارت

```json
{
  "name": "Mask Card Number",
  "path": "/mask-card",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "card_number",
        "value": "{{request.payload.card_number}}"
      }
    },
    {
      "string_op": {
        "operation": "length",
        "value": "{{card_number}}"
      }
    },
    {"set": {"var": "length", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "substring",
        "value": "{{card_number}}",
        "args": [0, 4]
      }
    },
    {"set": {"var": "first_four", "value": "{{string_result}}"}},
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{length}}", 4]
      }
    },
    {"set": {"var": "last_start", "value": "{{math_result}}"}},
    {
      "string_op": {
        "operation": "substring",
        "value": "{{card_number}}",
        "args": ["{{last_start}}", "{{length}}"]
      }
    },
    {"set": {"var": "last_four", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "concat",
        "value": "{{first_four}}",
        "args": [" **** **** ", "{{last_four}}"]
      }
    },
    {"set": {"var": "masked", "value": "{{string_result}}"}},
    {
      "return": {
        "value": {
          "original_length": "{{length}}",
          "masked_card": "{{masked}}"
        }
      }
    }
  ]
}
```

---

### مثال ۵: پردازش CSV

```json
{
  "name": "Parse CSV",
  "path": "/parse-csv",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "csv_data",
        "value": "name,age,city\nAli,25,Tehran\nSara,30,Shiraz"
      }
    },
    {
      "string_op": {
        "operation": "split",
        "value": "{{csv_data}}",
        "args": ["\n"]
      }
    },
    {"set": {"var": "lines", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "split",
        "value": "{{lines.0}}",
        "args": [","]
      }
    },
    {"set": {"var": "headers", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "split",
        "value": "{{lines.1}}",
        "args": [","]
      }
    },
    {"set": {"var": "row1", "value": "{{string_result}}"}},
    {
      "string_op": {
        "operation": "split",
        "value": "{{lines.2}}",
        "args": [","]
      }
    },
    {"set": {"var": "row2", "value": "{{string_result}}"}},
    {
      "return": {
        "value": {
          "headers": "{{headers}}",
          "rows": [
            {
              "name": "{{row1.0}}",
              "age": "{{row1.1}}",
              "city": "{{row1.2}}"
            },
            {
              "name": "{{row2.0}}",
              "age": "{{row2.1}}",
              "city": "{{row2.2}}"
            }
          ]
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
  "string_op": {
    "operation": "upper",
    "value": "{{text}}"
  }
}
```

❌ **غلط:**
```json
{
  "string_op": {
    "operation": "upper",
    "value": "text"
  }
}
```

### 2. نتیجه در string_result

```json
{
  "string_op": {
    "operation": "upper",
    "value": "hello"
  }
},
{
  "set": {
    "var": "result",
    "value": "{{string_result}}"
  }
}
```

### 3. Case Sensitivity

```json
// Contains is case-sensitive
{
  "string_op": {
    "operation": "contains",
    "value": "Hello World",
    "args": ["hello"]
  }
}
// string_result = false

// Convert to lower first
{
  "string_op": {
    "operation": "lower",
    "value": "Hello World"
  }
},
{
  "string_op": {
    "operation": "contains",
    "value": "{{string_result}}",
    "args": ["hello"]
  }
}
// string_result = true
```

### 4. Split همیشه آرایه برمی‌گرداند

```json
{
  "string_op": {
    "operation": "split",
    "value": "a,b,c",
    "args": [","]
  }
}
// string_result = ["a", "b", "c"]

// دسترسی به اولین عنصر
"{{string_result.0}}" // = "a"
```

---

## الگوهای رایج

### 1. Sanitize Input

```json
{
  "string_op": {"operation": "trim", "value": "{{input}}"}
},
{
  "string_op": {"operation": "lower", "value": "{{string_result}}"}
},
{
  "set": {"var": "clean_input", "value": "{{string_result}}"}}
}
```

### 2. Format Name

```json
{
  "string_op": {"operation": "trim", "value": "{{name}}"}
},
{
  "string_op": {"operation": "lower", "value": "{{string_result}}"}
},
{
  "set": {"var": "formatted", "value": "{{string_result}}"}}
}
```

### 3. Check Empty String

```json
{
  "string_op": {"operation": "trim", "value": "{{input}}"}
},
{
  "string_op": {"operation": "length", "value": "{{string_result}}"}
},
{
  "if": {
    "condition": "{{string_result}} == 0",
    "then": [
      {"set": {"var": "is_empty", "value": true}}
    ]
  }
}
```

---

## لیست کامل عملیات

| عملیات | توضیح | مثال |
|--------|-------|------|
| `upper` | حروف بزرگ | `upper("hello")` = "HELLO" |
| `lower` | حروف کوچک | `lower("HELLO")` = "hello" |
| `trim` | حذف فضای خالی | `trim(" hi ")` = "hi" |
| `length` | طول رشته | `length("hello")` = 5 |
| `concat` | الحاق | `concat("Hi", " ", "There")` = "Hi There" |
| `split` | تقسیم | `split("a,b,c", ",")` = ["a","b","c"] |
| `replace` | جایگزینی | `replace("hi", "h", "H")` = "Hi" |
| `substring` | زیررشته | `substring("hello", 0, 3)` = "hel" |
| `contains` | بررسی وجود | `contains("hello", "ell")` = true |
| `starts_with` | شروع با | `starts_with("hello", "he")` = true |
| `ends_with` | پایان با | `ends_with("hello", "lo")` = true |
| `reverse` | معکوس | `reverse("hello")` = "olleH" |

---

## بهترین روش‌ها (Best Practices)

1. **همیشه Trim کنید**: قبل از پردازش، فضای خالی را حذف کنید
2. **Case Insensitive مقایسه**: برای مقایسه، رشته‌ها را lower کنید
3. **Validate ورودی**: قبل از پردازش، رشته را بررسی کنید
4. **ذخیره نتایج میانی**: برای خوانایی بهتر
5. **استفاده از Contains**: برای بررسی وجود زیررشته

---

[← قبلی: عملیات ریاضی](07-math-operations.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: عملیات تاریخ →](09-date-operations.md)
