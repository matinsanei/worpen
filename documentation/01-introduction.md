# 1. معرفی و نصب

## Worpen Dynamic Routes Engine چیست؟

Worpen یک موتور مسیریابی پویا است که به شما اجازه می‌دهد API endpoints را بدون نیاز به کامپایل مجدد کد، به صورت پویا ایجاد و مدیریت کنید. این سیستم یک زبان JSON-based برای تعریف منطق برنامه‌نویسی ارائه می‌دهد.

## چرا از Worpen استفاده کنیم؟

### ✅ مزایا

1. **توسعه سریع**: بدون نیاز به rebuild و deploy
2. **انعطاف‌پذیری بالا**: تغییر منطق در runtime
3. **قدرت برنامه‌نویسی کامل**: توابع، حلقه‌ها، شرط‌ها
4. **مدیریت خطای پیشرفته**: Try/Catch/Finally
5. **اجرای موازی**: Parallel execution برای عملیات سنگین
6. **Type-Safe**: با Rust نوشته شده برای امنیت و سرعت

### ⚠️ محدودیت‌ها

- نیاز به SQLite برای ذخیره‌سازی routes
- JSON syntax می‌تواند برای منطق پیچیده verbose باشد
- فقط برای Rust/Axum

## پیش‌نیازها

### نرم‌افزارهای مورد نیاز

```bash
# Rust (نسخه 1.75 یا بالاتر)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# SQLite 3
# Windows: دانلود از sqlite.org
# Linux: sudo apt install sqlite3
# macOS: brew install sqlite3

# Git
# برای دریافت کد منبع
```

### بررسی نسخه‌ها

```bash
rustc --version
# rustc 1.75.0 (یا بالاتر)

cargo --version
# cargo 1.75.0 (یا بالاتر)

sqlite3 --version
# 3.x.x
```

## نصب

### روش 1: از طریق Git

```bash
# کلون مخزن
git clone https://github.com/matinsanei/worpen.git

# ورود به پوشه backend
cd worpen/backend

# دانلود dependencies
cargo build

# اجرای تست‌ها (اختیاری)
cargo test

# بیلد نسخه release
cargo build --release
```

### روش 2: دانلود Binary

```bash
# دانلود از GitHub Releases
# و استخراج فایل api.exe یا api (Linux/macOS)
```

## پیکربندی

### فایل پیکربندی (اختیاری)

فایل `config.toml` در پوشه `backend`:

```toml
[server]
host = "127.0.0.1"
port = 3000

[database]
path = "worpen.db"

[logging]
level = "info"

[runtime]
worker_threads = 4
max_blocking_threads = 512
```

### متغیرهای محیطی

```bash
# پورت سرور
export SERVER_PORT=3000

# مسیر دیتابیس
export DATABASE_URL="worpen.db"

# سطح لاگ
export RUST_LOG=info
```

## اجرای سرور

### حالت Development

```bash
cd worpen/backend
cargo run -p api
```

خروجی:
```
2025-12-18T10:00:00  INFO api: listening on 127.0.0.1:3000
```

### حالت Production

```bash
# بیلد optimized
cargo build --release

# اجرا
./target/release/api
```

### با Docker (در آینده)

```bash
docker build -t worpen .
docker run -p 3000:3000 worpen
```

## بررسی نصب

### تست API

```bash
# بررسی health endpoint
curl http://localhost:3000/health

# خروجی موردانتظار:
# {"status":"ok"}
```

### ثبت اولین Route

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Route",
    "path": "/test",
    "method": "GET",
    "description": "اولین route تستی",
    "logic": [
      {
        "return": {
          "value": {"message": "Hello World"}
        }
      }
    ]
  }'
```

خروجی:
```json
{
  "route_id": "abc123...",
  "message": "Route created successfully"
}
```

### اجرای Route

```bash
curl -X POST http://localhost:3000/api/v1/dynamic-routes/{route_id}/execute \
  -H "Content-Type: application/json" \
  -d '{}'
```

خروجی:
```json
{
  "success": true,
  "result": {
    "message": "Hello World"
  },
  "error": null,
  "execution_time_ms": 2,
  "steps_executed": [
    "Route found: Test Route (/test)",
    "Execution context created",
    "Step 1: Executing operation",
    "Returning value: {\"message\":\"Hello World\"}"
  ]
}
```

## ساختار پروژه

```
worpen/
├── backend/
│   ├── crates/
│   │   ├── api/          # HTTP handlers
│   │   ├── core/         # Business logic
│   │   ├── infra/        # Database, adapters
│   │   └── proto/        # Models, DTOs
│   ├── migrations/       # SQL migrations
│   ├── Cargo.toml
│   └── worpen.db        # SQLite database
├── documentation/        # این مستندات
├── LICENSE
└── README.md
```

## عیب‌یابی نصب

### خطای "command not found: cargo"

```bash
# اضافه کردن cargo به PATH
source $HOME/.cargo/env

# یا در Windows، restart terminal
```

### خطای "database locked"

```bash
# بستن تمام process‌های api
killall api  # Linux/macOS
Stop-Process -Name api  # Windows PowerShell

# حذف lock file
rm worpen.db-shm worpen.db-wal
```

### خطای "Address already in use"

```bash
# تغییر پورت
export SERVER_PORT=3001
cargo run -p api
```

### خطای Compilation

```bash
# پاک کردن cache و rebuild
cargo clean
cargo build
```

## مراحل بعدی

حالا که نصب کامل شد، می‌توانید به بخش‌های بعدی بروید:

- [ساختار پایه](02-basic-structure.md) - ساختار یک route
- [متغیرها و عملیات](03-variables-basics.md) - کار با متغیرها
- [نمونه‌های کامل](12-complete-examples.md) - مثال‌های آماده

## پشتیبانی

اگر در نصب مشکلی داشتید:

1. [Issues در GitHub](https://github.com/matinsanei/worpen/issues)
2. [Discussions](https://github.com/matinsanei/worpen/discussions)
3. ایمیل: support@worpen.dev (اگر موجود باشد)
