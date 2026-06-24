# Module 10 - Parser

## Trách nhiệm

Hiểu comment mua hàng và trích xuất intent, SKU, variant, số lượng.

## Task chi tiết

- Xây dựng text normalizer:
  - lowercase
  - trim khoảng trắng
  - chuẩn hóa Unicode
  - bỏ emoji/ký tự điều khiển
  - tạo thêm bản text không dấu để match SKU/keyword
- Định nghĩa buy keywords:
  - chốt
  - lấy
  - mua
  - lấy em
  - cho mình
- Định nghĩa cancel/noise keywords:
  - hủy
  - thôi
  - giá
  - còn không
- SKU extraction:
  - exact SKU
  - SKU alias
  - SKU nằm một mình trong comment
  - SKU đi sau keyword mua
- Quantity extraction:
  - `2`
  - `2 cái`
  - `x2`
  - mặc định `1`
- Variant extraction:
  - size: S/M/L/XL/XXL, số size
  - màu: đỏ, đen, trắng, xanh...
  - alias màu do shop cấu hình
- Confidence scoring:
  - match SKU exact: điểm cao
  - có keyword mua: cộng điểm
  - thiếu variant bắt buộc: needs_review
  - comment chứa nhiều SKU: needs_review
- Parser config theo shop:
  - keyword tùy chỉnh
  - alias SKU
  - màu/size tùy chỉnh
- Unit test parser với nhiều comment thật.
- Tạo parser debug view cho dev/test:
  - input
  - normalized text
  - matched rule
  - output

## Task coding chi tiết

### M10-T01 - Tạo ParsedIntent model

- Mục tiêu: chuẩn hóa output parser.
- File/thư mục dự kiến: `src-tauri/src/parser/models.rs`, `src/types/parser.ts`.
- Dependency: M05-T01.
- Output: struct/type `ParsedIntent`.
- Done khi: serialize output parser thành JSON.

### M10-T02 - Tạo text normalizer

- Mục tiêu: chuẩn hóa comment trước khi match.
- File/thư mục dự kiến: `src-tauri/src/parser/normalizer.rs`.
- Dependency: M10-T01.
- Output: normalized text có dấu và không dấu.
- Done khi: test lowercase, trim, bỏ emoji, bỏ dấu pass.

### M10-T03 - Tạo keyword matcher

- Mục tiêu: nhận diện intent mua/hủy/hỏi.
- File/thư mục dự kiến: `src-tauri/src/parser/keywords.rs`.
- Dependency: M10-T02.
- Output: match `chốt`, `lấy`, `mua`, `hủy`, `giá`.
- Done khi: test intent `buy/question/cancel/noise` pass.

### M10-T04 - Tạo SKU extractor

- Mục tiêu: lấy SKU từ comment.
- File/thư mục dự kiến: `src-tauri/src/parser/sku_extractor.rs`.
- Dependency: M04-T05, M10-T02.
- Output: SKU candidate list.
- Done khi: exact/alias/bỏ dấu match đúng.

### M10-T05 - Tạo quantity extractor

- Mục tiêu: lấy số lượng.
- File/thư mục dự kiến: `src-tauri/src/parser/quantity.rs`.
- Dependency: M10-T02.
- Output: parse `2`, `2 cái`, `x2`; default 1.
- Done khi: test quantity pass.

### M10-T06 - Tạo variant extractor

- Mục tiêu: lấy size/màu.
- File/thư mục dự kiến: `src-tauri/src/parser/variant.rs`.
- Dependency: M10-T02, M04-T03.
- Output: parse size/color và alias.
- Done khi: `A12 đỏ M` trả color đỏ size M.

### M10-T07 - Tạo confidence scoring

- Mục tiêu: quyết định tự xử lý hay review.
- File/thư mục dự kiến: `src-tauri/src/parser/confidence.rs`.
- Dependency: M10-T03, M10-T04, M10-T06.
- Output: score 0..1 và `needs_review`.
- Done khi: thiếu variant bắt buộc -> review.

### M10-T08 - Tạo ParserService

- Mục tiêu: gom các extractor thành pipeline parser.
- File/thư mục dự kiến: `src-tauri/src/parser/service.rs`.
- Dependency: M10-T02 đến M10-T07.
- Output: `parse_comment(event, session_config) -> ParsedIntent`.
- Done khi: comment mẫu tạo output đúng.

### M10-T09 - Tạo parser test corpus

- Mục tiêu: kiểm thử bằng comment thật/mẫu.
- File/thư mục dự kiến: `src-tauri/src/parser/tests.rs`, `fixtures/parser_comments.json`.
- Dependency: M10-T08.
- Output: bộ test tối thiểu 50 comment.
- Done khi: test pass và case mơ hồ vào review.

### M10-T10 - Tạo parser debug command

- Mục tiêu: dev/UI xem parser quyết định gì.
- File/thư mục dự kiến: `src-tauri/src/commands/parser.rs`.
- Dependency: M10-T08.
- Output: command `debug_parse_comment`.
- Done khi: truy cập từ mục "Cài đặt" -> "Công cụ nâng cao" -> màn hình test cú pháp hoạt động chuẩn.

## Done khi

- Parser xử lý được các cú pháp mua hàng phổ biến.
- Trường hợp mơ hồ được đưa vào review, không tự động trừ tồn sai.
