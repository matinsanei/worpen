# 🚀 Extensions Quick Start Guide

## راهنمای سریع استفاده از Custom Operations

این راهنما در **3 دقیقه** به شما نشان می‌دهد چطوری operation های سفارشی بسازید.

---

## مرحله 1: تعریف Operation (30 ثانیه)

1. منوی **Settings** را باز کنید
2. به **🧩 Extensions / Custom Operations** بروید
3. فیلدهای زیر را پر کنید:

**Operation Name:**
```
NotifyOp
```

**JSON Schema:**
```json
{
  "type": "object",
  "required": ["message"],
  "properties": {
    "message": {
      "type": "string",
      "description": "پیام اعلان (از {{متغیرها}} پشتیبانی می‌کند)"
    },
    "priority": {
      "type": "string",
      "enum": ["low", "medium", "high"],
      "default": "medium"
    }
  }
}
```

4. روی **Register Custom Schema** کلیک کنید

✅ **تمام!** Schema ذخیره شد.

---

## مرحله 2: استفاده در Route (1 دقیقه)

1. به **Route Builder** بروید
2. Route جدید بسازید:

```json
{
  "name": "test_notify",
  "path": "/api/notify",
  "method": "POST",
  "logic": [
    {
      "NotifyOp": {
        "message": "{{request.body.text}}",
        "priority": "high"
      }
    },
    {
      "return": {
        "value": {"status": "ok"}
      }
    }
  ]
}
```

3. **Save** کنید

---

## مرحله 3: تست (1 دقیقه)

PowerShell باز کنید و دستور زیر را اجرا کنید:

```powershell
Invoke-WebRequest `
  -Uri "http://127.0.0.1:3000/api/notify" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"text":"Hello Custom Operation!"}' `
  -UseBasicParsing
```

---

## چه اتفاقی افتاد؟

### 1. Monaco Editor
- ✅ `NotifyOp` رو شناخت
- ✅ Autocomplete داد
- ✅ Validation کرد

### 2. Backend
- ✅ Route رو پذیرفت (بدون error!)
- ✅ Variable رو scope کرد: `{{request.body.text}}`

### 3. فایل Compiled
برو به `backend/data/routes/` و آخرین فایل رو باز کن. می‌بینی:

```json
{
  "NotifyOp": {
    "message": "{{request.body.text}}",  // ← Variable scoped!
    "priority": "high"
  }
}
```

---

## مثال‌های بیشتر

### Email Operation

**Schema:**
```json
{
  "type": "object",
  "required": ["to", "subject"],
  "properties": {
    "to": {"type": "string", "format": "email"},
    "subject": {"type": "string"},
    "body": {"type": "string"}
  }
}
```

**استفاده:**
```json
{
  "EmailOp": {
    "to": "{{user_email}}",
    "subject": "خوش آمدید",
    "body": "سلام {{user_name}}"
  }
}
```

### SMS Operation

**Schema:**
```json
{
  "type": "object",
  "required": ["phone", "text"],
  "properties": {
    "phone": {"type": "string"},
    "text": {"type": "string", "maxLength": 160}
  }
}
```

**استفاده:**
```json
{
  "SmsOp": {
    "phone": "{{user_phone}}",
    "text": "کد شما: {{otp_code}}"
  }
}
```

---

## نکات کلیدی

### ✅ انجام دهید
- نام‌های توصیفی استفاده کنید (`EmailOp`, نه `op1`)
- `required` fields را مشخص کنید
- `description` برای هر property بنویسید
- از `enum` برای محدود کردن مقادیر استفاده کنید

### ❌ انجام ندهید
- نام‌های تکراری نسازید
- JSON Schema نامعتبر ننویسید
- فراموش نکنید که `{{variables}}` در description ذکر شود

---

## Keyboard Shortcuts

| کلید | عملکرد |
|------|--------|
| `Ctrl+Space` | Autocomplete در Monaco |
| `Ctrl+S` | Save route |
| `F2` | Rename variable |
| `Ctrl+/` | Toggle comment |

---

## Troubleshooting

**Q: Schema ثبت نمی‌شود؟**
- JSON Schema معتبر است؟ از [jsonschemavalidator.net](https://www.jsonschemavalidator.net/) استفاده کنید

**Q: Monaco autocomplete نمی‌دهد؟**
- Page را refresh کنید (F5)
- Developer Console باز کنید، error ندارید؟

**Q: Backend error می‌دهد؟**
- Server را restart کنید: `cargo run --release`

---

## دستورات مفید

### Export همه Schemas
```javascript
// در Browser Console بزنید
copy(localStorage.getItem('worpen_custom_schemas'))
```

### Import Schema
```javascript
// JSON را paste کنید
localStorage.setItem('worpen_custom_schemas', '[...]')
location.reload()
```

### Clear All Schemas
```javascript
localStorage.removeItem('worpen_custom_schemas')
location.reload()
```

---

## مراحل بعدی

✅ Schema تعریف کردید → [CUSTOM_OPERATIONS_GUIDE.md](../CUSTOM_OPERATIONS_GUIDE.md) را بخوانید  
✅ Route ساختید → در `backend/data/routes/` فایل compiled را بررسی کنید  
✅ تست کردید → Global Functions با custom operations بسازید  

---

**💡 نکته Pro:** می‌توانید چند custom operation را در یک Global Function ترکیب کنید:

```json
{
  "name": "notify_all_channels",
  "params": ["message"],
  "logic": [
    {"EmailOp": {"to": "admin@example.com", "subject": "Alert", "body": "{{message}}"}},
    {"SmsOp": {"phone": "+1234567890", "text": "{{message}}"}},
    {"NotifyOp": {"message": "{{message}}", "priority": "high"}}
  ]
}
```

---

**زمان خواندن:** 3 دقیقه | **سطح:** مبتدی | **نسخه:** 1.0.0
