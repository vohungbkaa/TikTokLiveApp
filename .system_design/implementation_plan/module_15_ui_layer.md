# Module 15 - UI Layer

## Trách nhiệm

Cung cấp giao diện vận hành live nhanh, rõ, ít thao tác.

## Màn hình MVP

- Live Setup:
  - tạo phiên live
  - nhập username/link
  - import/nhập sản phẩm
  - chọn selling mode
- Live Console:
  - trạng thái connector
  - feed comment
  - manual input
  - bảng sản phẩm/tồn
  - order draft board
  - review queue
- Order Review:
  - danh sách đơn
  - sửa/xác nhận/hủy
  - in/xuất
- Settings:
  - keyword parser
  - export template
  - connector/fallback config

## Task chi tiết

- Dùng `Vue 3`, `TypeScript`, `Pinia`, `Vue Router`.
- Dùng `lucide-vue-next` cho icon.
- Dùng `vue-virtual-scroller` cho comment feed.
- Dùng component nội bộ và CSS/Tailwind utility, không dùng framework UI nặng trong MVP.
- Thiết kế layout Live Console dạng vận hành:
  - cột trái: comment feed
  - giữa: order draft/claim mới
  - phải: sản phẩm/tồn/review queue
- Tạo component connector status.
- Tạo component comment feed dùng virtual list.
- Tạo component manual capture input.
- Tạo component product table.
- Tạo component order draft card/table.
- Tạo component review queue.
- Tạo toast/alert tối giản:
  - connector lỗi
  - chuyển fallback
  - hết hàng
  - cần xác nhận
- Tạo keyboard shortcuts.
- Tối ưu render khi nhiều comment.
- Không dùng card lồng card, giữ giao diện gọn.

## Task coding chi tiết

### M15-T01 - Tạo app routes và layout nền

- Mục tiêu: có navigation tối thiểu.
- File/thư mục dự kiến: `src/app/router.ts`, `src/app/App.vue`, `src/app/MainLayout.vue`.
- Dependency: M01-T02.
- Output: routes Live Setup, Live Console, Order Review, Settings.
- Done khi: chuyển route không reload app.

### M15-T02 - Tạo Pinia stores

- Mục tiêu: quản lý state UI.
- File/thư mục dự kiến: `src/stores/session.ts`, `connector.ts`, `orders.ts`, `products.ts`.
- Dependency: M15-T01.
- Output: stores có state/actions cơ bản.
- Done khi: component đọc/ghi state qua Pinia.

### M15-T03 - Tạo Live Setup screen

- Mục tiêu: tạo phiên live và nhập username/link.
- File/thư mục dự kiến: `src/features/session-setup/LiveSetupView.vue`.
- Dependency: M03-T03, M04-T08.
- Output: form tạo session và chọn sản phẩm.
- Done khi: tạo session xong chuyển sang Live Console.

### M15-T04 - Tạo ConnectorStatus component

- Mục tiêu: hiển thị trạng thái WebSocket/Scrapling/Manual.
- File/thư mục dự kiến: `src/features/live-console/components/ConnectorStatus.vue`.
- Dependency: M08-T10.
- Output: badge healthy/degraded/fallback/manual_required.
- Done khi: nhận event status và cập nhật UI.

### M15-T05 - Tạo CommentFeed virtual list

- Mục tiêu: hiển thị comment realtime không giật.
- File/thư mục dự kiến: `src/features/live-console/components/CommentFeed.vue`.
- Dependency: M05-T06.
- Output: danh sách comment dùng `vue-virtual-scroller`.
- Done khi: render 5.000 comment mock vẫn mượt.

### M15-T06 - Tạo ProductStockPanel

- Mục tiêu: xem tồn kho trong phiên.
- File/thư mục dự kiến: `src/features/live-console/components/ProductStockPanel.vue`.
- Dependency: M04-T08.
- Output: bảng SKU, tên, mode, tồn.
- Done khi: tồn cập nhật sau claim.

### M15-T07 - Tạo OrderDraftBoard

- Mục tiêu: xem đơn nháp theo khách.
- File/thư mục dự kiến: `src/features/live-console/components/OrderDraftBoard.vue`.
- Dependency: M12-T04.
- Output: danh sách order draft.
- Done khi: comment mua hàng tạo card/order row mới.

### M15-T08 - Tạo ReviewQueue

- Mục tiêu: xử lý comment mơ hồ/thiếu thông tin.
- File/thư mục dự kiến: `src/features/live-console/components/ReviewQueue.vue`.
- Dependency: M11-T06, M12-T04.
- Output: danh sách pending_info/needs_review.
- Done khi: nhân viên xác nhận/sửa được item.

### M15-T09 - Tích hợp ManualCaptureInput

- Mục tiêu: nhập tay ngay trong Live Console.
- File/thư mục dự kiến: `src/features/live-console/LiveConsoleView.vue`.
- Dependency: M09-T01 đến M09-T06.
- Output: manual input luôn sẵn sàng.
- Done khi: nhập manual tạo order draft.

### M15-T10 - Tạo toast/alert system

- Mục tiêu: cảnh báo lỗi nhẹ.
- File/thư mục dự kiến: `src/components/ToastHost.vue`, `src/stores/toast.ts`.
- Dependency: M15-T02.
- Output: toast connector lỗi, chuyển fallback, hết hàng.
- Done khi: store push toast hiển thị đúng.

### M15-T11 - Tạo Order Review screen

- Mục tiêu: xem/sửa/xuất đơn sau live.
- File/thư mục dự kiến: `src/features/orders/OrderReviewView.vue`.
- Dependency: M12-T04, M14-T04.
- Output: table orders + nút export/print.
- Done khi: export CSV từ UI.

### M15-T12 - Tạo Settings screen MVP

- Mục tiêu: cấu hình keyword/parser/fallback cơ bản.
- File/thư mục dự kiến: `src/features/settings/SettingsView.vue`.
- Dependency: M02-T06, M10-T08.
- Output: form settings lưu local.
- Done khi: thay keyword parser và parse dùng config mới.

## Done khi

- Nhân viên vận hành được live trên một màn hình chính.
- Comment mới và đơn mới không làm UI giật.
- Các trạng thái lỗi/fallback dễ hiểu.
