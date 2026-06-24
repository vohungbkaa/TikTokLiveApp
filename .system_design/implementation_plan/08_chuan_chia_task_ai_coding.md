# 08 - Chuẩn chia task cho AI coding

Mỗi module phải có phần `Task coding chi tiết` để AI hoặc developer có thể lấy từng task nhỏ và triển khai độc lập.

## Format task

```text
Mxx-Tyy - Tên task
- Mục tiêu:
- File/thư mục dự kiến:
- Dependency:
- Output:
- Done khi:
```

Trong đó:

- `Mxx`: số module, ví dụ `M03`.
- `Tyy`: số thứ tự task trong module, ví dụ `T01`.
- `Dependency`: task hoặc module phải có trước.
- `Output`: artifact cụ thể tạo ra sau task.
- `Done khi`: tiêu chí kiểm tra được, tránh mô tả mơ hồ.

## Quy tắc chia task

- Một task chỉ nên sửa một nhóm file liên quan.
- Một task phải có output rõ: model, migration, command, component, service, test.
- Không gộp UI + DB + connector trong cùng một task.
- Task tạo API phải nói rõ input/output.
- Task logic nghiệp vụ phải có test case tối thiểu.
- Task sidecar phải có protocol stdout/stderr rõ.
- Task nào phụ thuộc TikTok thật phải có mock/test mode để dev không bị chặn.

