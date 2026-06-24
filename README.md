# TikTokLiveApp

Dự án ứng dụng TikTok Live Desktop Client được xây dựng bằng **Tauri v2**, **Vue 3**, **Vite** và **TypeScript**.

## Yêu cầu hệ thống (Prerequisites)

Trước khi bắt đầu, hãy đảm bảo máy tính của bạn đã cài đặt:
1. **Node.js** (Khuyến nghị bản LTS mới nhất)
2. **pnpm** (Trình quản lý package chính của dự án)
3. **Rust & Cargo** (Xem hướng dẫn cài đặt tại [rustup.rs](https://rustup.rs/))
4. **Tauri Prerequisites**: Cần thiết để biên dịch phần backend Rust. Xem chi tiết tại [Tauri Setup Guide](https://tauri.app/start/prerequisites/).

---

## Hướng dẫn cài đặt và khởi chạy (How to Run)

### 1. Di chuyển vào thư mục dự án
```bash
cd TikTokLiveApp
```

### 2. Cài đặt các thư viện Node.js
```bash
pnpm install
```

### 3. Chạy dự án trong môi trường phát triển (Development)
Khởi động Vite dev server và khởi chạy ứng dụng desktop:
```bash
pnpm tauri dev
```

### 4. Build ứng dụng cho môi trường sản xuất (Production Build)
Để đóng gói ứng dụng desktop thành bản cài đặt hoàn chỉnh:
```bash
pnpm tauri build
```

---

## Khuyên dùng cho môi trường phát triển (Recommended IDE Setup)

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
