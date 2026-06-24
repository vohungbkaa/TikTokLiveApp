# 00 - Mục tiêu và nguyên tắc kiến trúc

## Mục tiêu kỹ thuật

Xây dựng ứng dụng desktop nhẹ, dễ cài đặt, dễ dùng để hỗ trợ seller livestream TikTok lên đơn từ comment. Hệ thống phải dễ mở rộng, dễ bảo trì và không phụ thuộc cứng vào một nguồn comment duy nhất.

## Nguyên tắc kiến trúc

- Offline-first: dữ liệu ca live, sản phẩm, đơn nháp và lịch sử xử lý phải lưu local trước.
- Modular: mỗi module có trách nhiệm rõ ràng, giao tiếp qua interface/schema ổn định.
- Event-driven: mọi nguồn comment đều được chuẩn hóa thành `LiveEvent` trước khi đi vào pipeline xử lý đơn.
- Replaceable connector: TikTok connector có thể chết hoặc thay đổi, nhưng order core vẫn phải hoạt động.
- Fail-safe operation: khi tự động đọc comment lỗi, app vẫn có Manual Capture Mode để tiếp tục lên đơn.
- MVP nhỏ, chắc, tập trung vào live order workflow; tránh biến app thành POS đầy đủ quá sớm.

## Quy tắc bảo trì lâu dài

- Không để UI gọi trực tiếp sidecar. UI chỉ gọi Rust command/state.
- Không để TikTok connector format lan vào order core. Luôn normalize thành `LiveEvent`.
- Không tự động trừ tồn nếu parser confidence thấp.
- Mọi thao tác tạo claim/order phải chạy trong transaction.
- Mọi connector phải emit health/error theo cùng protocol.
- Mỗi selling mode phải có test riêng.
- Manual Capture Mode luôn phải hoạt động, kể cả khi mọi connector tắt.
- Auto-DM không được thêm vào core workflow cho đến khi có đánh giá rủi ro đầy đủ.

