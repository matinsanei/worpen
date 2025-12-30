# Worpen Dynamic Routes Engine

## مستندات کامل سیستم مسیریابی پویا

این سیستم یک موتور قدرتمند برای ایجاد API endpoints پویا با قابلیت‌های برنامه‌نویسی کامل است که به شما اجازه می‌دهد بدون نیاز به کامپایل مجدد، منطق پیچیده را تعریف و اجرا کنید.

## فهرست مطالب

1. [معرفی و نصب](01-introduction.md)
2. [ساختار پایه](02-basic-structure.md)
3. [متغیرها و عملیات پایه](03-variables-basics.md)
4. [ساختارهای کنترلی](04-control-flow.md)
5. [حلقه‌ها](05-loops.md)
6. **[آموزش توابع - گام به گام](functions-tutorial.md)** ⭐ **توصیه می‌شود**
7. [توابع سفارشی](06-functions.md)
8. [عملیات ریاضی](07-math-operations.md)
9. [عملیات رشته‌ای](08-string-operations.md)
10. [عملیات تاریخ و زمان](09-date-operations.md)
11. [مدیریت خطا (Try/Catch)](10-error-handling.md)
12. [اجرای موازی](11-parallel-execution.md)
13. [نمونه‌های کامل](12-complete-examples.md)

## شروع سریع

### ایجاد اولین Route

```json
{
  "name": "سلام دنیا",
  "path": "/api/hello",
  "method": "POST",
  "description": "اولین API پویا",
  "logic": [
    {
      "set": {
        "var": "message",
        "value": "سلام دنیا!"
      }
    },
    {
      "return": {
        "value": {
          "message": "{{message}}"
        }
      }
    }
  ]
}
```

### ثبت Route

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes \
  -H "Content-Type: application/json" \
  -d @my-route.json
```

### اجرای Route

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes/{route_id}/execute \
  -H "Content-Type: application/json" \
  -d '{"name": "علی"}'
```

## ویژگی‌های کلیدی

✅ **توابع سفارشی** - تعریف و فراخوانی توابع با پارامترها  
✅ **ساختارهای کنترلی** - If/Else, Switch/Case  
✅ **حلقه‌ها** - While, ForEach با Break/Continue  
✅ **مدیریت خطا** - Try/Catch/Finally  
✅ **عملیات ریاضی** - جمع، ضرب، توان، ریشه و...  
✅ **عملیات رشته‌ای** - Upper, Lower, Split, Replace  
✅ **عملیات تاریخ** - Now, Timestamp, Format  
✅ **اجرای موازی** - اجرای همزمان چندین Task  
✅ **متغیرهای پویا** - Resolution و Nested Path Access  

## معماری

```
┌─────────────────┐
│  Client Request │
└────────┬────────┘
         │
    ┌────▼─────┐
    │   API    │
    │  Handler │
    └────┬─────┘
         │
    ┌────▼──────────┐
    │ Route Engine  │
    │ - Parse Logic │
    │ - Execute Ops │
    └────┬──────────┘
         │
    ┌────▼────────┐
    │  Response   │
    └─────────────┘
```

## پیش‌نیازها

- Rust 1.75+
- Cargo
- SQLite 3

## نصب و راه‌اندازی

```bash
# کلون پروژه
git clone https://github.com/matinsanei/worpen.git
cd worpen/backend

# بیلد پروژه
cargo build --release

# اجرای سرور
cargo run -p api
```

سرور روی `http://127.0.0.1:3000` اجرا می‌شود.

## لینک‌های مفید

- [API Reference](api-reference.md)
- [Troubleshooting](troubleshooting.md)
- [Performance Tips](performance.md)
- [Security Best Practices](security.md)

## مشارکت

برای مشارکت در پروژه، لطفاً [راهنمای مشارکت](../CONTRIBUTING.md) را مطالعه کنید.

## لایسنس

این پروژه تحت لایسنس MIT منتشر شده است. برای اطلاعات بیشتر [LICENSE](../LICENSE) را مشاهده کنید.
