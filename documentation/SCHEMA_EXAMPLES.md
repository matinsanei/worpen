# 📦 Custom Operation Schema Examples

این فایل شامل schema های آماده برای use case های مختلف است که می‌توانید مستقیماً در Settings → Extensions استفاده کنید.

---

## 🔔 Notification Systems

### NotifyOp - Basic Notification
```json
{
  "operationName": "NotifyOp",
  "schema": {
    "type": "object",
    "required": ["message"],
    "properties": {
      "message": {
        "type": "string",
        "description": "Notification message (supports {{variables}})"
      },
      "target": {
        "type": "string",
        "enum": ["admin", "users", "moderators", "all"],
        "default": "all",
        "description": "Target audience"
      },
      "priority": {
        "type": "string",
        "enum": ["low", "medium", "high", "urgent"],
        "default": "medium",
        "description": "Notification priority level"
      },
      "channel": {
        "type": "string",
        "description": "Notification channel (optional)"
      }
    }
  }
}
```

**Usage Example:**
```json
{
  "NotifyOp": {
    "message": "New order received: {{order_id}}",
    "target": "admin",
    "priority": "high"
  }
}
```

---

### PushNotificationOp - Mobile Push Notifications
```json
{
  "operationName": "PushNotificationOp",
  "schema": {
    "type": "object",
    "required": ["title", "body"],
    "properties": {
      "title": {
        "type": "string",
        "maxLength": 50,
        "description": "Notification title"
      },
      "body": {
        "type": "string",
        "maxLength": 200,
        "description": "Notification body text"
      },
      "user_ids": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Target user IDs"
      },
      "badge": {
        "type": "integer",
        "minimum": 0,
        "description": "Badge count (iOS)"
      },
      "sound": {
        "type": "string",
        "default": "default",
        "description": "Notification sound"
      },
      "data": {
        "type": "object",
        "description": "Custom data payload"
      }
    }
  }
}
```

---

## 📧 Email Operations

### EmailOp - Simple Email
```json
{
  "operationName": "EmailOp",
  "schema": {
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
        "description": "Email body (HTML or plain text)"
      },
      "cc": {
        "type": "array",
        "items": {"type": "string", "format": "email"},
        "description": "CC recipients"
      },
      "bcc": {
        "type": "array",
        "items": {"type": "string", "format": "email"},
        "description": "BCC recipients"
      },
      "reply_to": {
        "type": "string",
        "format": "email",
        "description": "Reply-to address"
      }
    }
  }
}
```

**Usage Example:**
```json
{
  "EmailOp": {
    "to": "{{user_email}}",
    "subject": "Welcome to Worpen",
    "body": "<h1>Hello {{user_name}}</h1><p>Thanks for joining!</p>",
    "reply_to": "support@worpen.com"
  }
}
```

---

### BulkEmailOp - Batch Email Sending
```json
{
  "operationName": "BulkEmailOp",
  "schema": {
    "type": "object",
    "required": ["recipients", "subject", "body"],
    "properties": {
      "recipients": {
        "type": "array",
        "items": {"type": "string", "format": "email"},
        "minItems": 1,
        "maxItems": 1000,
        "description": "List of recipient emails"
      },
      "subject": {
        "type": "string",
        "description": "Email subject"
      },
      "body": {
        "type": "string",
        "description": "Email body template"
      },
      "template_id": {
        "type": "string",
        "description": "Pre-defined template ID"
      },
      "batch_size": {
        "type": "integer",
        "minimum": 1,
        "maximum": 100,
        "default": 50,
        "description": "Emails per batch"
      }
    }
  }
}
```

---

## 📱 SMS Operations

