# Kế hoạch triển khai hệ thống TikTok Live Order App

Tài liệu này là mục lục tổng hợp. Chi tiết task đã được tách theo từng module trong thư mục `implementation_plan/` để dễ giao việc, bảo trì và mở rộng.

## Chức năng của app

TikTok Live Order App là ứng dụng desktop hỗ trợ seller chốt đơn khi livestream TikTok. App đọc comment live, hiểu comment mua hàng, gom đơn theo từng khách, xử lý tồn kho theo chế độ bán, tạo đơn nháp và hỗ trợ in hoặc xuất danh sách đơn sau live.

Workflow chính:

```text
Comment TikTok / nhập tay
  -> chuẩn hóa event
  -> đọc ý định mua hàng
  -> nhận diện SKU, size, màu, số lượng
  -> áp dụng rule bán hàng
  -> tạo claim/order draft
  -> nhân viên xác nhận, sửa, in hoặc xuất file
```

Các nhóm vấn đề app cần giải quyết cho shop:

| Khó khăn của khách hàng | Cách app giải quyết |
| --- | --- |
| Khi live, comment trôi nhanh khiến nhân viên dễ sót khách chốt đơn. | App đọc comment realtime và đưa về một màn hình vận hành riêng để nhân viên theo dõi các comment có ý định mua hàng. |
| Khách comment không theo một mẫu cố định, ví dụ "chốt A12", "lấy 2 cái", "áo đỏ size M". | App parse comment theo rule để nhận diện SKU, size, màu, số lượng và ý định mua hàng. |
| Một khách có thể comment nhiều lần, mua nhiều sản phẩm khác nhau trong cùng phiên live. | App gom các comment hợp lệ vào cùng một đơn nháp theo từng khách để tránh tạo nhiều đơn rời rạc. |
| Comment thiếu thông tin như size, màu, số lượng hoặc app không đủ tự tin để tự chốt. | App đưa các trường hợp mơ hồ vào hàng đợi xác nhận để nhân viên kiểm tra trước khi tạo đơn chính thức trong app. |
| Hàng có tồn kho dễ bị bán quá số lượng, nhất là khi nhiều khách chốt cùng một mã. | Stock Mode kiểm tra tồn theo SKU/variant, trừ tồn trong giao dịch an toàn và cảnh báo khi hết hàng. |
| Hàng độc bản, hàng si hoặc hàng chỉ có một vài mẫu dễ phát sinh tranh cãi ai chốt trước. | Unique Mode ghi nhận người comment hợp lệ đầu tiên là winner, các khách sau vào danh sách chờ/thua để nhân viên đối soát minh bạch. |
| Shop bán preorder hoặc gom đơn sau live không cần trừ tồn chặt ngay tại thời điểm khách comment. | Preorder Mode cho phép nhận đơn theo quota hoặc nhận danh sách đặt trước để xử lý sau live. |
| TikTok connector có thể lỗi, TikTok đổi giao thức hoặc máy seller không chạy được fallback nặng. | App có Scrapling fallback và Manual Capture Mode để nhân viên vẫn nhập nhanh đơn, không phải dừng phiên live. |
| Sau live, shop mất thời gian chép lại đơn, lọc khách, xuất file hoặc in đơn. | App tạo order draft trong lúc live, hỗ trợ rà soát, in nhanh và xuất CSV/Excel sau live. |
| Shop gặp khách có lịch sử đặt rồi bom hàng, tài khoản nghi đối thủ phá live hoặc nhiều lần chốt ảo làm rối vận hành. | App cho phép shop gắn tag rủi ro nội bộ như "cần xác minh", "boom hàng", "nghi phá live"; khi khách đó comment hoặc tạo đơn mới, app cảnh báo để nhân viên kiểm tra trước khi giữ hàng. |
| Nhiều shop muốn chia sẻ cảnh báo về khách bom hàng để giảm thiệt hại chung. | Phase sau có thể có danh sách cảnh báo cộng đồng dạng opt-in, chỉ chia sẻ dữ liệu đã ẩn bớt thông tin nhạy cảm, có bằng chứng đơn/hành vi, điểm tin cậy, cơ chế khiếu nại và không tự động cấm khách nếu chưa có nhân viên xác nhận. |
| Chủ shop khó biết lỗi nằm ở comment, parser, tồn kho hay connector khi app vận hành không ổn định. | App ghi log, health check và hiển thị trạng thái nguồn comment để dễ chẩn đoán sự cố. |
| Shop nhỏ không muốn dùng một hệ thống POS quá nặng, nhiều màn hình và khó cài đặt. | App tập trung vào quy trình live order, chạy local/offline-first, thao tác ít và ưu tiên màn hình vận hành live. |

