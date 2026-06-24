export function mapCommentEvent(data: any) {
    return {
        source: "tiktok",
        platform_event_id: data.msgId,
        user_id: data.userId?.toString(),
        unique_id: data.uniqueId,
        display_name: data.nickname,
        comment: data.comment,
        ts_platform: data.createTime ? new Date(parseInt(data.createTime)).toISOString() : new Date().toISOString(),
        raw_json: JSON.stringify(data)
    };
}
