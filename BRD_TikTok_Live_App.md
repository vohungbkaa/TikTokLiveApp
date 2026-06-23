# Tài Liệu Phân Tích Yêu Cầu & Giải Pháp (BRD) - TikTok Live Order App

## 1. Tổng quan dự án
Ứng dụng Desktop hỗ trợ tự động hóa quá trình chốt đơn, quản lý bình luận và tương tác với khách hàng trong các phiên Livestream trên nền tảng TikTok. Sản phẩm nhắm đến việc tối ưu hiệu suất bán hàng và loại bỏ các thao tác thủ công dễ sai sót.

## 2. Đối tượng khách hàng mục tiêu (Target Audience)
* Nhà bán hàng (Seller) cá nhân hoặc hộ kinh doanh đang livestream trên TikTok.
* Người bán "Hàng Si" (đồ secondhand), đồ thiết kế độc bản (mỗi mẫu chỉ có 1 chiếc duy nhất).
* Các chủ shop muốn tối ưu chi phí, tự quản lý đơn hàng trực tiếp và không muốn phụ thuộc hoàn toàn vào hệ thống TikTok Shop (tránh phí sàn cao).

## 3. Phân tích Nỗi đau khách hàng & Giải pháp phần mềm

| Vấn đề/Nỗi đau của Seller (Pain Points) | Giải pháp của Phần mềm (Software Features) |
| :--- | :--- |
| **Sót đơn, nhầm lẫn khách hàng:** Bình luận trôi quá nhanh, nhiều khách hàng trùng tên hiển thị (Nickname) dẫn đến gộp nhầm đơn, lộn mã. | **Cơ chế Định danh Tuyệt đối (Identity Mapping):** Sử dụng `userId` hoặc `@uniqueId` ẩn dưới payload thay vì Nickname. Nhận diện và gộp đơn chính xác 100% người dùng dù họ trùng tên hay đổi avatar. |
| **Tranh giành hàng Độc Bản:** Bán một cái áo si, 10 người cùng comment chốt. Phân xử bằng mắt thường sẽ gây tranh cãi. | **Thuật toán "Ai nhanh hơn" (First-come, First-served Lock):** App quét realtime, khóa mã sản phẩm cho người comment đúng cú pháp ĐẦU TIÊN với độ trễ tính bằng mili-giây. Hiển thị to, minh bạch tên người chiến thắng. |
| **Khách bùng hàng, quên địa chỉ:** Khách chốt xong rời live, gọi điện không nghe máy, không có thông tin giao hàng. | **Auto-DM (Nhắn tin chốt chặn):** Tự động gửi tin nhắn (Direct Message) cho khách ngay khi chốt đơn thành công để xin số điện thoại và địa chỉ, "khóa" tâm lý mua hàng lập tức. |
| **Boom hàng từ khách cũ:** Khách hàng có lịch sử "boom" hàng đi chốt đơn dạo phá rối. | **Bộ lọc Blacklist (Danh sách đen):** Tự động đối chiếu SĐT/ID với Danh sách đen của shop hoặc cộng đồng. Báo đỏ và tự động bỏ qua comment chốt đơn của "bom thủ". |
| **Xử lý hậu cần chậm trễ:** Viết tay hàng trăm đơn hàng sau mỗi ca live gây đau mỏi, tốn thời gian và dễ sai sót. | **In đơn thần tốc (1-Click Thermal Print):** Tích hợp driver máy in nhiệt nội bộ qua Desktop App. In ngay lập tức nhãn dán bưu kiện khi đơn nháp đủ thông tin chỉ với 1 click. |
| **Giảm tương tác (Tụt mắt xem):** Phiên live nhàm chán, khó giữ chân người xem ở lại lâu. | **Hệ sinh thái Mini-game:** Tích hợp vòng quay may mắn, bốc thăm theo comment (1-99) tự động, vẽ biểu đồ bình chọn realtime hiển thị trực tiếp trên màn hình live. |

## 4. Giải pháp Onboarding & Kết nối Livestream (Fast Setup)

Để giảm thiểu rào cản công nghệ cho người bán hàng (thường không rành IT), hệ thống được thiết kế theo triết lý "Plug & Play" (Cắm là chạy):

*   **Cài đặt phần mềm nhanh chóng:**
    *   **1-Click Install:** Ứng dụng đóng gói dưới dạng Portable hoặc Installer duy nhất, không yêu cầu cài đặt thêm các thư viện phức tạp (Node.js, Python...) vì mọi module lõi (Sidecar) đã được đóng gói ngầm.
    *   **Giao diện trực quan (Clean UI):** Lần đầu mở App, người dùng chỉ thấy một màn hình tinh gọn tập trung vào việc kết nối Live, ẩn đi các cấu hình kỹ thuật rối rắm.