### SmsOp - Basic SMS
```json
{
  "operationName": "SmsOp",
  "schema": {
    "type": "object",
    "required": ["phone", "text"],
    "properties": {
      "phone": {
        "type": "string",
        "pattern": "^\\+?[1-9]\\d{1,14}$",
        "description": "Phone number (E.164 format: +1234567890)"
      },
      "text": {
        "type": "string",
        "maxLength": 160,
        "description": "SMS text content (max 160 characters)"
      },
      "sender_id": {
        "type": "string",
        "maxLength": 11,
        "description": "Sender ID (alphanumeric)"
      },
      "delivery_report": {
        "type": "boolean",
        "default": false,
        "description": "Request delivery report"
      }
    }
  }
}
```

**Usage Example:**
```json
{
  "SmsOp": {
    "phone": "{{user_phone}}",
    "text": "Your OTP code is: {{otp_code}}",
    "sender_id": "Worpen",
    "delivery_report": true
  }
}
```

---

### WhatsAppOp - WhatsApp Messages
```json
{
  "operationName": "WhatsAppOp",
  "schema": {
    "type": "object",
    "required": ["phone", "message"],
    "properties": {
      "phone": {
        "type": "string",
        "pattern": "^\\+?[1-9]\\d{1,14}$",
        "description": "WhatsApp phone number"
      },
      "message": {
        "type": "string",
        "description": "Message text"
      },
      "media_url": {
        "type": "string",
        "format": "uri",
        "description": "Image/video URL"
      },
      "template_name": {
        "type": "string",
        "description": "WhatsApp Business template name"
      },
      "template_params": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Template parameters"
      }
    }
  }
}
```

---

## 🔗 Webhook & Integration

### WebhookOp - HTTP Webhook
```json
{
  "operationName": "WebhookOp",
  "schema": {
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
        "enum": ["POST", "PUT", "PATCH"],
        "default": "POST",
        "description": "HTTP method"
      },
      "payload": {
        "type": "object",
        "description": "JSON payload to send"
      },
      "headers": {
        "type": "object",
        "additionalProperties": {"type": "string"},
        "description": "Custom HTTP headers"
      },
      "timeout_ms": {
        "type": "integer",
        "minimum": 1000,
        "maximum": 30000,
        "default": 5000,
        "description": "Request timeout in milliseconds"
      },
      "retry_count": {
        "type": "integer",
        "minimum": 0,
        "maximum": 5,
        "default": 0,
        "description": "Number of retry attempts"
      },
      "retry_delay_ms": {
        "type": "integer",
        "minimum": 100,
        "default": 1000,
        "description": "Delay between retries (ms)"
      }
    }
  }
}
```

**Usage Example:**
```json
{
  "WebhookOp": {
    "url": "https://hooks.slack.com/services/XXX/YYY/ZZZ",
    "method": "POST",
    "payload": {
      "text": "Order {{order_id}} completed",
      "channel": "#orders"
    },
    "headers": {
      "Content-Type": "application/json"
    },
    "retry_count": 3
  }
}
```

---

### SlackOp - Slack Notifications
```json
{
  "operationName": "SlackOp",
  "schema": {
    "type": "object",
    "required": ["channel", "text"],
    "properties": {
      "channel": {
        "type": "string",
        "pattern": "^#[a-z0-9-_]+$",
        "description": "Slack channel (e.g., #general)"
      },
      "text": {
        "type": "string",
        "description": "Message text"
      },
      "username": {
        "type": "string",
        "description": "Bot username"
      },
      "icon_emoji": {
        "type": "string",
        "pattern": "^:[a-z0-9_+-]+:$",
        "description": "Bot icon emoji (e.g., :robot_face:)"
      },
      "attachments": {
        "type": "array",
        "description": "Message attachments"
      }
    }
  }
}
```

---

## 💳 Payment Operations

### PaymentOp - Payment Processing
```json
{
  "operationName": "PaymentOp",
  "schema": {
    "type": "object",
    "required": ["amount", "currency", "customer_id"],
    "properties": {
      "amount": {
        "type": "number",
        "minimum": 0,
        "description": "Payment amount"
      },
      "currency": {
        "type": "string",
        "enum": ["USD", "EUR", "GBP", "IRR"],
        "description": "Currency code"
      },
      "customer_id": {
        "type": "string",
        "description": "Customer identifier"
      },
      "payment_method": {
        "type": "string",
        "enum": ["card", "bank_transfer", "wallet"],
        "description": "Payment method"
      },
      "description": {
        "type": "string",
        "maxLength": 200,
        "description": "Payment description"
      },
      "metadata": {
        "type": "object",
        "description": "Additional metadata"
      }
    }
  }
}
```

