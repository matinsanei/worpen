# اجرای موازی (Parallel Execution)

اجرای موازی به شما امکان می‌دهد تا چندین task را به صورت همزمان اجرا کنید و از CPU و زمان بهتر استفاده کنید.

## فهرست مطالب

- [ساختار پایه](#ساختار-پایه)
- [محدودیت همزمانی](#محدودیت-همزمانی)
- [نمونه‌های کاربردی](#نمونه‌های-کاربردی)

---

## ساختار پایه

```json
{
  "parallel": {
    "tasks": [
      {
        "name": "task1",
        "actions": [
          // عملیات task 1
        ]
      },
      {
        "name": "task2",
        "actions": [
          // عملیات task 2
        ]
      }
    ],
    "max_concurrent": 2
  }
}
```

### پارامترها

- `tasks`: آرایه‌ای از taskها
- `max_concurrent`: حداکثر تعداد taskهایی که همزمان اجرا می‌شوند (اختیاری، پیش‌فرض: همه)

### مثال ۱: اجرای موازی ساده

```json
{
  "name": "Parallel Tasks",
  "path": "/parallel-simple",
  "method": "POST",
  "logic": [
    {
      "parallel": {
        "tasks": [
          {
            "name": "task1",
            "actions": [
              {
                "log": {
                  "message": "Task 1 شروع شد",
                  "level": "info"
                }
              },
              {
                "sleep": {
                  "ms": 1000
                }
              },
              {
                "set": {
                  "var": "task1_result",
                  "value": "Task 1 تکمیل شد"
                }
              }
            ]
          },
          {
            "name": "task2",
            "actions": [
              {
                "log": {
                  "message": "Task 2 شروع شد",
                  "level": "info"
                }
              },
              {
                "sleep": {
                  "ms": 1000
                }
              },
              {
                "set": {
                  "var": "task2_result",
                  "value": "Task 2 تکمیل شد"
                }
              }
            ]
          },
          {
            "name": "task3",
            "actions": [
              {
                "log": {
                  "message": "Task 3 شروع شد",
                  "level": "info"
                }
              },
              {
                "sleep": {
                  "ms": 1000
                }
              },
              {
                "set": {
                  "var": "task3_result",
                  "value": "Task 3 تکمیل شد"
                }
              }
            ]
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "task1": "{{task1_result}}",
          "task2": "{{task2_result}}",
          "task3": "{{task3_result}}"
        }
      }
    }
  ]
}
```

**نکته:** 3 task به صورت همزمان اجرا می‌شوند، بنابراین زمان کل حدود 1 ثانیه است (نه 3 ثانیه).

---

## محدودیت همزمانی

با `max_concurrent` می‌توانید تعداد taskهای همزمان را محدود کنید:

```json
{
  "parallel": {
    "tasks": [
      {"name": "task1", "actions": [...]},
      {"name": "task2", "actions": [...]},
      {"name": "task3", "actions": [...]},
      {"name": "task4", "actions": [...]}
    ],
    "max_concurrent": 2
  }
}
```

**نحوه اجرا:**
1. task1 و task2 همزمان شروع می‌شوند
2. وقتی یکی تمام شد، task3 شروع می‌شود
3. وقتی یکی دیگر تمام شد، task4 شروع می‌شود

### مثال ۲: محدودیت همزمانی

```json
{
  "name": "Limited Concurrency",
  "path": "/parallel-limited",
  "method": "POST",
  "logic": [
    {
      "date_op": {
        "operation": "timestamp"
      }
    },
    {
      "set": {
        "var": "start_time",
        "value": "{{date_result}}"
      }
    },
    {
      "parallel": {
        "tasks": [
          {
            "name": "task1",
            "actions": [
              {"log": {"message": "Task 1 start"}},
              {"sleep": {"ms": 2000}},
              {"set": {"var": "task1_done", "value": true}}
            ]
          },
          {
            "name": "task2",
            "actions": [
              {"log": {"message": "Task 2 start"}},
              {"sleep": {"ms": 2000}},
              {"set": {"var": "task2_done", "value": true}}
            ]
          },
          {
            "name": "task3",
            "actions": [
              {"log": {"message": "Task 3 start"}},
              {"sleep": {"ms": 2000}},
              {"set": {"var": "task3_done", "value": true}}
            ]
          },
          {
            "name": "task4",
            "actions": [
              {"log": {"message": "Task 4 start"}},
              {"sleep": {"ms": 2000}},
              {"set": {"var": "task4_done", "value": true}}
            ]
          }
        ],
        "max_concurrent": 2
      }
    },
    {
      "date_op": {
        "operation": "timestamp"
      }
    },
    {
      "set": {
        "var": "end_time",
        "value": "{{date_result}}"
      }
    },
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{end_time}}", "{{start_time}}"]
      }
    },
    {
      "set": {
        "var": "duration",
        "value": "{{math_result}}"
      }
    },
    {
      "return": {
        "value": {
          "task1": "{{task1_done}}",
          "task2": "{{task2_done}}",
          "task3": "{{task3_done}}",
          "task4": "{{task4_done}}",
          "duration_seconds": "{{duration}}",
          "note": "با max_concurrent=2، زمان حدود 4 ثانیه است"
        }
      }
    }
  ]
}
```

---

## نمونه‌های کاربردی

### مثال ۱: پردازش چند API همزمان

```json
{
  "name": "Fetch Multiple APIs",
  "path": "/fetch-apis",
  "method": "POST",
  "logic": [
    {
      "parallel": {
        "tasks": [
          {
            "name": "fetch_users",
            "actions": [
              {
                "log": {
                  "message": "Fetching users...",
                  "level": "info"
                }
              },
              {
                "sleep": {
                  "ms": 1500
                }
              },
              {
                "set": {
                  "var": "users",
                  "value": [
                    {"id": 1, "name": "Ali"},
                    {"id": 2, "name": "Sara"}
                  ]
                }
              },
              {
                "log": {
                  "message": "Users fetched",
                  "level": "info"
                }
              }
            ]
          },
          {
            "name": "fetch_products",
            "actions": [
              {
                "log": {
                  "message": "Fetching products...",
                  "level": "info"
                }
              },
              {
                "sleep": {
                  "ms": 2000
                }
              },
              {
                "set": {
                  "var": "products",
                  "value": [
                    {"id": 1, "name": "Laptop"},
                    {"id": 2, "name": "Phone"}
                  ]
                }
              },
              {
                "log": {
                  "message": "Products fetched",
                  "level": "info"
                }
              }
            ]
          },
          {
            "name": "fetch_orders",
            "actions": [
              {
                "log": {
                  "message": "Fetching orders...",
                  "level": "info"
                }
              },
              {
                "sleep": {
                  "ms": 1000
                }
              },
              {
                "set": {
                  "var": "orders",
                  "value": [
                    {"id": 1, "total": 1000},
                    {"id": 2, "total": 2000}
                  ]
                }
              },
              {
                "log": {
                  "message": "Orders fetched",
                  "level": "info"
                }
              }
            ]
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "users": "{{users}}",
          "products": "{{products}}",
          "orders": "{{orders}}"
        }
      }
    }
  ]
}
```

---

### مثال ۲: پردازش موازی با Error Handling

```json
{
  "name": "Parallel with Error Handling",
  "path": "/parallel-errors",
  "method": "POST",
  "logic": [
    {
      "parallel": {
        "tasks": [
          {
            "name": "safe_task",
            "actions": [
              {
                "try": {
                  "actions": [
                    {"sleep": {"ms": 1000}},
                    {"set": {"var": "safe_result", "value": "success"}}
                  ],
                  "catch": [
                    {"set": {"var": "safe_result", "value": "failed"}}
                  ]
                }
              }
            ]
          },
          {
            "name": "error_task",
            "actions": [
              {
                "try": {
                  "actions": [
                    {"sleep": {"ms": 500}},
                    {
                      "if": {
                        "condition": "{{request.payload.cause_error}} == true",
                        "then": [
                          {
                            "throw": {
                              "message": "خطای شبیه‌سازی شده",
                              "code": "TEST_ERROR"
                            }
                          }
                        ]
                      }
                    },
                    {"set": {"var": "error_task_result", "value": "success"}}
                  ],
                  "catch": [
                    {"set": {"var": "error_task_result", "value": "failed"}},
                    {"set": {"var": "error_message", "value": "{{error.message}}"}}
                  ]
                }
              }
            ]
          },
          {
            "name": "calculation_task",
            "actions": [
              {"sleep": {"ms": 800}},
              {"math_op": {"operation": "sum", "args": [10, 20, 30]}},
              {"set": {"var": "calc_result", "value": "{{math_result}}"}}
            ]
          }
        ]
      }
    },
    {
      "return": {
        "value": {
          "safe_task": "{{safe_result}}",
          "error_task": "{{error_task_result}}",
          "error_message": "{{error_message}}",
          "calculation": "{{calc_result}}"
        }
      }
    }
  ]
}
```

---

### مثال ۳: پردازش موازی آیتم‌های لیست

```json
{
  "name": "Process List in Parallel",
  "path": "/parallel-list",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "items",
        "value": [
          {"id": 1, "value": 10},
          {"id": 2, "value": 20},
          {"id": 3, "value": 30},
          {"id": 4, "value": 40}
        ]
      }
    },
    {
      "parallel": {
        "tasks": [
          {
            "name": "process_item_1",
            "actions": [
              {
                "math_op": {
                  "operation": "multiply",
                  "args": ["{{items.0.value}}", 2]
                }
              },
              {"set": {"var": "result_1", "value": "{{math_result}}"}}
            ]
          },
          {
            "name": "process_item_2",
            "actions": [
              {
                "math_op": {
                  "operation": "multiply",
                  "args": ["{{items.1.value}}", 2]
                }
              },
              {"set": {"var": "result_2", "value": "{{math_result}}"}}
            ]
          },
          {
            "name": "process_item_3",
            "actions": [
              {
                "math_op": {
                  "operation": "multiply",
                  "args": ["{{items.2.value}}", 2]
                }
              },
              {"set": {"var": "result_3", "value": "{{math_result}}"}}
            ]
          },
          {
            "name": "process_item_4",
            "actions": [
              {
                "math_op": {
                  "operation": "multiply",
                  "args": ["{{items.3.value}}", 2]
                }
              },
              {"set": {"var": "result_4", "value": "{{math_result}}"}}
            ]
          }
        ],
        "max_concurrent": 2
      }
    },
    {
      "return": {
        "value": {
          "results": [
            "{{result_1}}",
            "{{result_2}}",
            "{{result_3}}",
            "{{result_4}}"
          ]
        }
      }
    }
  ]
}
```

---

### مثال ۴: Dashboard Data Fetching

```json
{
  "name": "Dashboard Data",
  "path": "/dashboard",
  "method": "POST",
  "logic": [
    {
      "date_op": {
        "operation": "timestamp"
      }
    },
    {
      "set": {
        "var": "fetch_start",
        "value": "{{date_result}}"
      }
    },
    {
      "parallel": {
        "tasks": [
          {
            "name": "get_user_count",
            "actions": [
              {"sleep": {"ms": 500}},
              {"set": {"var": "total_users", "value": 1234}}
            ]
          },
          {
            "name": "get_revenue",
            "actions": [
              {"sleep": {"ms": 800}},
              {"set": {"var": "monthly_revenue", "value": 50000}}
            ]
          },
          {
            "name": "get_orders",
            "actions": [
              {"sleep": {"ms": 600}},
              {"set": {"var": "total_orders", "value": 567}}
            ]
          },
          {
            "name": "get_active_sessions",
            "actions": [
              {"sleep": {"ms": 300}},
              {"set": {"var": "active_sessions", "value": 89}}
            ]
          },
          {
            "name": "calculate_stats",
            "actions": [
              {"sleep": {"ms": 400}},
              {"math_op": {"operation": "avg", "args": [100, 200, 300]}},
              {"set": {"var": "avg_value", "value": "{{math_result}}"}}
            ]
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
        "var": "fetch_end",
        "value": "{{date_result}}"
      }
    },
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{fetch_end}}", "{{fetch_start}}"]
      }
    },
    {
      "set": {
        "var": "fetch_duration",
        "value": "{{math_result}}"
      }
    },
    {
      "return": {
        "value": {
          "dashboard": {
            "total_users": "{{total_users}}",
            "monthly_revenue": "{{monthly_revenue}}",
            "total_orders": "{{total_orders}}",
            "active_sessions": "{{active_sessions}}",
            "avg_value": "{{avg_value}}"
          },
          "metadata": {
            "fetch_duration_seconds": "{{fetch_duration}}",
            "note": "همه داده‌ها به صورت موازی دریافت شدند"
          }
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. متغیرهای مشترک

هر task می‌تواند متغیرهای خود را set کند و این متغیرها در context مشترک قابل دسترسی هستند:

```json
{
  "parallel": {
    "tasks": [
      {
        "name": "task1",
        "actions": [
          {"set": {"var": "x", "value": 10}}
        ]
      },
      {
        "name": "task2",
        "actions": [
          {"set": {"var": "y", "value": 20}}
        ]
      }
    ]
  }
},
{
  "math_op": {
    "operation": "sum",
    "args": ["{{x}}", "{{y}}"]
  }
}
// x و y هر دو قابل دسترسی هستند
```

### 2. Race Condition

مراقب race condition باشید:

❌ **خطر:**
```json
{
  "parallel": {
    "tasks": [
      {
        "name": "task1",
        "actions": [
          {"set": {"var": "counter", "value": 1}}
        ]
      },
      {
        "name": "task2",
        "actions": [
          {"set": {"var": "counter", "value": 2}}
        ]
      }
    ]
  }
}
// counter ممکن است 1 یا 2 باشد (نامشخص)
```

### 3. Error در یک Task

اگر یک task خطا بدهد، بقیه taskها همچنان اجرا می‌شوند:

```json
{
  "parallel": {
    "tasks": [
      {
        "name": "failing_task",
        "actions": [
          {
            "try": {
              "actions": [
                {"throw": {"message": "خطا"}}
              ],
              "catch": [
                {"set": {"var": "task1_failed", "value": true}}
              ]
            }
          }
        ]
      },
      {
        "name": "success_task",
        "actions": [
          {"set": {"var": "task2_success", "value": true}}
        ]
      }
    ]
  }
}
```

### 4. Performance

اجرای موازی زمان کل را کاهش می‌دهد:

- **بدون Parallel:** task1 (2s) + task2 (2s) + task3 (2s) = 6 ثانیه
- **با Parallel:** max(2s, 2s, 2s) = 2 ثانیه

---

## الگوهای رایج

### 1. Parallel API Calls

```json
{
  "parallel": {
    "tasks": [
      {"name": "api1", "actions": [/* fetch from API 1 */]},
      {"name": "api2", "actions": [/* fetch from API 2 */]},
      {"name": "api3", "actions": [/* fetch from API 3 */]}
    ]
  }
}
```

### 2. Batch Processing

```json
{
  "parallel": {
    "tasks": [
      {"name": "batch1", "actions": [/* process items 1-100 */]},
      {"name": "batch2", "actions": [/* process items 101-200 */]},
      {"name": "batch3", "actions": [/* process items 201-300 */]}
    ],
    "max_concurrent": 3
  }
}
```

### 3. Multi-Source Aggregation

```json
{
  "parallel": {
    "tasks": [
      {"name": "db_data", "actions": [/* fetch from DB */]},
      {"name": "cache_data", "actions": [/* fetch from cache */]},
      {"name": "api_data", "actions": [/* fetch from API */]}
    ]
  }
},
{
  "set": {
    "var": "aggregated",
    "value": {
      "db": "{{db_result}}",
      "cache": "{{cache_result}}",
      "api": "{{api_result}}"
    }
  }
}
```

---

## بهترین روش‌ها (Best Practices)

1. **استفاده برای I/O Operations**: Parallel برای عملیات I/O (API, DB, File) مفید است
2. **محدود کردن Concurrency**: با `max_concurrent` از overload جلوگیری کنید
3. **Error Handling در هر Task**: از try/catch در هر task استفاده کنید
4. **نام‌گذاری واضح**: نام taskها را معنادار انتخاب کنید
5. **اجتناب از Race Condition**: از تغییر همزمان یک متغیر جلوگیری کنید
6. **Logging**: در هر task شروع و پایان را log کنید

---

## مقایسه: Sequential vs Parallel

### Sequential (ترتیبی):
```json
[
  {"sleep": {"ms": 1000}},  // 1s
  {"sleep": {"ms": 1000}},  // 1s
  {"sleep": {"ms": 1000}}   // 1s
]
// کل: 3 ثانیه
```

### Parallel (موازی):
```json
{
  "parallel": {
    "tasks": [
      {"name": "t1", "actions": [{"sleep": {"ms": 1000}}]},
      {"name": "t2", "actions": [{"sleep": {"ms": 1000}}]},
      {"name": "t3", "actions": [{"sleep": {"ms": 1000}}]}
    ]
  }
}
// کل: 1 ثانیه
```

---

[← قبلی: مدیریت خطا](10-error-handling.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: نمونه‌های کامل →](12-complete-examples.md)
