# 02 - Chuẩn dữ liệu nội bộ

## LiveEvent

Mọi nguồn comment, gồm WebSocket, Scrapling và Manual Capture, đều phải chuyển về schema này.

```json
{
  "id": "local-generated-id",
  "session_id": "session-id",
  "source": "websocket | scrapling | manual",
  "platform": "tiktok",
  "event_type": "comment | gift | like | system",
  "platform_event_id": "string | null",
  "user_id": "string | null",
  "unique_id": "string | null",
  "display_name": "string | null",
  "avatar_url": "string | null",
  "comment": "string",
  "ts_platform": "datetime | null",
  "ts_received": "datetime",
  "sequence_no": 123,
  "raw_json": {}
}
```

## ParsedIntent

```json
{
  "event_id": "live-event-id",
  "intent": "buy | cancel | question | noise",
  "sku": "A12",
  "variant": {
    "size": "M",
    "color": "đỏ"
  },
  "qty": 1,
  "confidence": 0.92,
  "needs_review": false,
  "reason": "matched_sku_and_buy_keyword"
}
```

## OrderClaim

```json
{
  "id": "claim-id",
  "session_id": "session-id",
  "event_id": "live-event-id",
  "customer_id": "customer-id",
  "product_id": "product-id",
  "variant_id": "variant-id | null",
  "sku": "A12",
  "qty": 1,
  "selling_mode": "stock | unique | preorder",
  "status": "confirmed | won | lost | waitlist | out_of_stock | pending_info | needs_review | cancelled",
  "confidence": 0.92,
  "created_at": "datetime"
}
```

