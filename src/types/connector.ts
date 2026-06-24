export interface ConnectorSnapshot {
  status: 'idle' | 'connecting' | 'connected' | 'disconnected' | 'down' | 'error';
  stage?: string | null;
  sessionId?: string | null;
  username?: string | null;
  processAlive: boolean;
  stdoutLines?: number;
  eventCount?: number;
  lastMessage?: string | null;
}

export interface DebugLogEntry {
  ts: string;
  level: string;
  message: string;
}