---

### RefundOp - Refund Processing
```json
{
  "operationName": "RefundOp",
  "schema": {
    "type": "object",
    "required": ["transaction_id", "amount"],
    "properties": {
      "transaction_id": {
        "type": "string",
        "description": "Original transaction ID"
      },
      "amount": {
        "type": "number",
        "minimum": 0,
        "description": "Refund amount"
      },
      "reason": {
        "type": "string",
        "enum": ["duplicate", "fraudulent", "requested_by_customer", "other"],
        "description": "Refund reason"
      },
      "notes": {
        "type": "string",
        "description": "Internal notes"
      }
    }
  }
}
```

---

## 📊 Analytics & Tracking

### TrackEventOp - Event Tracking
```json
{
  "operationName": "TrackEventOp",
  "schema": {
    "type": "object",
    "required": ["event_name"],
    "properties": {
      "event_name": {
        "type": "string",
        "description": "Event identifier"
      },
      "user_id": {
        "type": "string",
        "description": "User ID"
      },
      "properties": {
        "type": "object",
        "description": "Event properties"
      },
      "timestamp": {
        "type": "string",
        "format": "date-time",
        "description": "Event timestamp (ISO 8601)"
      }
    }
  }
}
```

**Usage Example:**
```json
{
  "TrackEventOp": {
    "event_name": "order_completed",
    "user_id": "{{user_id}}",
    "properties": {
      "order_id": "{{order_id}}",
      "amount": "{{total_amount}}",
      "items_count": "{{items_count}}"
    }
  }
}
```

---

### MetricsOp - Custom Metrics
```json
{
  "operationName": "MetricsOp",
  "schema": {
    "type": "object",
    "required": ["metric_name", "value"],
    "properties": {
      "metric_name": {
        "type": "string",
        "description": "Metric identifier"
      },
      "value": {
        "type": "number",
        "description": "Metric value"
      },
      "unit": {
        "type": "string",
        "enum": ["count", "ms", "bytes", "percent"],
        "description": "Value unit"
      },
      "tags": {
        "type": "object",
        "additionalProperties": {"type": "string"},
        "description": "Metric tags/labels"
      }
    }
  }
}
```

---

## 🗄️ Data Operations

### CacheOp - Advanced Caching
```json
{
  "operationName": "CacheOp",
  "schema": {
    "type": "object",
    "required": ["operation", "key"],
    "properties": {
      "operation": {
        "type": "string",
        "enum": ["get", "set", "delete", "exists", "invalidate_pattern"],
        "description": "Cache operation type"
      },
      "key": {
        "type": "string",
        "description": "Cache key (supports {{variables}})"
      },
      "value": {
        "description": "Value to cache (for 'set' operation)"
      },
      "ttl_seconds": {
        "type": "integer",
        "minimum": 1,
        "description": "Time to live in seconds"
      },
      "namespace": {
        "type": "string",
        "description": "Cache namespace"
      },
      "output_var": {
        "type": "string",
        "description": "Variable to store result"
      }
    }
  }
}
```

---

### QueueOp - Message Queue
```json
{
  "operationName": "QueueOp",
  "schema": {
    "type": "object",
    "required": ["operation", "queue_name"],
    "properties": {
      "operation": {
        "type": "string",
        "enum": ["enqueue", "dequeue", "peek", "size", "clear"],
        "description": "Queue operation"
      },
      "queue_name": {
        "type": "string",
        "description": "Queue identifier"
      },
      "message": {
        "description": "Message to enqueue"
      },
      "priority": {
        "type": "integer",
        "minimum": 0,
        "maximum": 10,
        "default": 5,
        "description": "Message priority (0=lowest, 10=highest)"
      },
      "delay_seconds": {
        "type": "integer",
        "minimum": 0,
        "description": "Delay before processing"
      }
    }
  }
}
```

