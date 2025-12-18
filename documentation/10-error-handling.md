# مدیریت خطا (Error Handling)

مدیریت خطا به شما امکان می‌دهد تا خطاهای احتمالی را مدیریت کرده و از crash برنامه جلوگیری کنید.

## فهرست مطالب

- [Try/Catch/Finally](#trycatchfinally)
- [Throw (پرتاب خطا)](#throw-پرتاب-خطا)
- [دسترسی به خطا](#دسترسی-به-خطا)
- [نمونه‌های کاربردی](#نمونه‌های-کاربردی)

---

## Try/Catch/Finally

### ساختار پایه

```json
{
  "try": {
    "actions": [
      // عملیات‌هایی که ممکن است خطا بدهند
    ],
    "catch": [
      // عملیات‌هایی که در صورت خطا اجرا می‌شوند
    ],
    "finally": [
      // عملیات‌هایی که همیشه اجرا می‌شوند (اختیاری)
    ]
  }
}
```

### مثال ۱: مدیریت تقسیم بر صفر

```json
{
  "name": "Safe Division",
  "path": "/safe-divide",
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
      "try": {
        "actions": [
          {
            "if": {
              "condition": "{{b}} == 0",
              "then": [
                {
                  "throw": {
                    "message": "تقسیم بر صفر مجاز نیست"
                  }
                }
              ]
            }
          },
          {
            "math_op": {
              "operation": "divide",
              "args": ["{{a}}", "{{b}}"]
            }
          },
          {
            "set": {
              "var": "result",
              "value": "{{math_result}}"
            }
          },
          {
            "set": {
              "var": "success",
              "value": true
            }
          }
        ],
        "catch": [
          {
            "set": {
              "var": "success",
              "value": false
            }
          },
          {
            "set": {
              "var": "error_message",
              "value": "{{error.message}}"
            }
          },
          {
            "log": {
              "message": "خطا در تقسیم: {{error.message}}",
              "level": "error"
            }
          }
        ]
      }
    },
    {
      "if": {
        "condition": "{{success}} == true",
        "then": [
          {
            "return": {
              "value": {
                "success": true,
                "result": "{{result}}"
              }
            }
          }
        ],
        "else": [
          {
            "return": {
              "value": {
                "success": false,
                "error": "{{error_message}}"
              }
            }
          }
        ]
      }
    }
  ]
}
```

---

### مثال ۲: Try/Catch/Finally

```json
{
  "name": "Database Operation",
  "path": "/db-operation",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "transaction_started",
        "value": false
      }
    },
    {
      "try": {
        "actions": [
          {
            "log": {
              "message": "شروع تراکنش",
              "level": "info"
            }
          },
          {
            "set": {
              "var": "transaction_started",
              "value": true
            }
          },
          {
            "set": {
              "var": "user_id",
              "value": "{{request.payload.user_id}}"
            }
          },
          {
            "if": {
              "condition": "{{user_id}} == null",
              "then": [
                {
                  "throw": {
                    "message": "user_id نمی‌تواند خالی باشد"
                  }
                }
              ]
            }
          },
          {
            "log": {
              "message": "عملیات موفق بود",
              "level": "info"
            }
          },
          {
            "set": {
              "var": "operation_success",
              "value": true
            }
          }
        ],
        "catch": [
          {
            "log": {
              "message": "خطا رخ داد: {{error.message}}",
              "level": "error"
            }
          },
          {
            "set": {
              "var": "operation_success",
              "value": false
            }
          },
          {
            "set": {
              "var": "error_msg",
              "value": "{{error.message}}"
            }
          },
          {
            "if": {
              "condition": "{{transaction_started}} == true",
              "then": [
                {
                  "log": {
                    "message": "Rollback تراکنش",
                    "level": "warn"
                  }
                }
              ]
            }
          }
        ],
        "finally": [
          {
            "log": {
              "message": "بستن اتصال به دیتابیس",
              "level": "info"
            }
          },
          {
            "date_op": {
              "operation": "now"
            }
          },
          {
            "set": {
              "var": "completed_at",
              "value": "{{date_result}}"
            }
          }
        ]
      }
    },
    {
      "if": {
        "condition": "{{operation_success}} == true",
        "then": [
          {
            "return": {
              "value": {
                "success": true,
                "completed_at": "{{completed_at}}"
              }
            }
          }
        ],
        "else": [
          {
            "return": {
              "value": {
                "success": false,
                "error": "{{error_msg}}",
                "completed_at": "{{completed_at}}"
              }
            }
          }
        ]
      }
    }
  ]
}
```

---

## Throw (پرتاب خطا)

### ساختار پایه

```json
{
  "throw": {
    "message": "پیام خطا",
    "code": "کد خطا (اختیاری)"
  }
}
```

### مثال ۱: Validation با Throw

```json
{
  "name": "Create User",
  "path": "/create-user",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "email",
        "value": "{{request.payload.email}}"
      }
    },
    {
      "set": {
        "var": "password",
        "value": "{{request.payload.password}}"
      }
    },
    {
      "try": {
        "actions": [
          {
            "if": {
              "condition": "{{email}} == ''",
              "then": [
                {
                  "throw": {
                    "message": "ایمیل نمی‌تواند خالی باشد",
                    "code": "EMPTY_EMAIL"
                  }
                }
              ]
            }
          },
          {
            "string_op": {
              "operation": "contains",
              "value": "{{email}}",
              "args": ["@"]
            }
          },
          {
            "if": {
              "condition": "{{string_result}} == false",
              "then": [
                {
                  "throw": {
                    "message": "فرمت ایمیل نامعتبر است",
                    "code": "INVALID_EMAIL"
                  }
                }
              ]
            }
          },
          {
            "string_op": {
              "operation": "length",
              "value": "{{password}}"
            }
          },
          {
            "if": {
              "condition": "{{string_result}} < 8",
              "then": [
                {
                  "throw": {
                    "message": "رمز عبور باید حداقل 8 کاراکتر باشد",
                    "code": "WEAK_PASSWORD"
                  }
                }
              ]
            }
          },
          {
            "log": {
              "message": "کاربر ایجاد شد: {{email}}",
              "level": "info"
            }
          },
          {
            "set": {
              "var": "user_created",
              "value": true
            }
          }
        ],
        "catch": [
          {
            "set": {
              "var": "user_created",
              "value": false
            }
          },
          {
            "set": {
              "var": "error_message",
              "value": "{{error.message}}"
            }
          },
          {
            "set": {
              "var": "error_code",
              "value": "{{error.code}}"
            }
          }
        ]
      }
    },
    {
      "if": {
        "condition": "{{user_created}} == true",
        "then": [
          {
            "return": {
              "value": {
                "success": true,
                "message": "کاربر با موفقیت ایجاد شد"
              }
            }
          }
        ],
        "else": [
          {
            "return": {
              "value": {
                "success": false,
                "error": "{{error_message}}",
                "code": "{{error_code}}"
              }
            }
          }
        ]
      }
    }
  ]
}
```

---

## دسترسی به خطا

در بلاک `catch`، می‌توانید به اطلاعات خطا دسترسی داشته باشید:

- `{{error.message}}` - پیام خطا
- `{{error.code}}` - کد خطا (اگر تعریف شده باشد)
- `{{error.source}}` - منبع خطا

### مثال:

```json
{
  "try": {
    "actions": [
      {
        "throw": {
          "message": "خطای تست",
          "code": "TEST_ERROR"
        }
      }
    ],
    "catch": [
      {
        "log": {
          "message": "خطا: {{error.message}} (کد: {{error.code}})",
          "level": "error"
        }
      }
    ]
  }
}
```

---

## نمونه‌های کاربردی

### مثال ۱: API Call با Error Handling

```json
{
  "name": "Fetch User Data",
  "path": "/fetch-user",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "user_id",
        "value": "{{request.payload.user_id}}"
      }
    },
    {
      "try": {
        "actions": [
          {
            "if": {
              "condition": "{{user_id}} == null",
              "then": [
                {
                  "throw": {
                    "message": "user_id الزامی است",
                    "code": "MISSING_USER_ID"
                  }
                }
              ]
            }
          },
          {
            "log": {
              "message": "درخواست دریافت اطلاعات کاربر {{user_id}}",
              "level": "info"
            }
          },
          {
            "if": {
              "condition": "{{user_id}} < 1",
              "then": [
                {
                  "throw": {
                    "message": "user_id نامعتبر است",
                    "code": "INVALID_USER_ID"
                  }
                }
              ]
            }
          },
          {
            "set": {
              "var": "user",
              "value": {
                "id": "{{user_id}}",
                "name": "Test User",
                "email": "test@example.com"
              }
            }
          },
          {
            "set": {
              "var": "fetch_success",
              "value": true
            }
          }
        ],
        "catch": [
          {
            "set": {
              "var": "fetch_success",
              "value": false
            }
          },
          {
            "set": {
              "var": "error_info",
              "value": {
                "message": "{{error.message}}",
                "code": "{{error.code}}"
              }
            }
          },
          {
            "log": {
              "message": "خطا در دریافت کاربر: {{error.message}}",
              "level": "error"
            }
          }
        ],
        "finally": [
          {
            "date_op": {
              "operation": "now"
            }
          },
          {
            "log": {
              "message": "درخواست در {{date_result}} تکمیل شد",
              "level": "info"
            }
          }
        ]
      }
    },
    {
      "if": {
        "condition": "{{fetch_success}} == true",
        "then": [
          {
            "return": {
              "value": {
                "success": true,
                "data": "{{user}}"
              }
            }
          }
        ],
        "else": [
          {
            "return": {
              "value": {
                "success": false,
                "error": "{{error_info}}"
              }
            }
          }
        ]
      }
    }
  ]
}
```

---

### مثال ۲: Try تودرتو (Nested Try)

```json
{
  "name": "Complex Operation",
  "path": "/complex-op",
  "method": "POST",
  "logic": [
    {
      "try": {
        "actions": [
          {
            "log": {
              "message": "شروع عملیات پیچیده",
              "level": "info"
            }
          },
          {
            "try": {
              "actions": [
                {
                  "set": {
                    "var": "step1",
                    "value": "completed"
                  }
                },
                {
                  "if": {
                    "condition": "{{request.payload.test_error}} == true",
                    "then": [
                      {
                        "throw": {
                          "message": "خطای تست در مرحله 1",
                          "code": "STEP1_ERROR"
                        }
                      }
                    ]
                  }
                }
              ],
              "catch": [
                {
                  "log": {
                    "message": "خطا در مرحله 1: {{error.message}}",
                    "level": "error"
                  }
                },
                {
                  "throw": {
                    "message": "عملیات متوقف شد به دلیل خطا در مرحله 1",
                    "code": "OPERATION_FAILED"
                  }
                }
              ]
            }
          },
          {
            "set": {
              "var": "step2",
              "value": "completed"
            }
          },
          {
            "set": {
              "var": "all_success",
              "value": true
            }
          }
        ],
        "catch": [
          {
            "set": {
              "var": "all_success",
              "value": false
            }
          },
          {
            "set": {
              "var": "final_error",
              "value": "{{error.message}}"
            }
          }
        ],
        "finally": [
          {
            "log": {
              "message": "عملیات پیچیده به پایان رسید",
              "level": "info"
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "success": "{{all_success}}",
          "step1": "{{step1}}",
          "step2": "{{step2}}",
          "error": "{{final_error}}"
        }
      }
    }
  ]
}
```

---

### مثال ۳: Retry Mechanism

```json
{
  "name": "Retry Operation",
  "path": "/retry-op",
  "method": "POST",
  "logic": [
    {"set": {"var": "max_retries", "value": 3}},
    {"set": {"var": "retry_count", "value": 0}},
    {"set": {"var": "operation_success", "value": false}},
    {
      "while": {
        "condition": "{{retry_count}} < {{max_retries}} && {{operation_success}} == false",
        "body": [
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{retry_count}}", 1]
            }
          },
          {"set": {"var": "retry_count", "value": "{{math_result}}"}},
          {
            "log": {
              "message": "تلاش شماره {{retry_count}}",
              "level": "info"
            }
          },
          {
            "try": {
              "actions": [
                {
                  "if": {
                    "condition": "{{retry_count}} < 3",
                    "then": [
                      {
                        "throw": {
                          "message": "شبیه‌سازی خطا",
                          "code": "SIMULATED_ERROR"
                        }
                      }
                    ]
                  }
                },
                {
                  "log": {
                    "message": "عملیات موفق بود!",
                    "level": "info"
                  }
                },
                {"set": {"var": "operation_success", "value": true}}
              ],
              "catch": [
                {
                  "log": {
                    "message": "خطا در تلاش {{retry_count}}: {{error.message}}",
                    "level": "warn"
                  }
                },
                {
                  "if": {
                    "condition": "{{retry_count}} < {{max_retries}}",
                    "then": [
                      {"sleep": {"ms": 1000}}
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
          "success": "{{operation_success}}",
          "retries": "{{retry_count}}"
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. Finally همیشه اجرا می‌شود

```json
{
  "try": {
    "actions": [...],
    "catch": [...],
    "finally": [
      // این بلاک همیشه اجرا می‌شود
      // چه خطا رخ دهد، چه ندهد
    ]
  }
}
```

### 2. Throw فوراً به Catch می‌رود

```json
{
  "try": {
    "actions": [
      {"log": {"message": "قبل از throw"}},
      {"throw": {"message": "خطا"}},
      {"log": {"message": "این اجرا نمی‌شود"}}
    ],
    "catch": [
      {"log": {"message": "خطا گرفته شد"}}
    ]
  }
}
```

### 3. Catch می‌تواند خطا را دوباره throw کند

```json
{
  "try": {
    "actions": [...],
    "catch": [
      {"log": {"message": "Log خطا"}},
      {"throw": {"message": "{{error.message}}"}}
    ]
  }
}
```

### 4. دسترسی به Error Context

```json
{
  "catch": [
    {"set": {"var": "err_msg", "value": "{{error.message}}"}},
    {"set": {"var": "err_code", "value": "{{error.code}}"}},
    {"set": {"var": "err_source", "value": "{{error.source}}"}}
  ]
}
```

---

## الگوهای رایج

### 1. Validation Pattern

```json
{
  "try": {
    "actions": [
      {
        "if": {
          "condition": "validation_fails",
          "then": [
            {"throw": {"message": "Validation failed", "code": "VALIDATION_ERROR"}}
          ]
        }
      },
      // main logic
    ],
    "catch": [
      {"log": {"message": "Error: {{error.message}}"}},
      {"set": {"var": "result", "value": {"success": false, "error": "{{error.message}}"}}}
    ]
  }
}
```

### 2. Resource Cleanup Pattern

```json
{
  "try": {
    "actions": [
      {"set": {"var": "resource_opened", "value": true}},
      // use resource
    ],
    "finally": [
      {
        "if": {
          "condition": "{{resource_opened}} == true",
          "then": [
            {"log": {"message": "بستن منبع"}}
          ]
        }
      }
    ]
  }
}
```

### 3. Graceful Degradation

```json
{
  "try": {
    "actions": [
      // try primary method
    ],
    "catch": [
      {"log": {"message": "روش اصلی شکست خورد، استفاده از روش جایگزین"}},
      // fallback method
    ]
  }
}
```

---

## بهترین روش‌ها (Best Practices)

1. **همیشه خطاها را Log کنید**: در بلاک catch حتماً log بگیرید
2. **از Finally برای Cleanup استفاده کنید**: منابع را در finally آزاد کنید
3. **کد خطا تعریف کنید**: برای خطاها کد مشخص بگذارید
4. **پیام‌های معنادار**: پیام خطا باید واضح و مفید باشد
5. **Nested Try کم استفاده کنید**: خوانایی را حفظ کنید

---

[← قبلی: عملیات تاریخ](09-date-operations.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: اجرای موازی →](11-parallel-execution.md)
