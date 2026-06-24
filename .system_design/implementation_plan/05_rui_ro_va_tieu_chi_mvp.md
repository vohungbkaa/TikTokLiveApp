# 05 - Rủi ro và tiêu chí MVP

## Rủi ro kỹ thuật

| Rủi ro | Mức độ | Cách giảm rủi ro |
| --- | --- | --- |
| TikTok đổi giao thức WebSocket | Cao | Connector Supervisor, fallback, remote config, updater, Manual Mode |
| Parser hiểu sai comment | Cao | Confidence score, Review Queue, rule test bằng comment thật |
| Trừ tồn sai/race condition | Cao | Transaction SQLite, test Unique/Stock Mode kỹ |
| UI giật khi comment nhiều | Trung bình | Virtual list, batch update, không render toàn bộ feed |
| Sidecar khó đóng gói | Trung bình | Tách sidecar rõ, test build sớm, version sidecar |
| Seller không biết dùng | Cao | Manual-first UX, ít màn hình, phím tắt, setup ngắn |
| Auto-DM gây rủi ro tài khoản | Cao | Không đưa vào MVP |

## Tiêu chí MVP đạt yêu cầu

- Seller tạo được phiên live trong dưới 2 phút.
- Nhập/import được sản phẩm cơ bản.
- App đọc được comment TikTok bằng WebSocket connector.
- Comment hợp lệ tạo order draft tự động.
- Stock Mode không trừ tồn âm.
- Unique Mode không có 2 winner cho cùng sản phẩm.
- Comment mơ hồ vào Review Queue thay vì tự xử lý sai.
- Manual Capture Mode dùng được ngay cả khi connector tắt.
- Sau live xuất được CSV/Excel.
- App không bị đứng khi có feed comment liên tục.
- Có log đủ để biết connector lỗi ở stage nào.