*   **Kết nối luồng Livestream đơn giản nhất:**
    *   **Không yêu cầu Đăng nhập (No Login Required):** Tuyệt đối không yêu cầu người dùng nhập mật khẩu TikTok hay quét mã QR (giúp loại bỏ hoàn toàn rủi ro lộ tài khoản, mất kênh hay bị khóa Checkpoint).
    *   **Chỉ cần nhập @Username hoặc Link:** Khách hàng chỉ việc nhập `@username` (ID TikTok) của kênh đang phát Live, hoặc dán đường link chia sẻ của phiên Live. Hệ thống sẽ tự động quét, trích xuất `roomId` và thiết lập luồng kết nối ngay lập tức.
    *   **Lưu lịch sử kênh (1-Click Reconnect):** Tự động lưu lại danh sách các kênh đã từng kết nối. Trong những ca live tiếp theo, Seller chỉ cần mở App và bấm 1 nút duy nhất để bắt đầu đọc comment.
    *   **Chẩn đoán trạng thái rõ ràng:** Có đèn tín hiệu trực quan (Xanh lá: Đang bắt comment mượt mà, Đỏ: Mất kết nối/Kênh chưa live). Nếu có lỗi, thông báo hiển thị bằng tiếng Việt phổ thông, kèm nút bấm "Thử lại".

## 5. Yêu cầu Phi chức năng & Kiến trúc Kỹ thuật (Non-functional Requirements)

Để đảm bảo hệ thống hoạt động mượt mà, "bất tử" trước các đợt cập nhật chống Bot của TikTok, kiến trúc hệ thống được thiết kế tinh gọn nhưng cực kỳ mạnh mẽ:

* **Công nghệ cốt lõi:**
  * **Framework vỏ bọc:** `Tauri` (Đảm bảo App siêu nhẹ dưới 20MB, tốn rất ít CPU/RAM để không làm giật lag máy tính đang mở OBS Live của Seller).
  * **Frontend UI:** `Vue.js 3` (Hiệu năng cao để render hàng trăm comment/giây mượt mà trên Dashboard).
  * **Backend Core:** `Rust` (Xử lý đa luồng ưu việt, giao tiếp với Database SQLite nội bộ với tốc độ cao).

* **Kiến trúc Thu thập dữ liệu (Dual-Engine Extraction):**
  * **Primary Engine (90% Uptime):** Dùng cơ chế Tauri Sidecar gọi module `Node.js` (`TikTok-Live-Connector`) kết nối luồng WebSocket trực tiếp. Tốc độ Real-time, tốn cực ít băng thông.
  * **Fallback Engine (Smart Failover):** Dùng Sidecar `Python` (`Scrapling`). Nếu API WebSocket bị TikTok đổi giao thức, App tự động kích hoạt trình duyệt ẩn (Headless Browser) quét DOM HTML. Đảm bảo phiên live của khách hàng không bao giờ bị đứt gãy.
  * **Data Enrichment (Làm giàu dữ liệu):** Khi hệ thống phục hồi từ Fallback (cào Web) về Primary (WebSocket), App tự động nối phiên (Session Sync) và điền khuyết các ID khách hàng bị thiếu.

* **Cơ chế Triển khai & Bảo trì:**
  * **Dynamic Schema:** App tự động kéo file cấu hình giải mã (Protobuf Schema) mới nhất từ Server trung tâm mỗi lần khởi động.
  * **Silent Auto-Update:** Cơ chế tự động tải và cập nhật ngầm của Tauri giúp vá lỗi API nhanh chóng mà không bắt người dùng phải thao tác cài đặt lại.

## 6. Lộ trình phát triển (Phases Roadmap)
1. **Phase 1 (MVP - Sản phẩm khả dụng tối thiểu):** Hoàn thiện bộ khung Tauri + Vue + Rust. Nhúng Sidecar WebSocket làm tính năng cốt lõi: Bắt comment, bóc tách cú pháp, định danh khách hàng, lập đơn nháp và in đơn nhiệt.
2. **Phase 2 (Automation & Failover):** Tích hợp Sidecar Python (Scrapling). Chạy tính năng Auto-DM để chủ động nhắn tin khách hàng. Bật hệ thống Smart Failover chuyển đổi linh hoạt giữa WebSocket và Web DOM.
3. **Phase 3 (Ecosystem & Cloud):** Thêm các Mini-game tương tác. Đồng bộ dữ liệu hóa đơn lên hệ thống Cloud Server để Seller xem báo cáo qua Web. Tích hợp API tạo đơn tự động với GHTK/GHN/ViettelPost.
