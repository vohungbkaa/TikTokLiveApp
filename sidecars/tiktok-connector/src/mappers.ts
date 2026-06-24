export function mapCommentEvent(data: any) {
    const msgId = data.common?.msgId ?? data.msgId;
    const userId = data.user?.id ?? data.userId;
    const uniqueId = data.user?.displayId ?? data.user?.uniqueId ?? data.uniqueId;
    const displayName = data.user?.nickname ?? data.nickname;
    const comment = data.content ?? data.comment;
    const createTime = data.common?.createTime ?? data.createTime;

    return {
        source: "tiktok",
        platform_event_id: msgId?.toString(),
        user_id: userId?.toString(),
        unique_id: uniqueId?.toString(),
        display_name: displayName?.toString(),
        comment: comment?.toString(),
        ts_platform: createTime
            ? new Date(parseInt(createTime.toString())).toISOString()
            : new Date().toISOString(),
        raw_json: JSON.stringify(data),
    };
}
