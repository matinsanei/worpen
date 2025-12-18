# ساختارهای کنترلی (Control Flow)

ساختارهای کنترلی به شما امکان می‌دهند تا flow اجرای برنامه را بر اساس شرایط مختلف تغییر دهید.

## فهرست مطالب

- [If/Else](#ifelse)
- [Switch/Case](#switchcase)
- [نمونه‌های کاربردی](#نمونه‌های-کاربردی)

---

## If/Else

### ساختار پایه

```json
{
  "if": {
    "condition": "شرط",
    "then": [
      // عملیات‌هایی که اگر شرط true باشد اجرا می‌شوند
    ],
    "else": [
      // عملیات‌هایی که اگر شرط false باشد اجرا می‌شوند (اختیاری)
    ]
  }
}
```

### عملگرهای مقایسه‌ای

- `==` - برابر است با
- `!=` - نابرابر است با
- `>` - بزرگتر از
- `<` - کوچکتر از
- `>=` - بزرگتر یا مساوی
- `<=` - کوچکتر یا مساوی

### عملگرهای منطقی

- `&&` - AND (و)
- `||` - OR (یا)

### مثال ۱: بررسی سن

```json
{
  "name": "Check Age",
  "path": "/check-age",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "age",
        "value": "{{request.payload.age}}"
      }
    },
    {
      "if": {
        "condition": "{{age}} >= 18",
        "then": [
          {
            "set": {
              "var": "message",
              "value": "شما بالغ هستید"
            }
          },
          {
            "set": {
              "var": "can_vote",
              "value": true
            }
          }
        ],
        "else": [
          {
            "set": {
              "var": "message",
              "value": "شما نوجوان هستید"
            }
          },
          {
            "set": {
              "var": "can_vote",
              "value": false
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "age": "{{age}}",
          "message": "{{message}}",
          "can_vote": "{{can_vote}}"
        }
      }
    }
  ]
}
```

**درخواست:**
```json
{
  "age": 25
}
```

**پاسخ:**
```json
{
  "age": 25,
  "message": "شما بالغ هستید",
  "can_vote": true
}
```

### مثال ۲: If تودرتو (Nested If)

```json
{
  "name": "Grade Calculator",
  "path": "/calculate-grade",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "score",
        "value": "{{request.payload.score}}"
      }
    },
    {
      "if": {
        "condition": "{{score}} >= 90",
        "then": [
          {"set": {"var": "grade", "value": "A"}},
          {"set": {"var": "status", "value": "عالی"}}
        ],
        "else": [
          {
            "if": {
              "condition": "{{score}} >= 80",
              "then": [
                {"set": {"var": "grade", "value": "B"}},
                {"set": {"var": "status", "value": "خوب"}}
              ],
              "else": [
                {
                  "if": {
                    "condition": "{{score}} >= 70",
                    "then": [
                      {"set": {"var": "grade", "value": "C"}},
                      {"set": {"var": "status", "value": "متوسط"}}
                    ],
                    "else": [
                      {"set": {"var": "grade", "value": "F"}},
                      {"set": {"var": "status", "value": "مردود"}}
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
          "score": "{{score}}",
          "grade": "{{grade}}",
          "status": "{{status}}"
        }
      }
    }
  ]
}
```

### مثال ۳: شرط‌های ترکیبی (AND/OR)

```json
{
  "name": "Access Control",
  "path": "/check-access",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "is_logged_in",
        "value": "{{request.payload.is_logged_in}}"
      }
    },
    {
      "set": {
        "var": "has_permission",
        "value": "{{request.payload.has_permission}}"
      }
    },
    {
      "set": {
        "var": "is_active",
        "value": "{{request.payload.is_active}}"
      }
    },
    {
      "if": {
        "condition": "{{is_logged_in}} && {{has_permission}} && {{is_active}}",
        "then": [
          {"set": {"var": "access", "value": "granted"}},
          {"set": {"var": "message", "value": "دسترسی داده شد"}}
        ],
        "else": [
          {"set": {"var": "access", "value": "denied"}},
          {"set": {"var": "message", "value": "دسترسی رد شد"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "access": "{{access}}",
          "message": "{{message}}"
        }
      }
    }
  ]
}
```

---

## Switch/Case

Switch/Case برای بررسی یک مقدار در برابر چندین حالت مختلف استفاده می‌شود.

### ساختار پایه

```json
{
  "switch": {
    "value": "مقدار برای بررسی",
    "cases": [
      {
        "case": "حالت ۱",
        "actions": [
          // عملیات‌های حالت ۱
        ]
      },
      {
        "case": "حالت ۲",
        "actions": [
          // عملیات‌های حالت ۲
        ]
      }
    ],
    "default": [
      // عملیات پیش‌فرض (اختیاری)
    ]
  }
}
```

### مثال ۱: مدیریت وضعیت سفارش

```json
{
  "name": "Order Status Handler",
  "path": "/handle-order-status",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "status",
        "value": "{{request.payload.status}}"
      }
    },
    {
      "switch": {
        "value": "{{status}}",
        "cases": [
          {
            "case": "pending",
            "actions": [
              {"set": {"var": "message", "value": "سفارش در انتظار تایید"}},
              {"set": {"var": "next_action", "value": "تایید مدیر"}},
              {"set": {"var": "priority", "value": 1}}
            ]
          },
          {
            "case": "confirmed",
            "actions": [
              {"set": {"var": "message", "value": "سفارش تایید شد"}},
              {"set": {"var": "next_action", "value": "ارسال به انبار"}},
              {"set": {"var": "priority", "value": 2}}
            ]
          },
          {
            "case": "shipped",
            "actions": [
              {"set": {"var": "message", "value": "سفارش ارسال شد"}},
              {"set": {"var": "next_action", "value": "تحویل به مشتری"}},
              {"set": {"var": "priority", "value": 3}}
            ]
          },
          {
            "case": "delivered",
            "actions": [
              {"set": {"var": "message", "value": "سفارش تحویل داده شد"}},
              {"set": {"var": "next_action", "value": "بررسی رضایت"}},
              {"set": {"var": "priority", "value": 4}}
            ]
          },
          {
            "case": "cancelled",
            "actions": [
              {"set": {"var": "message", "value": "سفارش لغو شد"}},
              {"set": {"var": "next_action", "value": "بازگشت وجه"}},
              {"set": {"var": "priority", "value": 0}}
            ]
          }
        ],
        "default": [
          {"set": {"var": "message", "value": "وضعیت نامشخص"}},
          {"set": {"var": "next_action", "value": "بررسی دستی"}},
          {"set": {"var": "priority", "value": -1}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "status": "{{status}}",
          "message": "{{message}}",
          "next_action": "{{next_action}}",
          "priority": "{{priority}}"
        }
      }
    }
  ]
}
```

### مثال ۲: مدیریت سطح دسترسی کاربر

```json
{
  "name": "User Role Permissions",
  "path": "/get-permissions",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "role",
        "value": "{{request.payload.role}}"
      }
    },
    {
      "switch": {
        "value": "{{role}}",
        "cases": [
          {
            "case": "admin",
            "actions": [
              {"set": {"var": "can_create", "value": true}},
              {"set": {"var": "can_read", "value": true}},
              {"set": {"var": "can_update", "value": true}},
              {"set": {"var": "can_delete", "value": true}},
              {"set": {"var": "access_level", "value": 100}},
              {"set": {"var": "description", "value": "دسترسی کامل"}}
            ]
          },
          {
            "case": "editor",
            "actions": [
              {"set": {"var": "can_create", "value": true}},
              {"set": {"var": "can_read", "value": true}},
              {"set": {"var": "can_update", "value": true}},
              {"set": {"var": "can_delete", "value": false}},
              {"set": {"var": "access_level", "value": 70}},
              {"set": {"var": "description", "value": "ویرایشگر"}}
            ]
          },
          {
            "case": "viewer",
            "actions": [
              {"set": {"var": "can_create", "value": false}},
              {"set": {"var": "can_read", "value": true}},
              {"set": {"var": "can_update", "value": false}},
              {"set": {"var": "can_delete", "value": false}},
              {"set": {"var": "access_level", "value": 30}},
              {"set": {"var": "description", "value": "فقط مشاهده"}}
            ]
          },
          {
            "case": "guest",
            "actions": [
              {"set": {"var": "can_create", "value": false}},
              {"set": {"var": "can_read", "value": false}},
              {"set": {"var": "can_update", "value": false}},
              {"set": {"var": "can_delete", "value": false}},
              {"set": {"var": "access_level", "value": 0}},
              {"set": {"var": "description", "value": "بدون دسترسی"}}
            ]
          }
        ],
        "default": [
          {"set": {"var": "can_create", "value": false}},
          {"set": {"var": "can_read", "value": false}},
          {"set": {"var": "can_update", "value": false}},
          {"set": {"var": "can_delete", "value": false}},
          {"set": {"var": "access_level", "value": 0}},
          {"set": {"var": "description", "value": "نقش نامعتبر"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "role": "{{role}}",
          "permissions": {
            "create": "{{can_create}}",
            "read": "{{can_read}}",
            "update": "{{can_update}}",
            "delete": "{{can_delete}}"
          },
          "access_level": "{{access_level}}",
          "description": "{{description}}"
        }
      }
    }
  ]
}
```

---

## نمونه‌های کاربردی

### مثال ۱: سیستم تخفیف چندسطحی

```json
{
  "name": "Discount System",
  "path": "/calculate-discount",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "total_amount",
        "value": "{{request.payload.amount}}"
      }
    },
    {
      "set": {
        "var": "customer_type",
        "value": "{{request.payload.customer_type}}"
      }
    },
    {
      "set": {
        "var": "is_first_purchase",
        "value": "{{request.payload.is_first_purchase}}"
      }
    },
    {
      "set": {"var": "discount_percent", "value": 0}
    },
    {
      "switch": {
        "value": "{{customer_type}}",
        "cases": [
          {
            "case": "vip",
            "actions": [
              {"set": {"var": "discount_percent", "value": 20}}
            ]
          },
          {
            "case": "regular",
            "actions": [
              {"set": {"var": "discount_percent", "value": 10}}
            ]
          },
          {
            "case": "new",
            "actions": [
              {"set": {"var": "discount_percent", "value": 5}}
            ]
          }
        ]
      }
    },
    {
      "if": {
        "condition": "{{is_first_purchase}} == true",
        "then": [
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{discount_percent}}", 15]
            }
          },
          {"set": {"var": "discount_percent", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "if": {
        "condition": "{{total_amount}} > 1000",
        "then": [
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{discount_percent}}", 5]
            }
          },
          {"set": {"var": "discount_percent", "value": "{{math_result}}"}}
        ]
      }
    },
    {
      "math_op": {
        "operation": "multiply",
        "args": ["{{total_amount}}", "{{discount_percent}}"]
      }
    },
    {
      "math_op": {
        "operation": "divide",
        "args": ["{{math_result}}", 100]
      }
    },
    {"set": {"var": "discount_amount", "value": "{{math_result}}"}},
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{total_amount}}", "{{discount_amount}}"]
      }
    },
    {"set": {"var": "final_amount", "value": "{{math_result}}"}},
    {
      "return": {
        "value": {
          "original_amount": "{{total_amount}}",
          "customer_type": "{{customer_type}}",
          "discount_percent": "{{discount_percent}}",
          "discount_amount": "{{discount_amount}}",
          "final_amount": "{{final_amount}}"
        }
      }
    }
  ]
}
```

### مثال ۲: احراز هویت و بررسی دسترسی

```json
{
  "name": "Authentication & Authorization",
  "path": "/check-auth",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "username",
        "value": "{{request.payload.username}}"
      }
    },
    {
      "set": {
        "var": "password",
        "value": "{{request.payload.password}}"
      }
    },
    {
      "set": {
        "var": "requested_resource",
        "value": "{{request.payload.resource}}"
      }
    },
    {
      "if": {
        "condition": "{{username}} == '' || {{password}} == ''",
        "then": [
          {"set": {"var": "auth_status", "value": "failed"}},
          {"set": {"var": "message", "value": "نام کاربری یا رمز عبور خالی است"}},
          {
            "return": {
              "value": {
                "status": "{{auth_status}}",
                "message": "{{message}}"
              }
            }
          }
        ]
      }
    },
    {
      "if": {
        "condition": "{{username}} == 'admin' && {{password}} == 'admin123'",
        "then": [
          {"set": {"var": "role", "value": "admin"}},
          {"set": {"var": "auth_status", "value": "success"}}
        ],
        "else": [
          {
            "if": {
              "condition": "{{username}} == 'user' && {{password}} == 'user123'",
              "then": [
                {"set": {"var": "role", "value": "user"}},
                {"set": {"var": "auth_status", "value": "success"}}
              ],
              "else": [
                {"set": {"var": "auth_status", "value": "failed"}},
                {"set": {"var": "message", "value": "نام کاربری یا رمز عبور اشتباه است"}},
                {
                  "return": {
                    "value": {
                      "status": "{{auth_status}}",
                      "message": "{{message}}"
                    }
                  }
                }
              ]
            }
          }
        ]
      }
    },
    {
      "switch": {
        "value": "{{requested_resource}}",
        "cases": [
          {
            "case": "users",
            "actions": [
              {
                "if": {
                  "condition": "{{role}} == 'admin'",
                  "then": [
                    {"set": {"var": "access", "value": "granted"}},
                    {"set": {"var": "message", "value": "دسترسی به users داده شد"}}
                  ],
                  "else": [
                    {"set": {"var": "access", "value": "denied"}},
                    {"set": {"var": "message", "value": "فقط admin به users دسترسی دارد"}}
                  ]
                }
              }
            ]
          },
          {
            "case": "profile",
            "actions": [
              {"set": {"var": "access", "value": "granted"}},
              {"set": {"var": "message", "value": "دسترسی به profile داده شد"}}
            ]
          },
          {
            "case": "settings",
            "actions": [
              {
                "if": {
                  "condition": "{{role}} == 'admin'",
                  "then": [
                    {"set": {"var": "access", "value": "granted"}},
                    {"set": {"var": "message", "value": "دسترسی به settings داده شد"}}
                  ],
                  "else": [
                    {"set": {"var": "access", "value": "denied"}},
                    {"set": {"var": "message", "value": "فقط admin به settings دسترسی دارد"}}
                  ]
                }
              }
            ]
          }
        ],
        "default": [
          {"set": {"var": "access", "value": "denied"}},
          {"set": {"var": "message", "value": "منبع درخواستی نامعتبر است"}}
        ]
      }
    },
    {
      "return": {
        "value": {
          "username": "{{username}}",
          "role": "{{role}}",
          "auth_status": "{{auth_status}}",
          "requested_resource": "{{requested_resource}}",
          "access": "{{access}}",
          "message": "{{message}}"
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. Resolution متغیرها در شرط‌ها

✅ **درست:**
```json
{
  "condition": "{{age}} >= 18"
}
```

❌ **غلط:**
```json
{
  "condition": "age >= 18"
}
```

### 2. مقایسه رشته‌ها

```json
{
  "condition": "{{status}} == 'active'"
}
```

### 3. شرط‌های ترکیبی

```json
{
  "condition": "{{age}} >= 18 && {{is_verified}} == true"
}
```

### 4. استفاده از پرانتز

```json
{
  "condition": "({{age}} >= 18 || {{has_parent_permission}} == true) && {{is_active}} == true"
}
```

### 5. بررسی null

```json
{
  "if": {
    "condition": "{{user_id}} != null",
    "then": [
      // عملیات
    ]
  }
}
```

---

## الگوهای رایج

### 1. Validation ورودی

```json
{
  "if": {
    "condition": "{{email}} == '' || {{password}} == ''",
    "then": [
      {
        "return": {
          "value": {
            "error": "فیلدها نمی‌توانند خالی باشند"
          }
        }
      }
    ]
  }
}
```

### 2. بررسی محدوده

```json
{
  "if": {
    "condition": "{{value}} >= 0 && {{value}} <= 100",
    "then": [
      {"set": {"var": "valid", "value": true}}
    ],
    "else": [
      {"set": {"var": "valid", "value": false}}
    ]
  }
}
```

### 3. انتخاب بر اساس نوع

```json
{
  "switch": {
    "value": "{{type}}",
    "cases": [
      {"case": "type1", "actions": [...]},
      {"case": "type2", "actions": [...]},
      {"case": "type3", "actions": [...]}
    ],
    "default": [
      {"set": {"var": "error", "value": "نوع نامعتبر"}}
    ]
  }
}
```

---

## بهترین روش‌ها (Best Practices)

1. **استفاده از Switch برای چند حالت**: اگر بیش از 3 حالت دارید، از Switch استفاده کنید
2. **همیشه default تعریف کنید**: در Switch همیشه حالت default داشته باشید
3. **شرط‌ها را ساده نگه دارید**: شرط‌های پیچیده را به چند if تودرتو تبدیل کنید
4. **از پرانتز استفاده کنید**: برای وضوح بیشتر در شرط‌های ترکیبی
5. **Early Return**: اگر خطایی رخ داد، زودتر از تابع خارج شوید

---

[بازگشت به فهرست اصلی](README.md) | [بعدی: حلقه‌ها →](05-loops.md)