---

## 🔐 Security Operations

### EncryptOp - Data Encryption
```json
{
  "operationName": "EncryptOp",
  "schema": {
    "type": "object",
    "required": ["operation", "data"],
    "properties": {
      "operation": {
        "type": "string",
        "enum": ["encrypt", "decrypt", "hash"],
        "description": "Encryption operation"
      },
      "data": {
        "type": "string",
        "description": "Data to encrypt/decrypt/hash"
      },
      "algorithm": {
        "type": "string",
        "enum": ["AES-256", "RSA", "SHA-256", "bcrypt"],
        "default": "AES-256",
        "description": "Encryption algorithm"
      },
      "key": {
        "type": "string",
        "description": "Encryption key (use {{env.SECRET_KEY}})"
      },
      "output_var": {
        "type": "string",
        "description": "Variable to store result"
      }
    }
  }
}
```

---

### AuditLogOp - Security Audit Logging
```json
{
  "operationName": "AuditLogOp",
  "schema": {
    "type": "object",
    "required": ["action", "resource"],
    "properties": {
      "action": {
        "type": "string",
        "enum": ["create", "read", "update", "delete", "access", "denied"],
        "description": "Action performed"
      },
      "resource": {
        "type": "string",
        "description": "Resource identifier"
      },
      "user_id": {
        "type": "string",
        "description": "User who performed action"
      },
      "ip_address": {
        "type": "string",
        "description": "Source IP address"
      },
      "metadata": {
        "type": "object",
        "description": "Additional audit data"
      },
      "severity": {
        "type": "string",
        "enum": ["info", "warning", "critical"],
        "default": "info"
      }
    }
  }
}
```

---

## 🤖 AI & ML Operations

### AiPromptOp - AI Model Interaction
```json
{
  "operationName": "AiPromptOp",
  "schema": {
    "type": "object",
    "required": ["prompt"],
    "properties": {
      "prompt": {
        "type": "string",
        "description": "Prompt for AI model"
      },
      "model": {
        "type": "string",
        "enum": ["gpt-4", "gpt-3.5-turbo", "claude-3", "gemini-pro"],
        "default": "gpt-3.5-turbo",
        "description": "AI model to use"
      },
      "temperature": {
        "type": "number",
        "minimum": 0,
        "maximum": 2,
        "default": 0.7,
        "description": "Response randomness (0=deterministic, 2=creative)"
      },
      "max_tokens": {
        "type": "integer",
        "minimum": 1,
        "maximum": 4000,
        "default": 500,
        "description": "Maximum response length"
      },
      "output_var": {
        "type": "string",
        "description": "Variable to store AI response"
      }
    }
  }
}
```

---

## 📝 نکات استفاده

### Import کردن Schema

1. کل JSON object را کپی کنید
2. در Settings → Extensions فقط بخش **schema** را در فیلد "JSON Schema" paste کنید
3. مقدار **operationName** را در فیلد "Operation Name" وارد کنید

### مثال Import:

از این JSON:
```json
{
  "operationName": "EmailOp",
  "schema": { ... }
}
```

در UI:
- Operation Name: `EmailOp`
- JSON Schema: بخش `schema` را paste کنید

---

## 🔄 Schema Updates

اگر می‌خواهید schema را به‌روزرسانی کنید:

1. در Settings → Extensions، schema قدیمی را **حذف** کنید (🗑️)
2. Schema جدید را با همان `operationName` **ثبت** کنید
3. Page را **refresh** کنید (F5)

---

**تعداد Schema ها:** 20+  
**دسته‌بندی:** Notifications, Email, SMS, Webhooks, Payments, Analytics, Data, Security, AI  
**نسخه:** 1.0.0
