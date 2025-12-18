# نمونه‌های کامل (Complete Examples)

این بخش شامل نمونه‌های کامل و واقعی از پروژه‌های مختلف است.

## فهرست مطالب

- [سیستم مدیریت کاربر](#سیستم-مدیریت-کاربر)
- [سیستم سفارش آنلاین](#سیستم-سفارش-آنلاین)
- [API Gateway](#api-gateway)
- [سیستم گزارش‌گیری](#سیستم-گزارش‌گیری)

---

## سیستم مدیریت کاربر

### 1. ثبت کاربر

```json
{
  "name": "Register User",
  "path": "/api/users/register",
  "method": "POST",
  "description": "ثبت کاربر جدید با validation کامل",
  "logic": [
    {
      "try": {
        "actions": [
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
            "set": {
              "var": "full_name",
              "value": "{{request.payload.full_name}}"
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
          {"set": {"var": "email", "value": "{{string_result}}"}},
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
            "date_op": {
              "operation": "now"
            }
          },
          {
            "set": {
              "var": "created_at",
              "value": "{{date_result}}"
            }
          },
          {
            "date_op": {
              "operation": "timestamp"
            }
          },
          {
            "set": {
              "var": "user_id",
              "value": "{{date_result}}"
            }
          },
          {
            "set": {
              "var": "user",
              "value": {
                "id": "{{user_id}}",
                "email": "{{email}}",
                "full_name": "{{full_name}}",
                "created_at": "{{created_at}}",
                "is_active": true,
                "role": "user"
              }
            }
          },
          {
            "log": {
              "message": "کاربر جدید ثبت شد: {{email}}",
              "level": "info"
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
            "log": {
              "message": "خطا در ثبت کاربر: {{error.message}}",
              "level": "error"
            }
          },
          {
            "set": {
              "var": "success",
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
                "user": "{{user}}",
                "message": "کاربر با موفقیت ثبت شد"
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

### 2. ورود کاربر

```json
{
  "name": "User Login",
  "path": "/api/users/login",
  "method": "POST",
  "description": "ورود کاربر و ایجاد session",
  "logic": [
    {
      "try": {
        "actions": [
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
          {"set": {"var": "email", "value": "{{string_result}}"}},
          {
            "if": {
              "condition": "{{email}} == '' || {{password}} == ''",
              "then": [
                {
                  "throw": {
                    "message": "ایمیل و رمز عبور الزامی است",
                    "code": "MISSING_CREDENTIALS"
                  }
                }
              ]
            }
          },
          {
            "if": {
              "condition": "{{email}} == 'admin@example.com' && {{password}} == 'admin123'",
              "then": [
                {"set": {"var": "user_found", "value": true}},
                {"set": {"var": "user_role", "value": "admin"}},
                {"set": {"var": "user_id", "value": 1}}
              ],
              "else": [
                {
                  "if": {
                    "condition": "{{email}} == 'user@example.com' && {{password}} == 'user123'",
                    "then": [
                      {"set": {"var": "user_found", "value": true}},
                      {"set": {"var": "user_role", "value": "user"}},
                      {"set": {"var": "user_id", "value": 2}}
                    ],
                    "else": [
                      {
                        "throw": {
                          "message": "ایمیل یا رمز عبور اشتباه است",
                          "code": "INVALID_CREDENTIALS"
                        }
                      }
                    ]
                  }
                }
              ]
            }
          },
          {
            "date_op": {
              "operation": "timestamp"
            }
          },
          {
            "set": {
              "var": "session_token",
              "value": "{{date_result}}"
            }
          },
          {
            "date_op": {
              "operation": "now"
            }
          },
          {
            "set": {
              "var": "login_time",
              "value": "{{date_result}}"
            }
          },
          {
            "date_op": {
              "operation": "add_hours",
              "value": "{{login_time}}",
              "hours": 24
            }
          },
          {
            "set": {
              "var": "token_expires",
              "value": "{{date_result}}"
            }
          },
          {
            "log": {
              "message": "کاربر وارد شد: {{email}} ({{user_role}})",
              "level": "info"
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
            "log": {
              "message": "خطا در ورود: {{error.message}}",
              "level": "error"
            }
          },
          {
            "set": {
              "var": "success",
              "value": false
            }
          },
          {
            "set": {
              "var": "error_msg",
              "value": "{{error.message}}"
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
                "session": {
                  "token": "{{session_token}}",
                  "expires_at": "{{token_expires}}",
                  "user": {
                    "id": "{{user_id}}",
                    "email": "{{email}}",
                    "role": "{{user_role}}"
                  }
                }
              }
            }
          }
        ],
        "else": [
          {
            "return": {
              "value": {
                "success": false,
                "error": "{{error_msg}}"
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

## سیستم سفارش آنلاین

### سفارش جدید با محاسبه کامل

```json
{
  "name": "Create Order",
  "path": "/api/orders/create",
  "method": "POST",
  "description": "ایجاد سفارش جدید با محاسبه قیمت، تخفیف، و مالیات",
  "logic": [
    {
      "try": {
        "actions": [
          {
            "set": {
              "var": "user_id",
              "value": "{{request.payload.user_id}}"
            }
          },
          {
            "set": {
              "var": "items",
              "value": "{{request.payload.items}}"
            }
          },
          {
            "set": {
              "var": "coupon_code",
              "value": "{{request.payload.coupon_code}}"
            }
          },
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
          {"set": {"var": "subtotal", "value": 0}},
          {
            "foreach": {
              "collection": "{{items}}",
              "item_var": "item",
              "index_var": "i",
              "body": [
                {
                  "math_op": {
                    "operation": "multiply",
                    "args": ["{{item.price}}", "{{item.quantity}}"]
                  }
                },
                {"set": {"var": "item_total", "value": "{{math_result}}"}},
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{subtotal}}", "{{item_total}}"]
                  }
                },
                {"set": {"var": "subtotal", "value": "{{math_result}}"}}
              ]
            }
          },
          {"set": {"var": "discount_amount", "value": 0}},
          {"set": {"var": "discount_percent", "value": 0}},
          {
            "switch": {
              "value": "{{coupon_code}}",
              "cases": [
                {
                  "case": "SAVE10",
                  "actions": [
                    {"set": {"var": "discount_percent", "value": 10}}
                  ]
                },
                {
                  "case": "SAVE20",
                  "actions": [
                    {"set": {"var": "discount_percent", "value": 20}}
                  ]
                },
                {
                  "case": "SAVE30",
                  "actions": [
                    {"set": {"var": "discount_percent", "value": 30}}
                  ]
                }
              ]
            }
          },
          {
            "if": {
              "condition": "{{discount_percent}} > 0",
              "then": [
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
                {"set": {"var": "discount_amount", "value": "{{math_result}}"}}
              ]
            }
          },
          {
            "math_op": {
              "operation": "subtract",
              "args": ["{{subtotal}}", "{{discount_amount}}"]
            }
          },
          {"set": {"var": "after_discount", "value": "{{math_result}}"}},
          {"set": {"var": "tax_rate", "value": 9}},
          {
            "math_op": {
              "operation": "multiply",
              "args": ["{{after_discount}}", "{{tax_rate}}"]
            }
          },
          {
            "math_op": {
              "operation": "divide",
              "args": ["{{math_result}}", 100]
            }
          },
          {"set": {"var": "tax_amount", "value": "{{math_result}}"}},
          {
            "math_op": {
              "operation": "sum",
              "args": ["{{after_discount}}", "{{tax_amount}}"]
            }
          },
          {"set": {"var": "total", "value": "{{math_result}}"}},
          {
            "date_op": {
              "operation": "now"
            }
          },
          {
            "set": {
              "var": "created_at",
              "value": "{{date_result}}"
            }
          },
          {
            "date_op": {
              "operation": "timestamp"
            }
          },
          {
            "set": {
              "var": "order_id",
              "value": "{{date_result}}"
            }
          },
          {
            "set": {
              "var": "order",
              "value": {
                "order_id": "{{order_id}}",
                "user_id": "{{user_id}}",
                "items": "{{items}}",
                "subtotal": "{{subtotal}}",
                "discount": {
                  "code": "{{coupon_code}}",
                  "percent": "{{discount_percent}}",
                  "amount": "{{discount_amount}}"
                },
                "tax": {
                  "rate": "{{tax_rate}}",
                  "amount": "{{tax_amount}}"
                },
                "total": "{{total}}",
                "status": "pending",
                "created_at": "{{created_at}}"
              }
            }
          },
          {
            "log": {
              "message": "سفارش جدید ایجاد شد: {{order_id}} (مبلغ: {{total}})",
              "level": "info"
            }
          },
          {"set": {"var": "success", "value": true}}
        ],
        "catch": [
          {
            "log": {
              "message": "خطا در ایجاد سفارش: {{error.message}}",
              "level": "error"
            }
          },
          {"set": {"var": "success", "value": false}},
          {"set": {"var": "error_msg", "value": "{{error.message}}"}}
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
                "order": "{{order}}"
              }
            }
          }
        ],
        "else": [
          {
            "return": {
              "value": {
                "success": false,
                "error": "{{error_msg}}"
              }
            }
          }
        ]
      }
    }
  ]
}
```

**درخواست:**
```json
{
  "user_id": 123,
  "items": [
    {"id": 1, "name": "Laptop", "price": 1000, "quantity": 1},
    {"id": 2, "name": "Mouse", "price": 25, "quantity": 2}
  ],
  "coupon_code": "SAVE20"
}
```

**پاسخ:**
```json
{
  "success": true,
  "order": {
    "order_id": "1703154645",
    "user_id": 123,
    "items": [...],
    "subtotal": 1050,
    "discount": {
      "code": "SAVE20",
      "percent": 20,
      "amount": 210
    },
    "tax": {
      "rate": 9,
      "amount": 75.6
    },
    "total": 915.6,
    "status": "pending",
    "created_at": "2024-12-18T10:30:45Z"
  }
}
```

---

## API Gateway

### درخواست با Retry و Fallback

```json
{
  "name": "API Gateway with Retry",
  "path": "/api/gateway/fetch",
  "method": "POST",
  "description": "درخواست به API خارجی با retry و fallback",
  "logic": [
    {"set": {"var": "api_url", "value": "{{request.payload.api_url}}"}},
    {"set": {"var": "max_retries", "value": 3}},
    {"set": {"var": "retry_count", "value": 0}},
    {"set": {"var": "request_success", "value": false}},
    {
      "while": {
        "condition": "{{retry_count}} < {{max_retries}} && {{request_success}} == false",
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
              "message": "تلاش {{retry_count}} برای {{api_url}}",
              "level": "info"
            }
          },
          {
            "try": {
              "actions": [
                {
                  "if": {
                    "condition": "{{request.payload.simulate_error}} == true && {{retry_count}} < 3",
                    "then": [
                      {
                        "throw": {
                          "message": "شبیه‌سازی خطای شبکه",
                          "code": "NETWORK_ERROR"
                        }
                      }
                    ]
                  }
                },
                {
                  "set": {
                    "var": "api_response",
                    "value": {
                      "status": "success",
                      "data": {"message": "Response from API"}
                    }
                  }
                },
                {"set": {"var": "request_success", "value": true}},
                {
                  "log": {
                    "message": "درخواست موفق بود",
                    "level": "info"
                  }
                }
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
                      {
                        "log": {
                          "message": "انتظار قبل از تلاش مجدد...",
                          "level": "info"
                        }
                      },
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
      "if": {
        "condition": "{{request_success}} == false",
        "then": [
          {
            "log": {
              "message": "تمام تلاش‌ها شکست خورد، استفاده از fallback",
              "level": "warn"
            }
          },
          {
            "set": {
              "var": "api_response",
              "value": {
                "status": "fallback",
                "data": {"message": "Cached or default response"}
              }
            }
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "success": "{{request_success}}",
          "retries": "{{retry_count}}",
          "response": "{{api_response}}"
        }
      }
    }
  ]
}
```

---

## سیستم گزارش‌گیری

### گزارش فروش ماهانه

```json
{
  "name": "Monthly Sales Report",
  "path": "/api/reports/monthly-sales",
  "method": "POST",
  "description": "تولید گزارش فروش ماهانه با آمار کامل",
  "logic": [
    {
      "date_op": {
        "operation": "now"
      }
    },
    {
      "set": {
        "var": "report_time",
        "value": "{{date_result}}"
      }
    },
    {
      "parallel": {
        "tasks": [
          {
            "name": "fetch_orders",
            "actions": [
              {
                "log": {
                  "message": "دریافت سفارشات...",
                  "level": "info"
                }
              },
              {"sleep": {"ms": 500}},
              {
                "set": {
                  "var": "orders",
                  "value": [
                    {"id": 1, "amount": 1000, "status": "completed"},
                    {"id": 2, "amount": 2000, "status": "completed"},
                    {"id": 3, "amount": 1500, "status": "cancelled"},
                    {"id": 4, "amount": 3000, "status": "completed"}
                  ]
                }
              }
            ]
          },
          {
            "name": "fetch_customers",
            "actions": [
              {
                "log": {
                  "message": "دریافت مشتریان...",
                  "level": "info"
                }
              },
              {"sleep": {"ms": 300}},
              {"set": {"var": "total_customers", "value": 150}},
              {"set": {"var": "new_customers", "value": 25}}
            ]
          },
          {
            "name": "fetch_products",
            "actions": [
              {
                "log": {
                  "message": "دریافت محصولات...",
                  "level": "info"
                }
              },
              {"sleep": {"ms": 400}},
              {"set": {"var": "total_products", "value": 50}},
              {"set": {"var": "out_of_stock", "value": 5}}
            ]
          }
        ]
      }
    },
    {"set": {"var": "total_revenue", "value": 0}},
    {"set": {"var": "completed_orders", "value": 0}},
    {"set": {"var": "cancelled_orders", "value": 0}},
    {
      "foreach": {
        "collection": "{{orders}}",
        "item_var": "order",
        "index_var": "i",
        "body": [
          {
            "if": {
              "condition": "{{order.status}} == 'completed'",
              "then": [
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{total_revenue}}", "{{order.amount}}"]
                  }
                },
                {"set": {"var": "total_revenue", "value": "{{math_result}}"}},
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{completed_orders}}", 1]
                  }
                },
                {"set": {"var": "completed_orders", "value": "{{math_result}}"}}
              ],
              "else": [
                {
                  "math_op": {
                    "operation": "sum",
                    "args": ["{{cancelled_orders}}", 1]
                  }
                },
                {"set": {"var": "cancelled_orders", "value": "{{math_result}}"}}
              ]
            }
          }
        ]
      }
    },
    {
      "math_op": {
        "operation": "sum",
        "args": ["{{completed_orders}}", "{{cancelled_orders}}"]
      }
    },
    {"set": {"var": "total_orders", "value": "{{math_result}}"}},
    {
      "if": {
        "condition": "{{completed_orders}} > 0",
        "then": [
          {
            "math_op": {
              "operation": "divide",
              "args": ["{{total_revenue}}", "{{completed_orders}}"]
            }
          },
          {
            "math_op": {
              "operation": "round",
              "args": ["{{math_result}}"]
            }
          },
          {"set": {"var": "avg_order_value", "value": "{{math_result}}"}}
        ],
        "else": [
          {"set": {"var": "avg_order_value", "value": 0}}
        ]
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{report_time}}",
        "format": "%Y-%m-%d %H:%M:%S"
      }
    },
    {
      "set": {
        "var": "report_timestamp",
        "value": "{{date_result}}"
      }
    },
    {
      "set": {
        "var": "report",
        "value": {
          "title": "گزارش فروش ماهانه",
          "generated_at": "{{report_timestamp}}",
          "sales": {
            "total_revenue": "{{total_revenue}}",
            "total_orders": "{{total_orders}}",
            "completed_orders": "{{completed_orders}}",
            "cancelled_orders": "{{cancelled_orders}}",
            "average_order_value": "{{avg_order_value}}"
          },
          "customers": {
            "total": "{{total_customers}}",
            "new_this_month": "{{new_customers}}"
          },
          "products": {
            "total": "{{total_products}}",
            "out_of_stock": "{{out_of_stock}}"
          }
        }
      }
    },
    {
      "log": {
        "message": "گزارش تولید شد: درآمد کل = {{total_revenue}}",
        "level": "info"
      }
    },
    {
      "return": {
        "value": {
          "success": true,
          "report": "{{report}}"
        }
      }
    }
  ]
}
```

**پاسخ:**
```json
{
  "success": true,
  "report": {
    "title": "گزارش فروش ماهانه",
    "generated_at": "2024-12-18 10:30:45",
    "sales": {
      "total_revenue": 6000,
      "total_orders": 4,
      "completed_orders": 3,
      "cancelled_orders": 1,
      "average_order_value": 2000
    },
    "customers": {
      "total": 150,
      "new_this_month": 25
    },
    "products": {
      "total": 50,
      "out_of_stock": 5
    }
  }
}
```

---

## نکات پیاده‌سازی

### 1. Validation همیشه در ابتدا

```json
[
  {"set": {"var": "input", "value": "{{request.payload.input}}"}},
  {
    "if": {
      "condition": "{{input}} == null",
      "then": [
        {"throw": {"message": "input is required"}}
      ]
    }
  },
  // ادامه منطق
]
```

### 2. استفاده از Try/Catch برای عملیات خطرناک

```json
{
  "try": {
    "actions": [
      // عملیات اصلی
    ],
    "catch": [
      {"log": {"message": "Error: {{error.message}}"}},
      // fallback یا error response
    ]
  }
}
```

### 3. Logging مناسب

```json
{"log": {"message": "شروع عملیات X", "level": "info"}},
// عملیات
{"log": {"message": "عملیات X تکمیل شد", "level": "info"}}
```

### 4. استفاده از Parallel برای بهینه‌سازی

```json
{
  "parallel": {
    "tasks": [
      {"name": "fetch_data1", "actions": [...]},
      {"name": "fetch_data2", "actions": [...]},
      {"name": "fetch_data3", "actions": [...]}
    ]
  }
}
```

---

## بهترین روش‌ها

1. **ساختار واضح**: کد را به بخش‌های منطقی تقسیم کنید
2. **Error Handling جامع**: همه خطاهای ممکن را مدیریت کنید
3. **Logging کافی**: برای debug کردن لاگ بگیرید
4. **Validation کامل**: ورودی‌ها را کامل بررسی کنید
5. **Performance**: از Parallel برای عملیات مستقل استفاده کنید
6. **Clean Response**: پاسخ‌های واضح و استاندارد برگردانید
7. **Documentation**: توضیحات در description بنویسید

---

[← قبلی: اجرای موازی](11-parallel-execution.md) | [بازگشت به فهرست اصلی](README.md)
