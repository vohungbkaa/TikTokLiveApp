# 06 - Cấu trúc thư mục đề xuất

```text
.
|-- docs/
|   |-- TikTok_Live_Order_App.md
|   |-- Implementation_Plan.md
|
|-- src/
|   |-- app/
|   |-- components/
|   |-- features/
|   |   |-- live-console/
|   |   |-- session-setup/
|   |   |-- products/
|   |   |-- orders/
|   |   |-- settings/
|   |-- stores/
|   |-- types/
|
|-- src-tauri/
|   |-- src/
|   |   |-- main.rs
|   |   |-- commands/
|   |   |-- db/
|   |   |-- sessions/
|   |   |-- events/
|   |   |-- parser/
|   |   |-- products/
|   |   |-- inventory/
|   |   |-- selling_modes/
|   |   |-- orders/
|   |   |-- customers/
|   |   |-- connectors/
|   |   |-- diagnostics/
|   |   |-- export/
|   |-- migrations/
|
|-- sidecars/
|   |-- tiktok-connector/
|   |-- scrapling-fallback/
|
|-- scripts/
|   |-- build-sidecars/
|   |-- package-app/
|
|-- implementation_plan/
|   |-- module_01_project_foundation.md
|   |-- module_02_local_storage.md
|   |-- ...
```

