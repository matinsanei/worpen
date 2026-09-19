# 🎨 Custom Operations Guide

## مقدمه

**Worpen** از یک معماری **Zero-Touch Extensibility** استفاده می‌کند که به شما اجازه می‌دهد **بدون rebuild یا تغییر کد backend**، operation های جدید تعریف کنید.

این امکان شامل دو لایه است:
1. **Frontend Validation**: Monaco Editor برای autocomplete و type checking
2. **Backend Processing**: Generic Compiler برای variable scoping و execution

---

## 📋 فهرست مطالب

- [نحوه کار سیستم](#نحوه-کار-سیستم)
- [تعریف Custom Operation در UI](#تعریف-custom-operation-در-ui)
- [استفاده در Route Builder](#استفاده-در-route-builder)
- [مثال‌های کاربردی](#مثال‌های-کاربردی)
- [Best Practices](#best-practices)
- [محدودیت‌ها و راه‌حل‌ها](#محدودیت‌ها-و-راه‌حل‌ها)
- [API Reference](#api-reference)

---

## 🔄 نحوه کار سیستم

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  1. USER DEFINES SCHEMA (UI Settings)                      │
│     ↓                                                        │
│     localStorage['worpen_custom_schemas']                   │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  2. MONACO EDITOR (Route Builder)                          │
│     • Loads schemas from localStorage                       │
│     • Provides autocomplete                                 │
│     • Validates JSON structure                              │
│     • Shows inline documentation                            │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  3. BACKEND DESERIALIZATION                                 │
│     • Accepts as CustomOp(HashMap<String, Value>)          │
│     • No errors for unknown operations! ✅                  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  4. GENERIC WALKER (Compiler)                              │
│     • Scans JSON tree recursively                           │
│     • Finds {{variables}} in all strings                    │
│     • Performs scope transformation                         │
│     • {{msg}} → {{_0_msg}}                                  │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│  5. EXECUTION (VM/Interpreter)                             │
│     • CustomOp currently returns debug info                 │
│     • Future: Plugin system or executor registry            │
└─────────────────────────────────────────────────────────────┘
```

### Component Breakdown

| Component | Purpose | Location |
|-----------|---------|----------|
| **Settings UI** | Schema registration | `views/SettingsView.tsx` |
| **localStorage** | Schema persistence | Browser storage |
| **Monaco Editor** | Validation & Autocomplete | `components/RouteBuilder/EditorPanel.tsx` |
| **CustomOp Variant** | Backend acceptance | `backend/crates/proto/src/models/routes.rs` |
| **Generic Walker** | Variable scoping | `backend/crates/core/src/services/dynamic_routes/service.rs` |

---

## ➕ تعریف Custom Operation در UI

### مرحله 1: باز کردن Settings

1. از منوی سمت راست روی **Settings** کلیک کنید
2. به بخش **🧩 Extensions / Custom Operations** بروید

### مرحله 2: وارد کردن اطلاعات

#### Operation Name
نام operation شما (باید unique باشد):
```
NotifyOp
EmailOp
SmsOp
```

#### JSON Schema
ساختار JSON Schema (Draft 7) برای validation:

**مثال ساده:**
```json
{
  "type": "object",
  "required": ["message"],
  "properties": {
    "message": {
      "type": "string",
      "description": "Message to send (supports {{variables}})"
    }
  }
}
```

**مثال پیشرفته:**
```json
{
  "type": "object",
  "required": ["to", "subject", "body"],
  "properties": {
    "to": {
      "type": "string",
      "format": "email",
      "description": "Recipient email address"
    },
    "subject": {
      "type": "string",
      "minLength": 1,
      "maxLength": 100,
      "description": "Email subject line"
    },
    "body": {
      "type": "string",
      "description": "Email body (supports {{variables}} and HTML)"
    },
    "cc": {
      "type": "array",
      "items": {
        "type": "string",
        "format": "email"
      },
      "description": "CC recipients (optional)"
    },
    "priority": {
      "type": "string",
      "enum": ["low", "normal", "high"],
      "default": "normal",
      "description": "Email priority level"
    }
  }
}
```

### مرحله 3: ثبت Schema

1. روی دکمه **Register Custom Schema** کلیک کنید
2. پیغام تأیید نمایش داده می‌شود
3. Schema در لیست **Registered Custom Schemas** ظاهر می‌شود

### مرحله 4: مدیریت Schemas

- **حذف**: روی آیکن 🗑️ کنار schema کلیک کنید
- **ویرایش**: Schema را حذف کرده و دوباره با اطلاعات جدید ثبت کنید

---

## 🛠️ استفاده در Route Builder

### Autocomplete

وقتی در Monaco Editor تایپ می‌کنید:

```json
{
  "logic": [
    {
      "Not"  // ← Ctrl+Space را بزنید
    }
  ]
}
```

Monaco پیشنهاد می‌دهد:
- `NotifyOp`
- (سایر operations که با Not شروع می‌شوند)

### Inline Documentation

روی property ها hover کنید:

```json
{
  "NotifyOp": {
    "message": ""  // ← Hover: "Message to send (supports {{variables}})"
  }
}
```

### Validation

اگر required field را فراموش کنید:

```json
{
  "NotifyOp": {
    "content": "Hello"  // ❌ خط قرمز: "message" is required
  }
}
```

### Variable Support

تمام string fields از `{{variables}}` پشتیبانی می‌کنند:

```json
{
  "logic": [
    {"set": {"var": "username", "value": "{{request.body.name}}"}},
    {
      "NotifyOp": {
        "message": "Welcome, {{username}}!"
      }
    }
  ]
}
```

**نتیجه بعد از Compilation:**
```json
{
  "NotifyOp": {
    "message": "Welcome, {{_0_username}}!"  // ← Scoped!
  }
}
```

---

## 💡 مثال‌های کاربردی

### مثال 1: Notification System

#### تعریف Schema در Settings

```json
{
  "type": "object",
  "required": ["target", "content"],
  "properties": {
    "target": {
      "type": "string",
      "enum": ["admin", "users", "moderators"],
      "description": "Notification target audience"
    },
    "content": {
      "type": "string",
      "description": "Notification message"
    },
    "priority": {
      "type": "string",
      "enum": ["low", "medium", "high"],
      "default": "medium"
    }
  }
}
```

#### استفاده در Route

```json
{
  "name": "alert_admins",
  "path": "/api/admin/alert",
  "method": "POST",
  "logic": [
    {"set": {"var": "message", "value": "{{request.body.message}}"}},
    {"set": {"var": "severity", "value": "{{request.body.severity}}"}},
    {
      "NotifyOp": {
        "target": "admin",
        "content": "[{{severity}}] {{message}}",
        "priority": "high"
      }
    },
    {
      "return": {
        "value": {"status": "notified"}
      }
    }
  ]
}
```

#### تست با PowerShell

```powershell
Invoke-WebRequest -Uri "http://127.0.0.1:3000/api/admin/alert" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"message":"Database connection failed","severity":"CRITICAL"}' `
  -UseBasicParsing
```

---

### مثال 2: Email Service

#### تعریف Schema

```json
{
  "type": "object",
  "required": ["to", "subject", "body"],
  "properties": {
    "to": {
      "type": "string",
      "format": "email"
    },
    "subject": {
      "type": "string"
    },
    "body": {
      "type": "string"
    },
    "template": {
      "type": "string",
      "description": "Email template name (optional)"
    }
  }
}
```

#### Global Function

```json
{
  "name": "send_welcome_email",
  "params": ["user_email", "user_name"],
  "logic": [
    {
      "EmailOp": {
        "to": "{{user_email}}",
        "subject": "Welcome to Worpen!",
        "body": "Hello {{user_name}}, thanks for joining us!",
        "template": "welcome"
      }
    }
  ]
}
```

#### استفاده در Route

```json
{
  "name": "user_registration",
  "path": "/api/users/register",
  "method": "POST",
  "logic": [
    {
      "sql_op": {
        "query": "INSERT INTO users (email, name) VALUES (?, ?)",
        "args": ["{{request.body.email}}", "{{request.body.name}}"],
        "output_var": "insert_result"
      }
    },
    {
      "call_function": {
        "name": "send_welcome_email",
        "args": ["{{request.body.email}}", "{{request.body.name}}"],
        "output_var": "email_result"
      }
    },
    {
      "return": {
        "value": {"status": "registered", "user_id": "{{insert_result.id}}"}
      }
    }
  ]
}
```

---

### مثال 3: SMS Gateway

#### تعریف Schema

```json
{
  "type": "object",
  "required": ["phone", "text"],
  "properties": {
    "phone": {
      "type": "string",
      "pattern": "^\\+?[1-9]\\d{1,14}$",
      "description": "Phone number (E.164 format)"
    },
    "text": {
      "type": "string",
      "maxLength": 160,
      "description": "SMS text (max 160 chars)"
    },
    "sender_id": {
      "type": "string",
      "description": "Sender ID (optional)"
    }
  }
}
```

#### استفاده در Route

```json
{
  "name": "send_otp",
  "path": "/api/auth/send-otp",
  "method": "POST",
  "logic": [
    {
      "math_op": {
        "operation": "random_int",
        "args": [1000, 9999]
      }
    },
    {"set": {"var": "otp_code", "value": "${_last}"}},
    {
      "redis_op": {
        "command": "SET",
        "key": "otp:{{request.body.phone}}",
        "value": "{{otp_code}}",
        "ttl_seconds": 300
      }
    },
    {
      "SmsOp": {
        "phone": "{{request.body.phone}}",
        "text": "Your verification code is: {{otp_code}}",
        "sender_id": "Worpen"
      }
    },
    {
      "return": {
        "value": {"status": "sent"}
      }
    }
  ]
}
```

---

### مثال 4: Webhook Integration

#### تعریف Schema

```json
{
  "type": "object",
  "required": ["url", "payload"],
  "properties": {
    "url": {
      "type": "string",
      "format": "uri",
      "description": "Webhook endpoint URL"
    },
    "method": {
      "type": "string",
      "enum": ["POST", "PUT"],
      "default": "POST"
    },
    "payload": {
      "type": "object",
      "description": "JSON payload to send"
    },
    "headers": {
      "type": "object",
      "description": "Custom headers"
    },
    "retry_count": {
      "type": "integer",
      "minimum": 0,
      "maximum": 5,
      "default": 3
    }
  }
}
```

#### استفاده در Route

```json
{
  "name": "order_webhook",
  "path": "/api/orders/create",
  "method": "POST",
  "logic": [
    {
      "sql_op": {
        "query": "INSERT INTO orders (user_id, amount) VALUES (?, ?)",
        "args": ["{{request.body.user_id}}", "{{request.body.amount}}"],
        "output_var": "order"
      }
    },
    {
      "WebhookOp": {
        "url": "https://api.example.com/webhooks/orders",
        "method": "POST",
        "payload": {
          "event": "order.created",
          "order_id": "{{order.id}}",
          "amount": "{{request.body.amount}}"
        },
        "headers": {
          "X-API-Key": "{{env.WEBHOOK_API_KEY}}"
        },
        "retry_count": 3
      }
    },
    {
      "return": {
        "value": {"order_id": "{{order.id}}"}
      }
    }
  ]
}
```

---

## 📚 Best Practices

### 1. نام‌گذاری

✅ **خوب:**
```
EmailOp
SmsOp
NotifyOp
WebhookOp
```

❌ **بد:**
```
email_operation
Send-Email
op1
customOp
```

**قوانین:**
- از PascalCase استفاده کنید
- با `Op` یا `Operation` پایان دهید
- نام باید توصیفی باشد

### 2. Schema Design

✅ **Required Fields را مشخص کنید:**
```json
{
  "required": ["to", "message"]
}
```

✅ **Default Values تعریف کنید:**
```json
{
  "properties": {
    "priority": {
      "type": "string",
      "default": "normal"
    }
  }
}
```

✅ **Description بنویسید:**
```json
{
  "description": "Email address (supports {{user_email}} variable)"
}
```

✅ **از enum برای محدود کردن مقادیر استفاده کنید:**
```json
{
  "type": "string",
  "enum": ["low", "medium", "high"]
}
```

### 3. Variable Naming

در documentation خود مشخص کنید کدام field ها از variables پشتیبانی می‌کنند:

```json
{
  "properties": {
    "message": {
      "type": "string",
      "description": "Message content (supports {{variables}})"
    }
  }
}
```

### 4. Error Handling

Custom operations باید graceful fail کنند. در logic خود try/catch استفاده کنید:

```json
{
  "logic": [
    {
      "try": {
        "body": [
          {
            "EmailOp": {
              "to": "{{user_email}}",
              "subject": "Test",
              "body": "Hello"
            }
          }
        ],
        "catch": [
          {
            "log": {
              "level": "error",
              "message": "Email send failed: {{_error}}"
            }
          }
        ]
      }
    }
  ]
}
```

### 5. Documentation

برای هر custom operation یک فایل مثال بسازید:

```
backend/data/examples/
  ├── email_op_example.json
  ├── sms_op_example.json
  └── notify_op_example.json
```

---

## ⚠️ محدودیت‌ها و راه‌حل‌ها

### محدودیت 1: Execution

**مشکل:**
```
CustomOp در runtime اجرا نمی‌شود، فقط variable scoping انجام می‌دهد.
```

**راه‌حل فعلی:**
از built-in operations ترکیبی استفاده کنید:

```json
{
  "logic": [
    {
      "log": {"level": "info", "message": "Sending notification: {{msg}}"}
    },
    {
      "ws_op": {
        "command": "broadcast",
        "message": "{{msg}}",
        "channel": "notifications"
      }
    },
    {
      "http_request": {
        "url": "https://api.pushover.net/1/messages.json",
        "method": "POST",
        "body": {"message": "{{msg}}"}
      }
    }
  ]
}
```

**راه‌حل آینده:**
Backend executor registry که custom operations را lookup و اجرا کند.

### محدودیت 2: Cross-Tab Sync

**مشکل:**
```
اگر در یک tab schema ثبت کنید، tab های دیگر باید refresh شوند.
```

**راه‌حل:**
از storage event listener استفاده می‌شود (قبلاً پیاده‌سازی شده):

```typescript
useEffect(() => {
  const handleStorageChange = (e: StorageEvent) => {
    if (e.key === 'worpen_custom_schemas') {
      configureMonaco(); // Reload schemas
    }
  };
  window.addEventListener('storage', handleStorageChange);
  return () => window.removeEventListener('storage', handleStorageChange);
}, []);
```

### محدودیت 3: Schema Validation Depth

**مشکل:**
```
Monaco فقط first-level validation انجام می‌دهد.
```

**راه‌حل:**
Schema های دقیق‌تر با `$ref` و `definitions` بنویسید:

```json
{
  "type": "object",
  "required": ["emails"],
  "properties": {
    "emails": {
      "type": "array",
      "items": {
        "$ref": "#/definitions/EmailConfig"
      }
    }
  },
  "definitions": {
    "EmailConfig": {
      "type": "object",
      "required": ["to", "subject"],
      "properties": {
        "to": {"type": "string", "format": "email"},
        "subject": {"type": "string"}
      }
    }
  }
}
```

---

## 🔌 API Reference

### localStorage Schema Structure

```typescript
interface CustomSchema {
  id: string;           // UUID v4
  operationName: string;
  schema: object;       // JSON Schema Draft 7
  createdAt: string;    // ISO 8601
}

// Storage key: 'worpen_custom_schemas'
// Value: CustomSchema[]
```

### Example localStorage Content

```json
[
  {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "operationName": "NotifyOp",
    "schema": {
      "type": "object",
      "required": ["message"],
      "properties": {
        "message": {"type": "string"}
      }
    },
    "createdAt": "2026-01-03T12:30:00.000Z"
  }
]
```

### Monaco Schema Extension

Custom schemas به `WORPEN_ROUTE_SCHEMA` اضافه می‌شوند:

```typescript
const customOperations = customSchemas.reduce((acc, schema) => {
  acc[schema.operationName] = {
    type: 'object',
    properties: {
      [schema.operationName]: schema.schema
    },
    required: [schema.operationName]
  };
  return acc;
}, {});

monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
  validate: true,
  schemas: [{
    uri: 'http://worpen.local/route-schema.json',
    schema: {
      ...WORPEN_ROUTE_SCHEMA,
      definitions: {
        ...WORPEN_ROUTE_SCHEMA.definitions,
        LogicOperation: {
          oneOf: [
            ...existingOperations,
            ...Object.values(customOperations)
          ]
        }
      }
    }
  }]
});
```

---

## 🚀 Quick Start Checklist

- [ ] باز کردن Settings → Extensions
- [ ] نوشتن JSON Schema برای operation جدید
- [ ] ثبت schema با دکمه Register
- [ ] باز کردن Route Builder
- [ ] تست autocomplete با Ctrl+Space
- [ ] نوشتن route با custom operation
- [ ] استفاده از `{{variables}}` در string fields
- [ ] ذخیره و بررسی compiled route در `backend/data/routes/`
- [ ] بررسی variable scoping: `{{msg}}` → `{{_0_msg}}`

---

## 📞 پشتیبانی و سوالات

**سوال:** چرا custom operation اجرا نمی‌شود؟

**پاسخ:** فعلاً CustomOp فقط variable scoping انجام می‌دهد. برای execution واقعی، از built-in operations ترکیبی استفاده کنید یا منتظر plugin system بمانید.

---

**سوال:** آیا می‌توانم schema را export کنم؟

**پاسخ:** بله، از Developer Tools → Application → Local Storage → `worpen_custom_schemas` کپی کنید.

---

**سوال:** چطوری schema را بین تیم share کنم؟

**پاسخ:** JSON را از localStorage کپی کرده و در یک فایل ذخیره کنید. اعضای تیم می‌توانند آن را import کنند.

---

## 🎓 مثال‌های پیشرفته

### Conditional Custom Operations

```json
{
  "logic": [
    {"set": {"var": "notification_enabled", "value": true}},
    {
      "if": {
        "condition": "{{notification_enabled}} == true",
        "then": [
          {
            "NotifyOp": {
              "target": "admin",
              "message": "Event occurred"
            }
          }
        ]
      }
    }
  ]
}
```

### Loop with Custom Operations

```json
{
  "logic": [
    {"set": {"var": "users", "value": ["user1@example.com", "user2@example.com"]}},
    {
      "loop": {
        "collection": "{{users}}",
        "var": "email",
        "body": [
          {
            "EmailOp": {
              "to": "{{email}}",
              "subject": "Announcement",
              "body": "Hello!"
            }
          }
        ]
      }
    }
  ]
}
```

### Nested Custom Operations in Global Functions

```json
{
  "name": "multi_channel_notify",
  "params": ["message", "urgency"],
  "logic": [
    {
      "parallel": {
        "tasks": [
          [
            {
              "EmailOp": {
                "to": "admin@example.com",
                "subject": "[{{urgency}}] Alert",
                "body": "{{message}}"
              }
            }
          ],
          [
            {
              "SmsOp": {
                "phone": "+1234567890",
                "text": "{{message}}"
              }
            }
          ],
          [
            {
              "NotifyOp": {
                "target": "admin",
                "content": "{{message}}",
                "priority": "{{urgency}}"
              }
            }
          ]
        ]
      }
    }
  ]
}
```

---

## 🔮 آینده: Plugin System

در نسخه‌های بعدی، قابلیت اجرای واقعی custom operations اضافه می‌شود:

```rust
// Future: Plugin Registry
pub trait OperationExecutor {
    fn execute(&self, params: HashMap<String, Value>) -> Result<Value>;
}

// Custom plugins
pub struct EmailOperationExecutor;
impl OperationExecutor for EmailOperationExecutor {
    fn execute(&self, params: HashMap<String, Value>) -> Result<Value> {
        let to = params.get("to").unwrap();
        let subject = params.get("subject").unwrap();
        // ... send email
        Ok(json!({"sent": true}))
    }
}
```

---

**نسخه:** 1.0.0  
**تاریخ:** 2026-01-03  
**نویسنده:** Worpen Development Team
