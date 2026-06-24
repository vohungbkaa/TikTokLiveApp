# Tình trạng triển khai các Module

Bảng theo dõi trạng thái thi công mã nguồn dựa trên tài liệu thiết kế.

## ✅ Đã thi công & hoàn thành cơ bản (Backend)
- **Module 01 - Project Foundation**: Setup kiến trúc Tauri, cấu hình core.
- **Module 02 - Local Storage**: Khởi tạo SQLite, cấu trúc thư mục database.
- **Module 03 - Session**: Quản lý phiên live.
- **Module 04 - Product & Inventory**: Quản lý danh mục sản phẩm và tồn kho.
- **Module 05 - Event Ingestion**: Pipeline tiếp nhận event từ các connector.
- **Module 10 - Parser**: Phân tích cú pháp comment thành ý định mua hàng.
- **Module 11 - Selling Mode Engine**: Rule bán hàng (Stock, Unique, Preorder).
- **Module 12 - Order**: Tạo đơn nháp (draft order) và quản lý trạng thái.
- **Module 13 - Customer**: Identity resolver, Tag, Risk alert và API khách hàng.

## 🚧 Đang thi công / Chưa hoàn thành trọn vẹn
- **Module 08 - Connector Supervisor**: Mới định hình thư mục `src-tauri/src/connectors`, chưa có bộ quản lý tiến trình (supervisor logic).
- **Module 16 - Telemetry & Diagnostics**: Mới thiết lập tracing/logging khởi tạo app ở `src-tauri/src/diagnostics`, thiếu tính năng export local logs.

## ❌ Chưa thi công
- **Module 06 - Node TikTok Connector Sidecar**
- **Module 07 - Scrapling Fallback Sidecar**
- **Module 09 - Manual Capture** (Cần thiết cho Milestone 1)
- **Module 14 - Print & Export** (Cần thiết cho Milestone 1)
- **Module 15 - UI Layer** (Cần thiết cho Milestone 1)
- **Module 17 - Thin Cloud Services**

## 💡 Đề xuất hành động (Milestone 1 - Manual-first MVP)
Để khép kín luồng sử dụng cơ bản nhất (chốt đơn thủ công không cần kết nối TikTok), các module tiếp theo cần được tập trung thi công:
1. **Module 09 - Manual Capture**
2. **Module 14 - Print & Export**
3. **Module 15 - UI Layer**
