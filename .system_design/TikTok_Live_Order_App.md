# TikTok Live Order App - Tổng quan sản phẩm và kỹ thuật

## 1. Định vị sản phẩm

Ứng dụng desktop chốt đơn livestream TikTok: đọc comment realtime, hiểu comment mua hàng, gom đơn theo khách, trừ tồn kho, hỗ trợ phân xử ai chốt trước khi cần, và in/xuất đơn nhanh sau live.

Sản phẩm không cần cạnh tranh với các hệ thống POS/quản lý bán hàng lớn bằng cách làm thật nhiều tính năng. Hướng cạnh tranh là:

- Nhẹ, cài đặt nhanh, không yêu cầu seller biết kỹ thuật.
- Mở app là live được, thao tác ít, giao diện rõ.
- Chạy local/offline-first để không phụ thuộc cloud cho các thao tác quan trọng.
- Tập trung vào quy trình live: đọc comment, lên đơn, trừ tồn, in/xuất đơn.
- Có cơ chế cứu phiên live khi TikTok connector lỗi: Scrapling fallback và Manual Capture Mode.

Thông điệp sản phẩm:

```text
App chốt đơn livestream TikTok nhẹ, dễ dùng, lên đơn từ comment nhanh và không làm dừng phiên live khi connector gặp lỗi.
```

## 2. Khách hàng mục tiêu

- Seller cá nhân, hộ kinh doanh, shop nhỏ và vừa đang livestream trên TikTok.
- Shop bán mỹ phẩm, đồ gia dụng, thực phẩm, phụ kiện, quần áo có tồn kho số lượng.
- Shop bán hàng si, hàng vintage, hàng độc bản, hàng handmade, đồ sưu tầm hoặc sản phẩm mỗi mẫu chỉ có 1/vài cái.
- Shop bán preorder, gom đơn sau live, hàng order theo đợt.
- Shop muốn một công cụ nhẹ hơn các hệ thống POS lớn, không cần CRM/phụ kiện phức tạp ngay từ đầu.

## 3. Lợi thế cạnh tranh so với các hệ thống lớn

| Hướng của các hệ thống lớn | Hướng của sản phẩm này |
| --- | --- |
| Hệ sinh thái rộng, nhiều module | Một công cụ live order gọn, tập trung vào ca live |
| Phụ thuộc cloud và quy trình POS đầy đủ | Offline-first, dữ liệu và logic chính chạy local |
| Nhiều màn hình, nhiều cấu hình | Ít nút, thao tác nhanh, setup ngắn |
| Tối ưu cho bán hàng đa kênh | Tối ưu cho livestream TikTok và quy trình comment -> đơn |
| Nếu connector lỗi thì seller dễ bị dừng | Có Manual Capture Mode để vẫn lên đơn được |

Mục tiêu không phải thay thế POS lớn ngay lập tức. Mục tiêu là trở thành công cụ mà seller mở trong lúc live vì nó nhanh, nhẹ và làm đúng việc.

## 4. Vấn đề cần giải quyết

| Pain point | Giải pháp trong app |
| --- | --- |
| Comment trôi nhanh, dễ sót đơn | Đọc comment realtime và hiển thị feed vận hành riêng cho nhân viên live |
| Khách comment nhiều lần, nhiều sản phẩm | Gom đơn theo khách, cộng item theo comment |
| Comment sai cú pháp, thiếu size/màu | Parser rule-based, đưa đơn thiếu thông tin vào hàng đợi cần xác nhận |
| Sản phẩm có số lượng tồn | Stock Mode: trừ tồn theo số lượng, báo hết hàng khi vượt tồn |
| Sản phẩm độc bản/số lượng ít | Unique Mode: khóa sản phẩm cho người comment hợp lệ đầu tiên |
| Bán preorder/gom đơn | Preorder Mode: nhận đơn theo quota hoặc không cần trừ tồn chặt |
| Trùng nickname, đổi avatar, nhầm khách | Ưu tiên định danh bằng `userId`/`uniqueId` từ payload khi engine đọc được dữ liệu |
| Viết tay đơn sau live chậm và dễ sai | Tạo order draft, gom đơn, in nhiệt hoặc xuất CSV/Excel |
| TikTok đổi giao thức làm engine lỗi | Smart fallback sang Scrapling hoặc Manual Capture Mode |

