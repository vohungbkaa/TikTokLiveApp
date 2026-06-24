# Module 04 - Product & Inventory

## Trách nhiệm

Quản lý sản phẩm, SKU, variant, tồn kho và selling mode.

## Task chi tiết

- Tạo model:
  - `Product`
  - `ProductVariant`
  - `InventorySnapshot`
- Hỗ trợ nhập sản phẩm thủ công:
  - SKU
  - tên
  - giá
  - tồn kho
  - selling mode
- Hỗ trợ variant:
  - size
  - màu
  - giá riêng theo variant
  - tồn kho riêng
- Hỗ trợ import CSV/Excel cơ bản.
- Hỗ trợ export danh sách sản phẩm.
- Tạo SKU resolver:
  - match exact SKU
  - match không phân biệt hoa/thường
  - match bỏ dấu tiếng Việt
  - alias SKU do shop cấu hình
- Tạo inventory service:
  - check tồn
  - reserve/trừ tồn
  - release tồn khi hủy claim
  - snapshot tồn đầu phiên và cuối phiên
- Tạo cảnh báo:
  - sắp hết hàng
  - hết hàng
  - SKU bị trùng
- Tạo API Tauri cho UI:
  - list products
  - create/update/delete product
  - import products
  - update stock

## Task coding chi tiết

### M04-T01 - Định nghĩa model Product/Variant

- Mục tiêu: thống nhất kiểu dữ liệu sản phẩm.
- File/thư mục dự kiến: `src-tauri/src/products/models.rs`, `src/types/product.ts`.
- Dependency: M02-T04.
- Output: struct/type `Product`, `ProductVariant`, `SellingMode`.
- Done khi: model serialize được qua Tauri command.

### M04-T02 - Tạo ProductRepository CRUD

- Mục tiêu: thao tác DB cho sản phẩm.
- File/thư mục dự kiến: `src-tauri/src/products/repository.rs`.
- Dependency: M04-T01.
- Output: `create`, `update`, `delete`, `list`, `get_by_sku`.
- Done khi: unit test CRUD pass.

### M04-T03 - Tạo VariantRepository CRUD

- Mục tiêu: quản lý size/màu/tồn theo variant.
- File/thư mục dự kiến: `src-tauri/src/products/variant_repository.rs`.
- Dependency: M04-T02.
- Output: CRUD variant theo `product_id`.
- Done khi: một product có nhiều variant và query được.

### M04-T04 - Tạo InventoryService

- Mục tiêu: check/trừ/release tồn qua một service duy nhất.
- File/thư mục dự kiến: `src-tauri/src/inventory/service.rs`.
- Dependency: M04-T02, M04-T03.
- Output: `check_stock`, `reserve_stock`, `release_stock`.
- Done khi: test không trừ tồn âm.

### M04-T05 - Tạo SKU Resolver

- Mục tiêu: tìm sản phẩm từ text comment.
- File/thư mục dự kiến: `src-tauri/src/products/sku_resolver.rs`.
- Dependency: M04-T02.
- Output: resolve exact, case-insensitive, bỏ dấu, alias.
- Done khi: test `A12`, `a12`, `áo 12`, alias đều match đúng.

### M04-T06 - Tạo import CSV sản phẩm

- Mục tiêu: nhập nhanh danh sách sản phẩm.
- File/thư mục dự kiến: `src-tauri/src/products/import.rs`.
- Dependency: M04-T02.
- Output: parser CSV columns `sku,name,price,stock_qty,selling_mode,size,color`.
- Done khi: import file mẫu tạo đúng product/variant.

### M04-T07 - Tạo export CSV sản phẩm

- Mục tiêu: xuất danh sách sản phẩm để backup/sửa ngoài app.
- File/thư mục dự kiến: `src-tauri/src/products/export.rs`.
- Dependency: M04-T02.
- Output: CSV product list.
- Done khi: file export import lại được.

### M04-T08 - Tạo Tauri commands product

- Mục tiêu: UI gọi được product API.
- File/thư mục dự kiến: `src-tauri/src/commands/product.rs`.
- Dependency: M04-T02, M04-T04, M04-T06.
- Output: commands `create_product`, `list_products`, `import_products_csv`, `update_stock`.
- Done khi: UI gọi command và dữ liệu thay đổi trong DB.

### M04-T09 - Tạo cảnh báo tồn kho

- Mục tiêu: báo hết/sắp hết hàng.
- File/thư mục dự kiến: `src-tauri/src/inventory/alerts.rs`.
- Dependency: M04-T04.
- Output: function trả `in_stock`, `low_stock`, `out_of_stock`.
- Done khi: UI nhận đúng trạng thái tồn.

## Done khi

- User nhập/import được sản phẩm.
- Parser resolve SKU từ comment bằng exact/case-insensitive/bỏ dấu/alias.
- Selling Mode Engine check và trừ tồn qua Inventory Service.