Phạm vi MVP không phải POS đầy đủ và không tự tạo đơn TikTok Shop chính thức. App tạo đơn nội bộ/đơn nháp từ comment để shop xác nhận, xử lý và xuất/in; đơn TikTok Shop thật chỉ phát sinh khi khách đặt hàng qua TikTok Shop hoặc khi sau này có tích hợp API chính thức phù hợp.

## Tài liệu tổng quan

- [00 - Mục tiêu và nguyên tắc kiến trúc](implementation_plan/00_muc_tieu_kien_truc.md)
- [01 - Kiến trúc module tổng thể](implementation_plan/01_kien_truc_module.md)
- [02 - Chuẩn dữ liệu nội bộ](implementation_plan/02_chuan_du_lieu_noi_bo.md)
- [03 - Milestone và thứ tự triển khai](implementation_plan/03_milestone_trien_khai.md)
- [04 - Backlog ưu tiên](implementation_plan/04_backlog_uu_tien.md)
- [05 - Rủi ro và tiêu chí MVP](implementation_plan/05_rui_ro_va_tieu_chi_mvp.md)
- [06 - Cấu trúc thư mục đề xuất](implementation_plan/06_cau_truc_thu_muc.md)
- [07 - Quyết định kỹ thuật đã chốt](implementation_plan/07_quyet_dinh_ky_thuat.md)
- [08 - Chuẩn chia task cho AI coding](implementation_plan/08_chuan_chia_task_ai_coding.md)

## Task theo module

- [Module 01 - Project Foundation](implementation_plan/module_01_project_foundation.md)
- [Module 02 - Local Storage](implementation_plan/module_02_local_storage.md)
- [Module 03 - Session](implementation_plan/module_03_session.md)
- [Module 04 - Product & Inventory](implementation_plan/module_04_product_inventory.md)
- [Module 05 - Event Ingestion](implementation_plan/module_05_event_ingestion.md)
- [Module 06 - Node TikTok Connector Sidecar](implementation_plan/module_06_node_tiktok_connector.md)
- [Module 07 - Scrapling Fallback Sidecar](implementation_plan/module_07_scrapling_fallback.md)
- [Module 08 - Connector Supervisor](implementation_plan/module_08_connector_supervisor.md)
- [Module 09 - Manual Capture](implementation_plan/module_09_manual_capture.md)
- [Module 10 - Parser](implementation_plan/module_10_parser.md)
- [Module 11 - Selling Mode Engine](implementation_plan/module_11_selling_mode_engine.md)
- [Module 12 - Order](implementation_plan/module_12_order.md)
- [Module 13 - Customer](implementation_plan/module_13_customer.md)
- [Module 14 - Print & Export](implementation_plan/module_14_print_export.md)
- [Module 15 - UI Layer](implementation_plan/module_15_ui_layer.md)
- [Module 16 - Telemetry & Diagnostics](implementation_plan/module_16_telemetry_diagnostics.md)
- [Module 17 - Thin Cloud Services](implementation_plan/module_17_thin_cloud_services.md)