## 5. Nguyên tắc thiết kế

- App phải nhẹ và mở nhanh. Không biến MVP thành một POS đầy đủ.
- Màn hình đầu tiên là màn hình vận hành live, không phải landing page hay dashboard marketing.
- Mọi thao tác quan trọng trong ca live phải có phím tắt hoặc flow rất ngắn.
- Không hứa "chính xác 100%" vì hệ thống phụ thuộc TikTok và reverse-engineering.
- Module lấy comment phải tách khỏi module lên đơn. TikTok connector chết thì parser/order core vẫn dùng được.
- Dữ liệu ca live lưu local trước. Cloud chỉ là tùy chọn để đồng bộ, license, update và log.

## 6. Các chế độ bán hàng

### 6.1 Stock Mode - Hàng có tồn kho

Dùng cho shop bán sản phẩm có số lượng:

- Mỹ phẩm.
- Đồ gia dụng.
- Đồ ăn/thực phẩm.
- Phụ kiện.
- Quần áo có size/màu và tồn kho.

Ví dụ comment:

```text
chốt A12
A12 2 cái
lấy kem01
A12 size M màu đen
```

Xử lý:

- Parse SKU, size, màu, số lượng.
- Trừ tồn theo variant.
- Gom item vào đơn của khách.
- Nếu thiếu size/màu, đưa vào `pending_info`.
- Nếu hết hàng, đưa vào `out_of_stock` hoặc waitlist.

### 6.2 Unique Mode - Hàng độc bản/số lượng ít

Dùng cho hàng si, hàng vintage, handmade, đồ sưu tầm, sản phẩm mỗi mẫu có số lượng rất ít.

Xử lý:

- Ai comment hợp lệ trước thì thắng.
- Người sau vào `lost` hoặc `waitlist`.
- UI hiển thị rõ winner để nhân viên live đọc tên minh bạch.
- Có lịch sử để audit khi có tranh cãi.

### 6.3 Preorder Mode - Gom đơn/đặt trước

Dùng cho hàng order, flash sale, gom đơn theo đợt.

Xử lý:

- Nhận comment mua hàng mà không cần trừ tồn chặt.
- Có thể giới hạn quota nếu shop cấu hình.
- Sau live xuất danh sách đặt hàng/gom đơn.

## 7. Kiến trúc tổng quan

```text
Vue UI
  |
  | Tauri IPC
  v
Rust Core
  |-- Event Normalizer
  |-- Comment Parser
  |-- Selling Mode Engine
  |-- Stock/Unique/Preorder Rules
  |-- SQLite Local DB
  |-- Print/Export Service
  |
  | Sidecar stdio/local API
  v
Node Sidecar: TikTok-Live-Connector
  |
  v
TikTok Live Webcast/WebSocket

Python Sidecar: Scrapling
  |
  v
TikTok Web DOM Fallback

Manual Capture Mode
  |
  v
Nhân viên nhập nhanh comment vào app khi engine tự động gặp lỗi
```

## 8. Tech stack đề xuất

- Desktop framework: `Tauri`.
- Frontend: `Vue 3`.
- Backend core: `Rust`.
- Local database: `SQLite`.
- Primary live engine: `Node.js sidecar` dùng `TikTok-Live-Connector`.
- Fallback browser engine: `Python sidecar` dùng `Scrapling`.
- Update: `Tauri Updater`.
- Server mỏng: license, remote config, kill switch, update manifest, log lỗi diện rộng.

Không nên đưa Auto-DM vào MVP vì nó làm app nặng hơn, tăng rủi ro tài khoản/session và tăng phụ thuộc browser automation. MVP chỉ cần tạo danh sách khách cần liên hệ, copy mẫu tin nhắn, xuất đơn.

## 9. Chiến lược lấy comment

### 9.1 Primary Engine - WebSocket

Dùng `TikTok-Live-Connector` làm engine chính để kết nối luồng TikTok Live Webcast/WebSocket.

Ưu điểm:

- Realtime.
- Nhẹ RAM/CPU.
- Lấy được comment và một số event live khác.
- Không cần người dùng đăng nhập TikTok nếu chỉ đọc live public.

Rủi ro:

- Là giải pháp reverse-engineering, không phải API chính thức.
- TikTok có thể đổi giao thức, schema protobuf, signing hoặc header.
- Cần health check, telemetry, remote config, auto-update và fallback.

