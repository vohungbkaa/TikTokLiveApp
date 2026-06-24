# Module 02 - Local Storage

## Trách nhiệm

Quản lý SQLite, migration, repository layer và transaction cho các thao tác quan trọng.

## Task chi tiết

- Dùng `sqlx` làm thư viện Rust SQLite.
- Bật các feature `sqlite`, `runtime-tokio`, `macros`, `chrono`, `json`.
- Dùng `sqlx::migrate!` làm migration system.
- Tạo thư mục migration tại `src-tauri/migrations`.
- Tạo bảng:
  - `live_sessions`
  - `live_events`
  - `products`
  - `product_variants`
  - `customers`
  - `order_claims`
  - `orders`
  - `order_items`
  - `settings`
  - `connector_health_logs`
- Tạo repository:
  - `SessionRepository`
  - `EventRepository`
  - `ProductRepository`
  - `CustomerRepository`
  - `OrderRepository`
  - `SettingsRepository`
- Tạo transaction helper cho pipeline:
  - lưu event
  - parse
  - áp rule
  - tạo claim/order draft
- Tạo index quan trọng:
  - `live_events(session_id, sequence_no)`
  - `live_events(platform_event_id)`
  - `products(sku)`
  - `product_variants(sku)`
  - `order_claims(session_id, sku)`
  - `orders(session_id, customer_id)`
- Xử lý backup database local.
- Xử lý database corruption ở mức cơ bản:
  - phát hiện lỗi mở DB
  - tạo backup
  - thông báo user
- Thiết lập connection pool SQLite với giới hạn 5 connection.
- Bật WAL mode cho SQLite.
- Bật foreign key bằng `PRAGMA foreign_keys = ON`.

## Task coding chi tiết

### M02-T01 - Cài sqlx và cấu hình database path

- Mục tiêu: app biết DB nằm ở đâu trên máy user.
- File/thư mục dự kiến: `src-tauri/Cargo.toml`, `src-tauri/src/db/mod.rs`.
- Dependency: M01-T04.
- Output: `DbState` chứa `SqlitePool`.
- Done khi: app mở được SQLite file trong app data directory.

### M02-T02 - Tạo migration system

- Mục tiêu: quản lý schema bằng migration.
- File/thư mục dự kiến: `src-tauri/migrations/`, `src-tauri/src/db/migrations.rs`.
- Dependency: M02-T01.
- Output: `sqlx::migrate!` chạy lúc app startup.
- Done khi: app tự tạo DB mới với schema rỗng ban đầu.

### M02-T03 - Tạo migration bảng session/event

- Mục tiêu: lưu phiên live và raw event.
- File/thư mục dự kiến: `src-tauri/migrations/*_create_sessions_events.sql`.
- Dependency: M02-T02.
- Output: bảng `live_sessions`, `live_events`, index liên quan.
- Done khi: migration chạy pass và foreign key đúng.

### M02-T04 - Tạo migration bảng product/inventory

- Mục tiêu: lưu sản phẩm, variant và tồn.
- File/thư mục dự kiến: `src-tauri/migrations/*_create_products.sql`.
- Dependency: M02-T02.
- Output: bảng `products`, `product_variants`, `session_products`.
- Done khi: SKU có unique index phù hợp.

### M02-T05 - Tạo migration bảng customer/order

- Mục tiêu: lưu khách, claim, order draft.
- File/thư mục dự kiến: `src-tauri/migrations/*_create_orders.sql`.
- Dependency: M02-T02.
- Output: bảng `customers`, `order_claims`, `orders`, `order_items`.
- Done khi: tạo order item có FK tới order/product.

### M02-T06 - Tạo migration bảng settings/health logs

- Mục tiêu: lưu cấu hình và log connector.
- File/thư mục dự kiến: `src-tauri/migrations/*_create_settings_health.sql`.
- Dependency: M02-T02.
- Output: bảng `settings`, `connector_health_logs`.
- Done khi: ghi được một health log mẫu.

### M02-T07 - Tạo repository base

- Mục tiêu: chuẩn hóa query DB.
- File/thư mục dự kiến: `src-tauri/src/db/repositories/`.
- Dependency: M02-T03, M02-T04, M02-T05.
- Output: repository modules compile được.
- Done khi: có trait/helper transaction dùng chung.

### M02-T08 - Tạo backup DB local

- Mục tiêu: giảm rủi ro mất dữ liệu.
- File/thư mục dự kiến: `src-tauri/src/db/backup.rs`.
- Dependency: M02-T01.
- Output: command tạo bản copy DB vào thư mục backup.
- Done khi: backup file tồn tại và mở được.

### M02-T09 - Bật WAL và foreign key

- Mục tiêu: tăng độ ổn định SQLite.
- File/thư mục dự kiến: `src-tauri/src/db/mod.rs`.
- Dependency: M02-T01.
- Output: chạy PRAGMA khi tạo connection.
- Done khi: query kiểm tra trả `journal_mode = wal` và `foreign_keys = 1`.

## Done khi

- App tạo/migrate SQLite được.
- Tạo được session, product, event và order draft.
- Pipeline thao tác DB nằm trong transaction.
- SQLite chạy WAL mode và foreign key được bật.
