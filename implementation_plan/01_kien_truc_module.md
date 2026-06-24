# 01 - Kiến trúc module tổng thể

```text
Desktop App
|
|-- UI Layer (Vue)
|   |-- Live Console
|   |-- Product Session Setup
|   |-- Order Draft Board
|   |-- Review Queue
|   |-- Manual Capture Input
|   |-- Settings
|
|-- App Core (Rust/Tauri)
|   |-- Session Module
|   |-- Event Ingestion Module
|   |-- Event Normalizer Module
|   |-- Parser Module
|   |-- Product & Inventory Module
|   |-- Selling Mode Engine
|   |-- Order Module
|   |-- Customer Module
|   |-- Print/Export Module
|   |-- Connector Supervisor
|   |-- Local Storage Module
|   |-- Telemetry & Diagnostics Module
|
|-- Sidecars
|   |-- Node TikTok Connector Sidecar
|   |-- Python Scrapling Fallback Sidecar
|
|-- Local Database
|   |-- SQLite
|
|-- Thin Cloud Services
    |-- License Service
    |-- Remote Config Service
    |-- Update Manifest Service
    |-- Error/Health Log Service
```

## Luồng xử lý chính

```text
Raw Event
  -> Event Ingestion
  -> Event Normalizer
  -> Parser
  -> Selling Mode Engine
  -> Order Module
  -> UI Update
  -> Print/Export
```

## Nguyên tắc phụ thuộc

- UI chỉ phụ thuộc Rust command/event, không phụ thuộc sidecar.
- Sidecar chỉ phát raw event/health/error, không tạo đơn.
- Parser không truy cập UI.
- Selling Mode Engine không biết event đến từ WebSocket, Scrapling hay Manual.
- Order Module không phụ thuộc TikTok.

