# عملیات تاریخ (Date Operations)

عملیات تاریخ برای کار با زمان و تاریخ استفاده می‌شوند.

## فهرست مطالب

- [دریافت زمان فعلی](#دریافت-زمان-فعلی)
- [فرمت کردن تاریخ](#فرمت-کردن-تاریخ)
- [محاسبات تاریخ](#محاسبات-تاریخ)
- [نمونه‌های کاربردی](#نمونه‌های-کاربردی)

---

## دریافت زمان فعلی

### ساختار کلی

```json
{
  "date_op": {
    "operation": "نام_عملیات"
  }
}
```

نتیجه در متغیر ویژه `{{date_result}}` ذخیره می‌شود.

---

### Now (زمان فعلی)

```json
{
  "date_op": {
    "operation": "now"
  }
}
// date_result = "2024-12-18T10:30:45Z"
```

**مثال کامل:**
```json
{
  "name": "Get Current Time",
  "path": "/current-time",
  "method": "POST",
  "logic": [
    {
      "date_op": {
        "operation": "now"
      }
    },
    {
      "set": {
        "var": "current_time",
        "value": "{{date_result}}"
      }
    },
    {
      "return": {
        "value": {
          "current_time": "{{current_time}}"
        }
      }
    }
  ]
}
```

---

### Timestamp (زمان Unix)

```json
{
  "date_op": {
    "operation": "timestamp"
  }
}
// date_result = 1703154645 (Unix timestamp)
```

**مثال:**
```json
{
  "name": "Get Timestamp",
  "path": "/timestamp",
  "method": "POST",
  "logic": [
    {
      "date_op": {
        "operation": "timestamp"
      }
    },
    {
      "set": {
        "var": "ts",
        "value": "{{date_result}}"
      }
    },
    {
      "return": {
        "value": {
          "timestamp": "{{ts}}",
          "description": "Unix timestamp in seconds"
        }
      }
    }
  ]
}
```

---

## فرمت کردن تاریخ

### Format Date

```json
{
  "date_op": {
    "operation": "format",
    "value": "2024-12-18T10:30:45Z",
    "format": "%Y-%m-%d %H:%M:%S"
  }
}
// date_result = "2024-12-18 10:30:45"
```

### فرمت‌های رایج

- `%Y` - سال (4 رقم): 2024
- `%m` - ماه (2 رقم): 01-12
- `%d` - روز (2 رقم): 01-31
- `%H` - ساعت (24 ساعته): 00-23
- `%M` - دقیقه: 00-59
- `%S` - ثانیه: 00-59
- `%A` - نام روز: Monday
- `%B` - نام ماه: January

**مثال:**
```json
{
  "name": "Format Date",
  "path": "/format-date",
  "method": "POST",
  "logic": [
    {
      "date_op": {
        "operation": "now"
      }
    },
    {
      "set": {
        "var": "current",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{current}}",
        "format": "%Y-%m-%d"
      }
    },
    {
      "set": {
        "var": "date_only",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{current}}",
        "format": "%H:%M:%S"
      }
    },
    {
      "set": {
        "var": "time_only",
        "value": "{{date_result}}"
      }
    },
    {
      "return": {
        "value": {
          "iso": "{{current}}",
          "date": "{{date_only}}",
          "time": "{{time_only}}"
        }
      }
    }
  ]
}
```

---

## محاسبات تاریخ

### Add Days (اضافه کردن روز)

```json
{
  "date_op": {
    "operation": "add_days",
    "value": "2024-12-18",
    "days": 7
  }
}
// date_result = "2024-12-25"
```

### Add Hours (اضافه کردن ساعت)

```json
{
  "date_op": {
    "operation": "add_hours",
    "value": "2024-12-18T10:00:00Z",
    "hours": 5
  }
}
// date_result = "2024-12-18T15:00:00Z"
```

### Subtract Days (کم کردن روز)

```json
{
  "date_op": {
    "operation": "subtract_days",
    "value": "2024-12-18",
    "days": 3
  }
}
// date_result = "2024-12-15"
```

---

## نمونه‌های کاربردی

### مثال ۱: محاسبه تاریخ انقضا

```json
{
  "name": "Calculate Expiry Date",
  "path": "/expiry-date",
  "method": "POST",
  "logic": [
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
      "set": {
        "var": "validity_days",
        "value": "{{request.payload.validity_days}}"
      }
    },
    {
      "date_op": {
        "operation": "add_days",
        "value": "{{created_at}}",
        "days": "{{validity_days}}"
      }
    },
    {
      "set": {
        "var": "expires_at",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{expires_at}}",
        "format": "%Y-%m-%d %H:%M:%S"
      }
    },
    {
      "set": {
        "var": "expires_at_formatted",
        "value": "{{date_result}}"
      }
    },
    {
      "return": {
        "value": {
          "created_at": "{{created_at}}",
          "validity_days": "{{validity_days}}",
          "expires_at": "{{expires_at}}",
          "expires_at_formatted": "{{expires_at_formatted}}"
        }
      }
    }
  ]
}
```

**درخواست:**
```json
{
  "validity_days": 30
}
```

**پاسخ:**
```json
{
  "created_at": "2024-12-18T10:30:45Z",
  "validity_days": 30,
  "expires_at": "2025-01-17T10:30:45Z",
  "expires_at_formatted": "2025-01-17 10:30:45"
}
```

---

### مثال ۲: بررسی انقضا

```json
{
  "name": "Check Expiration",
  "path": "/check-expiry",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "expiry_date",
        "value": "{{request.payload.expiry_date}}"
      }
    },
    {
      "date_op": {
        "operation": "now"
      }
    },
    {
      "set": {
        "var": "current_time",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "timestamp",
        "value": "{{expiry_date}}"
      }
    },
    {
      "set": {
        "var": "expiry_ts",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "timestamp",
        "value": "{{current_time}}"
      }
    },
    {
      "set": {
        "var": "current_ts",
        "value": "{{date_result}}"
      }
    },
    {
      "if": {
        "condition": "{{current_ts}} > {{expiry_ts}}",
        "then": [
          {"set": {"var": "status", "value": "expired"}},
          {"set": {"var": "is_valid", "value": false}}
        ],
        "else": [
          {"set": {"var": "status", "value": "valid"}},
          {"set": {"var": "is_valid", "value": true}}
        ]
      }
    },
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{expiry_ts}}", "{{current_ts}}"]
      }
    },
    {
      "set": {
        "var": "seconds_remaining",
        "value": "{{math_result}}"
      }
    },
    {
      "return": {
        "value": {
          "expiry_date": "{{expiry_date}}",
          "current_time": "{{current_time}}",
          "status": "{{status}}",
          "is_valid": "{{is_valid}}",
          "seconds_remaining": "{{seconds_remaining}}"
        }
      }
    }
  ]
}
```

---

### مثال ۳: محاسبه سن

```json
{
  "name": "Calculate Age",
  "path": "/calculate-age",
  "method": "POST",
  "logic": [
    {
      "set": {
        "var": "birth_date",
        "value": "{{request.payload.birth_date}}"
      }
    },
    {
      "date_op": {
        "operation": "now"
      }
    },
    {
      "set": {
        "var": "current_date",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "timestamp",
        "value": "{{birth_date}}"
      }
    },
    {
      "set": {
        "var": "birth_ts",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "timestamp",
        "value": "{{current_date}}"
      }
    },
    {
      "set": {
        "var": "current_ts",
        "value": "{{date_result}}"
      }
    },
    {
      "math_op": {
        "operation": "subtract",
        "args": ["{{current_ts}}", "{{birth_ts}}"]
      }
    },
    {
      "set": {
        "var": "age_seconds",
        "value": "{{math_result}}"
      }
    },
    {
      "math_op": {
        "operation": "divide",
        "args": ["{{age_seconds}}", 31536000]
      }
    },
    {
      "math_op": {
        "operation": "floor",
        "args": ["{{math_result}}"]
      }
    },
    {
      "set": {
        "var": "age_years",
        "value": "{{math_result}}"
      }
    },
    {
      "return": {
        "value": {
          "birth_date": "{{birth_date}}",
          "current_date": "{{current_date}}",
          "age": "{{age_years}}"
        }
      }
    }
  ]
}
```

---

### مثال ۴: گزارش با زمان‌بندی

```json
{
  "name": "Create Report",
  "path": "/create-report",
  "method": "POST",
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
      "date_op": {
        "operation": "timestamp"
      }
    },
    {
      "set": {
        "var": "report_id",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{report_time}}",
        "format": "%Y-%m-%d"
      }
    },
    {
      "set": {
        "var": "report_date",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{report_time}}",
        "format": "%H:%M:%S"
      }
    },
    {
      "set": {
        "var": "report_time_only",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "subtract_days",
        "value": "{{report_time}}",
        "days": 7
      }
    },
    {
      "set": {
        "var": "period_start",
        "value": "{{date_result}}"
      }
    },
    {
      "date_op": {
        "operation": "format",
        "value": "{{period_start}}",
        "format": "%Y-%m-%d"
      }
    },
    {
      "set": {
        "var": "period_start_date",
        "value": "{{date_result}}"
      }
    },
    {
      "return": {
        "value": {
          "report_id": "{{report_id}}",
          "generated_at": "{{report_time}}",
          "report_date": "{{report_date}}",
          "report_time": "{{report_time_only}}",
          "period_start": "{{period_start_date}}",
          "period_end": "{{report_date}}",
          "title": "Weekly Report"
        }
      }
    }
  ]
}
```

---

### مثال ۵: تولید کد یکتا با زمان

```json
{
  "name": "Generate Unique Code",
  "path": "/generate-code",
  "method": "POST",
  "logic": [
    {
      "date_op": {
        "operation": "timestamp"
      }
    },
    {
      "set": {
        "var": "ts",
        "value": "{{date_result}}"
      }
    },
    {
      "math_op": {
        "operation": "mod",
        "args": ["{{ts}}", 1000000]
      }
    },
    {
      "set": {
        "var": "random_part",
        "value": "{{math_result}}"
      }
    },
    {
      "string_op": {
        "operation": "concat",
        "value": "CODE-",
        "args": ["{{random_part}}"]
      }
    },
    {
      "set": {
        "var": "unique_code",
        "value": "{{string_result}}"
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
        "operation": "add_days",
        "value": "{{created_at}}",
        "days": 1
      }
    },
    {
      "set": {
        "var": "expires_at",
        "value": "{{date_result}}"
      }
    },
    {
      "return": {
        "value": {
          "code": "{{unique_code}}",
          "created_at": "{{created_at}}",
          "expires_at": "{{expires_at}}",
          "valid_for": "24 hours"
        }
      }
    }
  ]
}
```

---

## نکات مهم

### 1. فرمت‌های تاریخ

ISO 8601 استاندارد است:
```
2024-12-18T10:30:45Z
```

### 2. Timestamp در ثانیه است

```json
{
  "date_op": {
    "operation": "timestamp"
  }
}
// date_result = 1703154645 (seconds)
```

### 3. محاسبات با Timestamp

```json
// تفاوت بین دو تاریخ
{
  "math_op": {
    "operation": "subtract",
    "args": ["{{ts2}}", "{{ts1}}"]
  }
}
// نتیجه بر حسب ثانیه
```

### 4. تبدیل ثانیه به روز

```json
{
  "math_op": {
    "operation": "divide",
    "args": ["{{seconds}}", 86400]
  }
}
// 86400 = 24 * 60 * 60
```

---

## الگوهای رایج

### 1. Log با زمان

```json
{
  "date_op": {"operation": "now"}
},
{
  "log": {
    "message": "[{{date_result}}] User logged in",
    "level": "info"
  }
}
```

### 2. Audit Trail

```json
{
  "date_op": {"operation": "now"}
},
{
  "set": {
    "var": "action",
    "value": {
      "type": "user_update",
      "timestamp": "{{date_result}}",
      "user_id": "{{user_id}}"
    }
  }
}
```

### 3. Cache با Expiry

```json
{
  "date_op": {"operation": "now"}
},
{
  "set": {"var": "cached_at", "value": "{{date_result}}"}
},
{
  "date_op": {
    "operation": "add_hours",
    "value": "{{cached_at}}",
    "hours": 1
  }
},
{
  "set": {"var": "cache_expires_at", "value": "{{date_result}}"}}
}
```

---

## لیست کامل عملیات

| عملیات | توضیح | مثال |
|--------|-------|------|
| `now` | زمان فعلی | `now()` = "2024-12-18T10:30:45Z" |
| `timestamp` | Unix timestamp | `timestamp()` = 1703154645 |
| `format` | فرمت تاریخ | `format(date, "%Y-%m-%d")` |
| `add_days` | اضافه کردن روز | `add_days(date, 7)` |
| `add_hours` | اضافه کردن ساعت | `add_hours(date, 5)` |
| `add_minutes` | اضافه کردن دقیقه | `add_minutes(date, 30)` |
| `subtract_days` | کم کردن روز | `subtract_days(date, 3)` |
| `subtract_hours` | کم کردن ساعت | `subtract_hours(date, 2)` |

---

## بهترین روش‌ها (Best Practices)

1. **استفاده از ISO 8601**: همیشه از فرمت استاندارد استفاده کنید
2. **UTC استفاده کنید**: برای جلوگیری از مشکلات timezone
3. **Timestamp برای مقایسه**: برای محاسبات از timestamp استفاده کنید
4. **Log همیشه با زمان**: در log ها زمان را ذخیره کنید
5. **Validation تاریخ**: تاریخ‌های ورودی را بررسی کنید

---

[← قبلی: عملیات رشته‌ای](08-string-operations.md) | [بازگشت به فهرست اصلی](README.md) | [بعدی: مدیریت خطا →](10-error-handling.md)