### 9.2 Fallback Engine - Scrapling

Dùng `Scrapling`/browser automation để đọc DOM comment khi WebSocket engine bị lỗi.

Vai trò:

- Chỉ dùng để cứu phiên live.
- Không chạy mặc định để tránh tốn CPU/RAM.
- Khi primary engine lỗi liên tục, app hiển thị thông báo ngắn và chuyển sang chế độ dự phòng.

Hạn chế:

- Có thể sót comment khi live đông vì DOM không render tất cả comment.
- Tốn tài nguyên máy.
- Khó lấy `userId` chính xác như WebSocket payload.

### 9.3 Manual Capture Mode

Manual Capture Mode là chế độ nhập tay siêu nhanh để cứu phiên live khi WebSocket và Scrapling gặp lỗi, hoặc khi máy seller quá yếu để chạy browser fallback.

Nhân viên live nhập nhanh vào app:

```text
@username A12
linh A12 2 cái
mai kem01
chốt A12 đỏ L
```

App vẫn xử lý như comment thật:

- Normalize text.
- Parse mã hàng, size, màu, số lượng.
- Áp dụng Stock/Unique/Preorder rule.
- Gom đơn theo khách.
- Tạo order draft.
- In hoặc xuất đơn.

Tính năng cần có:

- Ô nhập nhanh luôn focus.
- Autocomplete SKU.
- Gợi ý khách đã từng mua.
- Phím tắt để xác nhận/sửa/bỏ qua.
- Ghi rõ source của event là `manual` để audit.
- Cảnh báo UI: thứ tự chốt dựa trên thứ tự nhân viên nhập, không phải timestamp TikTok.

Giá trị:

- Không phụ thuộc TikTok.
- Không tốn CPU.
- Không rủi ro checkpoint tài khoản.
- Giữ được quy trình lên đơn khi engine tự động chết.

## 10. Pipeline xử lý comment lên đơn

```text
Raw Event
  -> Normalize Event
  -> Deduplicate
  -> Parse Intent
  -> Resolve SKU/Variant
  -> Apply Selling Mode Rule
  -> Create Claim/Order Draft
  -> UI Confirm/Review
  -> Print/Export
```

### 10.1 Normalize Event

Mọi nguồn comment đều được chuyển về schema nội bộ:

```json
{
  "source": "websocket | scrapling | manual",
  "platform": "tiktok",
  "event_id": "string",
  "user_id": "string | null",
  "unique_id": "string | null",
  "display_name": "string | null",
  "comment": "string",
  "ts_platform": "datetime | null",
  "ts_received": "datetime",
  "raw": {}
}
```

### 10.2 Parser comment

Nên bắt đầu bằng rule-based parser, không cần AI cho MVP.

Hỗ trợ mẫu comment:

```text
A12
chốt A12
lấy A12
A12 2 cái
A12 size M
A12 đỏ M
chốt áo 12 size m
```

Parser trả về:

```json
{
  "intent": "buy | question | noise",
  "sku": "A12",
  "variant": {
    "size": "M",
    "color": "đỏ"
  },
  "qty": 1,
  "confidence": 0.92,
  "needs_review": false
}
```

Nếu confidence thấp, app không tự khóa/trừ tồn mà đưa vào hàng đợi cần nhân viên xác nhận.

### 10.3 Selling Mode Rule

```text
Stock Mode:
- Trừ tồn theo SKU/variant.
- Nếu còn tồn thì tạo item vào order draft.
- Nếu hết tồn thì báo hết hàng hoặc đưa vào waitlist.

Unique Mode:
- Comment hợp lệ đầu tiên thắng.
- Các comment sau vào lost/waitlist.
- Giữ audit trail để xem lại khi tranh cãi.

Preorder Mode:
- Nhận đơn theo quota hoặc không giới hạn.
- Không cần trừ tồn chặt như Stock Mode.
```

## 11. Data model local

Bảng tối thiểu trong SQLite:

```text
live_sessions
live_events
products
product_variants
order_claims
orders
order_items
customers
settings
```

Cột quan trọng:

