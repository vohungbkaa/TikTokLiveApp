# 03 - Milestone và thứ tự triển khai

## Milestone 0 - Chuẩn bị nền

1. Project Foundation.
2. Local Storage.
3. Session Module.
4. Product & Inventory cơ bản.

Kết quả: app mở được, tạo phiên live, nhập sản phẩm, lưu SQLite.

## Milestone 1 - Manual-first MVP

1. Manual Capture Module.
2. Parser Module bản đầu.
3. Selling Mode Engine:
   - Stock Mode.
   - Unique Mode.
4. Order Module.
5. UI Live Console bản đầu.
6. Export CSV.

Kết quả: chưa cần đọc TikTok tự động, app vẫn có thể lên đơn từ nhập tay. Đây là cách validate workflow nhanh nhất với shop thật.

## Milestone 2 - TikTok WebSocket MVP

1. Node TikTok Connector Sidecar.
2. Event Ingestion Module.
3. Event Normalizer.
4. Connector Supervisor cơ bản.
5. UI connector status.
6. Health logs local.

Kết quả: app đọc comment TikTok realtime, comment tự biến thành order draft.

## Milestone 3 - Ổn định vận hành

1. Retry primary connector tối đa 3 lần, mỗi lần cách 5 giây, rồi failover.
2. Dedupe nâng cao.
3. Review Queue.
4. Parser config theo shop.
5. Virtual list comment feed.
6. In đơn cơ bản.
7. Diagnostic export.

Kết quả: dùng được trong live thật quy mô nhỏ/vừa.

## Milestone 4 - Fallback và update

1. Scrapling Fallback Sidecar.
2. Smart failover rule.
3. Remote Config.
4. Kill switch.
5. Tauri updater.
6. App/sidecar versioning.

Kết quả: khi TikTok connector lỗi, app có đường cứu live rõ ràng.

## Milestone 5 - Mở rộng vận hành shop

1. Preorder Mode.
2. Customer Module nâng cao.
3. Blacklist nội bộ.
4. Template tin nhắn cần copy.
5. Đồng bộ cloud một chiều từ local lên server cho báo cáo phiên live.
6. Tích hợp vận chuyển nếu đã có nhu cầu thật.
