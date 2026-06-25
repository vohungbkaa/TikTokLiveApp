import { RouteConfig } from 'tiktok-live-connector';

export interface StreamInfo {
    hls_url?: string;
    flv_url?: string;
    live_page_url: string;
    room_id?: string;
    source: string;
    error?: string;
}

function pickFlvUrl(flv: unknown): string | undefined {
    if (!flv) return undefined;
    if (typeof flv === 'string' && flv.startsWith('http')) return flv;
    if (typeof flv === 'object' && flv !== null) {
        const map = flv as Record<string, string>;
        for (const key of ['FULL_HD1', 'HD1', 'SD1', 'LD1']) {
            if (map[key]?.startsWith('http')) return map[key];
        }
        for (const value of Object.values(map)) {
            if (typeof value === 'string' && value.startsWith('http')) return value;
        }
    }
    return undefined;
}

function extractFromRoomInfo(roomInfo: unknown): { hls?: string; flv?: string } {
    if (!roomInfo || typeof roomInfo !== 'object') return {};
    const info = roomInfo as Record<string, unknown>;
    const data = (info.data ?? info) as Record<string, unknown>;
    const streamUrl = data.stream_url as Record<string, unknown> | undefined;
    if (!streamUrl) return {};

    const hls = streamUrl.hls_pull_url;
    const hlsUrl = typeof hls === 'string' && hls.length > 0 ? hls : undefined;
    const flvUrl = pickFlvUrl(streamUrl.flv_pull_url) ?? pickFlvUrl(streamUrl.rtmp_pull_url);

    return { hls: hlsUrl, flv: flvUrl };
}

function findUrlDeep(obj: unknown, key: string): string | undefined {
    if (!obj || typeof obj !== 'object') return undefined;
    const record = obj as Record<string, unknown>;
    const direct = record[key];
    if (typeof direct === 'string' && direct.startsWith('http')) return direct;
    for (const value of Object.values(record)) {
        const found = findUrlDeep(value, key);
        if (found) return found;
    }
    return undefined;
}

export async function resolveStreamInfo(
    connection: {
        roomId?: string;
        roomInfo?: unknown;
        fetchRoomInfo: () => Promise<unknown>;
        webClient: unknown;
    },
    username: string,
): Promise<StreamInfo> {
    const cleanUser = username.replace(/^@/, '').split(/[/?]/)[0];
    const livePageUrl = `https://www.tiktok.com/@${cleanUser}/live`;
    const roomId = connection.roomId;

    const fromConnect = extractFromRoomInfo(connection.roomInfo);
    if (fromConnect.hls || fromConnect.flv) {
        return {
            hls_url: fromConnect.hls,
            flv_url: fromConnect.flv,
            live_page_url: livePageUrl,
            room_id: roomId,
            source: 'connect_room_info',
        };
    }

    try {
        const fetched = await connection.fetchRoomInfo();
        const fromFetch = extractFromRoomInfo(fetched);
        if (fromFetch.hls || fromFetch.flv) {
            return {
                hls_url: fromFetch.hls,
                flv_url: fromFetch.flv,
                live_page_url: livePageUrl,
                room_id: roomId,
                source: 'fetch_room_info',
            };
        }
    } catch {
        // continue to HTML fallback
    }

    try {
        const sigi = await RouteConfig.fetchRoomInfoFromHtml({
            webClient: connection.webClient as never,
            uniqueId: cleanUser,
        });
        const hls = findUrlDeep(sigi, 'hls_pull_url');
        const flv = findUrlDeep(sigi, 'flv_pull_url') ?? findUrlDeep(sigi, 'rtmp_pull_url');
        if (hls || flv) {
            return {
                hls_url: hls,
                flv_url: flv,
                live_page_url: livePageUrl,
                room_id: roomId,
                source: 'html_scrape',
            };
        }
    } catch {
        // stream URL unavailable without session cookie
    }

    return {
        live_page_url: livePageUrl,
        room_id: roomId,
        source: 'unavailable',
        error: 'TikTok chưa trả link stream (cần cookie đăng nhập hoặc mở cửa sổ live)',
    };
}
