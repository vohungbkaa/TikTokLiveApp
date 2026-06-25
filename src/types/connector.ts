export interface ConnectorSnapshot {
  status: 'idle' | 'connecting' | 'connected' | 'disconnected' | 'down' | 'error';
  stage?: string | null;
  sessionId?: string | null;
  username?: string | null;
  processAlive: boolean;
  stdoutLines?: number;
  eventCount?: number;
  lastMessage?: string | null;
  streamHlsUrl?: string | null;
  streamFlvUrl?: string | null;
  livePageUrl?: string | null;
  streamSource?: string | null;
  streamError?: string | null;
}

export interface StreamInfo {
  hls_url?: string;
  flv_url?: string;
  live_page_url?: string;
  room_id?: string;
  source?: string;
  error?: string;
}

export interface DebugLogEntry {
  ts: string;
  level: string;
  message: string;
}