```text
products:
- id
- sku
- name
- price
- selling_mode: stock | unique | preorder
- stock_qty
- is_active

product_variants:
- id
- product_id
- sku
- size
- color
- stock_qty
- price

live_events:
- id
- session_id
- source
- platform_event_id
- user_id
- unique_id
- display_name
- comment
- ts_platform
- ts_received
- raw_json

order_claims:
- id
- session_id
- event_id
- user_id
- sku
- variant_json
- qty
- confidence
- status: confirmed | won | lost | waitlist | out_of_stock | pending_info | needs_review | cancelled
- created_at

orders:
- id
- session_id
- customer_id
- status: draft | pending_info | confirmed | printed | exported | cancelled
- total_amount
- created_at
```

## 12. UI MVP

Màn hình đầu tiên là công cụ vận hành live.

Thành phần chính:

- Ô nhập `@username` hoặc link live.
- Nút kết nối live.
- Trạng thái engine: WebSocket, Scrapling, Manual.
- Feed comment realtime.
- Ô Manual Capture nhập nhanh.
- Bảng sản phẩm và tồn còn lại.
- Danh sách order draft theo khách.
- Hàng đợi cần xác nhận.
- Danh sách hết hàng/waitlist.
- Nút in đơn/xuất CSV.

Nguyên tắc UI:

- Ít màn hình, ít cấu hình.
- Nút rõ nghĩa, thao tác 1-2 bước.
- Feed comment và đơn mới phải đọc được khi live đông.
- Ưu tiên phím tắt cho nhân viên vận hành.
- Không cần dashboard phức tạp trong MVP.

## 13. Luồng sử dụng MVP

### Trước live

1. Tạo phiên live.
2. Import hoặc nhập nhanh danh sách sản phẩm.
3. Chọn selling mode cho sản phẩm: stock, unique hoặc preorder.
4. Bấm kết nối TikTok live.

### Trong live

1. App đọc comment.
2. Parser nhận diện comment mua hàng.
3. Engine áp dụng rule theo mode.
4. Đơn hợp lệ vào order draft.
5. Đơn thiếu thông tin vào hàng đợi cần xác nhận.
6. Nếu connector lỗi, app chuyển Scrapling hoặc Manual Capture Mode.

### Sau live

1. Kiểm tra đơn draft.
2. In đơn hoặc xuất CSV/Excel.
3. Lưu lịch sử phiên live.

## 14. Roadmap

### Phase 1 - MVP nhẹ và dùng được

- Tauri + Vue + Rust.
- SQLite local.
- Node sidecar đọc comment bằng `TikTok-Live-Connector`.
- Normalize event.
- Rule-based parser.
- Stock Mode và Unique Mode.
- Manual Capture Mode.
- Tạo order draft.
- Export CSV/Excel hoặc in đơn cơ bản.

### Phase 2 - Ổn định và cứu live

- Auto reconnect.
- Health check engine.
- Smart fallback sang Scrapling.
- Remote config/kill switch.
- Tauri auto updater.
- Log lỗi gửi về server.
- Virtual list cho comment feed lớn.

### Phase 3 - Vận hành shop

- Preorder Mode.
- Quản lý khách hàng cơ bản.
- Lịch sử boom/blacklist nội bộ của shop.
- Mẫu tin nhắn cần copy cho khách.
- Tích hợp máy in nhiệt tốt hơn.
- Đồng bộ cloud tùy chọn.

### Phase 4 - Mở rộng

- Tích hợp vận chuyển.
- Mini-game live.
- OCR screen fallback nếu cần.
- Auto-DM chỉ làm sau khi đánh giá kỹ rủi ro chính sách và tài khoản.

## 15. Kết luận

Sản phẩm nên cạnh tranh bằng độ nhẹ, độ rõ ràng và tốc độ vận hành trong ca live, không cần cạnh tranh trực diện bằng một hệ sinh thái POS đầy đủ.

Giá trị cốt lõi:

- Lên đơn từ comment nhanh.
- Hỗ trợ cả hàng có tồn kho và hàng độc bản.
- Giảm sót đơn, giảm nhầm khách.
- Giữ quy trình bán hàng tiếp tục khi TikTok connector lỗi.
- Cài đặt dễ, dùng dễ, thao tác nhanh.

MVP tốt nhất là một live order console offline-first: mở lên, kết nối live, nhập/import sản phẩm, comment tự động biến thành đơn, và nếu tự động lỗi thì vẫn có Manual Capture Mode để không dừng phiên bán hàng.
