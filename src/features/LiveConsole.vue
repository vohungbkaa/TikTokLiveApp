<template>
  <div class="live-console">
    <div class="main-layout">
      <!-- Left side: Video/Stats Placeholder -->
      <div class="left-panel">
        <div class="glass-panel video-placeholder">
          <div class="status-indicator" :class="connectorStatus">
            <span class="pulse-dot"></span> {{ statusLabel }}
          </div>

          <div v-if="!activeSession" class="video-empty">
            <h2>Chưa có phiên live đang chạy</h2>
            <p class="muted">Vào tab "Cấu Hình Live" để tạo và bắt đầu phiên.</p>
          </div>

          <LiveViewerPanel
            v-else
            :username="liveUsername"
          />

          <div class="video-meta" v-if="activeSession">
            <h2>{{ activeSession.title || activeSession.platformSessionId || 'Phiên Live' }}</h2>
            <p class="muted">
              @{{ liveUsername }} · {{ activeSession.status }}
              <span v-if="connectorSnapshot">
                · {{ connectorSnapshot.processAlive ? 'process ok' : 'process dead' }}
                · events={{ connectorSnapshot.eventCount ?? 0 }}
              </span>
            </p>
            <button
              v-if="connectorStatus === 'connecting'"
              class="btn-retry"
              @click="retryConnector"
            >
              Thử kết nối lại
            </button>
          </div>
        </div>

        <div v-if="debugLogs.length > 0" class="glass-panel debug-panel">
          <div class="debug-header">Connector Debug Log</div>
          <div class="debug-lines">
            <div v-for="(line, idx) in debugLogs.slice(-12)" :key="idx" class="debug-line">
              <span class="debug-ts">{{ formatLogTime(line.ts) }}</span>
              <span class="debug-level" :class="line.level">{{ line.level }}</span>
              <span class="debug-msg">{{ line.message }}</span>
            </div>
          </div>
        </div>

        <div class="stats-cards">
          <div class="glass-panel stat-card">
            <h3>Tổng Lượt Comment</h3>
            <div class="value">{{ events.length }}</div>
          </div>
          <div class="glass-panel stat-card">
            <h3>Đơn Thành Công</h3>
            <div class="value text-green">0</div>
          </div>
        </div>
      </div>

      <!-- Right side: Chat Stream -->
      <div class="right-panel">
        <div class="glass-panel chat-container">
          <div class="chat-header">
            <h3>Luồng Bình Luận (Realtime)</h3>
            <span class="badge">{{ events.length }} msgs</span>
          </div>
          
          <div class="chat-stream" ref="chatStreamRef">
            <div v-for="event in events" :key="event.id" class="chat-message">
              <div class="avatar">{{ event.displayName?.charAt(0) || 'U' }}</div>
              <div class="message-content">
                <div class="author">
                  <span class="name">{{ event.displayName || event.uniqueId || 'Unknown' }}</span>
                  <span class="time">{{ new Date(event.tsReceived).toLocaleTimeString() }}</span>
                </div>
                <div class="text">{{ event.comment || 'Vừa tham gia phòng live' }}</div>
              </div>
            </div>
            
            <div v-if="events.length === 0" class="empty-chat">
              Chưa có bình luận nào.
            </div>
          </div>
          
          <!-- Mock Ingestion for Testing -->
          <div class="chat-input-mock">
            <input v-model="mockComment" @keyup.enter="sendMockComment" placeholder="Nhập comment giả lập để test..." class="input-field" />
            <button @click="sendMockComment" class="btn-primary">Gửi</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import LiveViewerPanel from '../components/LiveViewerPanel.vue';
import type { LiveEvent } from '../types/live-event';
import type { LiveSession } from '../types/session';
import type { ConnectorSnapshot, DebugLogEntry } from '../types/connector';

const events = ref<LiveEvent[]>([]);
const activeSession = ref<LiveSession | null>(null);
const connectorStatus = ref<'connected' | 'connecting' | 'disconnected'>('disconnected');
const connectorStage = ref<string | null>(null);
const connectorSnapshot = ref<ConnectorSnapshot | null>(null);
const debugLogs = ref<DebugLogEntry[]>([]);
const chatStreamRef = ref<HTMLElement | null>(null);
const mockComment = ref('');
const unlisteners: Array<() => void> = [];
let statusPollTimer: number | null = null;

const liveUsername = computed(() => {
  const raw = activeSession.value?.platformSessionId || '';
  return raw.replace(/^https?:\/\/(www\.)?tiktok\.com\/@/i, '').replace(/^@/, '').split(/[/?]/)[0] || raw;
});

const statusLabel = computed(() => {
  if (!activeSession.value) return 'Chưa kết nối';
  if (connectorStatus.value === 'connected') return 'Đang Phát Trực Tiếp';
  if (connectorStatus.value === 'connecting') {
    return connectorStage.value ? `Đang kết nối (${connectorStage.value})...` : 'Đang kết nối...';
  }
  return 'Mất kết nối';
});

const applyConnectorSnapshot = (snapshot: ConnectorSnapshot) => {
  connectorSnapshot.value = snapshot;
  connectorStage.value = snapshot.stage ?? null;
  if (snapshot.status === 'connected') connectorStatus.value = 'connected';
  else if (snapshot.status === 'connecting' || snapshot.status === 'idle') connectorStatus.value = 'connecting';
  else if (snapshot.status === 'error') connectorStatus.value = 'disconnected';
  else connectorStatus.value = 'disconnected';
};

const formatLogTime = (ts: string) => new Date(ts).toLocaleTimeString('vi-VN');

const appendDebugLog = (entry: DebugLogEntry) => {
  debugLogs.value.push(entry);
  if (debugLogs.value.length > 50) debugLogs.value.shift();
};

const refreshConnectorStatus = async () => {
  if (!activeSession.value) return;
  try {
    const snapshot = await invoke<ConnectorSnapshot>('get_connector_status');
    applyConnectorSnapshot(snapshot);
    if (snapshot.status !== 'connected' && snapshot.status !== 'down') {
      console.log('[LiveConsole] poll status', snapshot);
    }
  } catch (e) {
    console.error('[LiveConsole] poll status failed', e);
  }
};

const startStatusPolling = () => {
  if (statusPollTimer) clearInterval(statusPollTimer);
  statusPollTimer = window.setInterval(async () => {
    if (connectorStatus.value === 'connected') return;
    await refreshConnectorStatus();
  }, 3000);
};

const buffer: LiveEvent[] = [];
let flushTimeout: number | null = null;

const flushBuffer = () => {
  if (buffer.length > 0) {
    events.value.push(...buffer);
    buffer.length = 0;

    nextTick(() => {
      if (chatStreamRef.value) {
        chatStreamRef.value.scrollTop = chatStreamRef.value.scrollHeight;
      }
    });
  }
  flushTimeout = null;
};

const handleNewEvent = (event: LiveEvent) => {
  if (activeSession.value && event.sessionId !== activeSession.value.id) return;
  connectorStatus.value = 'connected';
  connectorStage.value = 'connected';
  buffer.push(event);
  if (!flushTimeout) {
    flushTimeout = window.setTimeout(flushBuffer, 100);
  }
};

const loadRunningSession = async () => {
  try {
    console.log('[LiveConsole] loadRunningSession');
    const sessions = await invoke<LiveSession[]>('get_sessions');
    const running = sessions.find((s) => s.status === 'running') || null;
    activeSession.value = running;

    const logs = await invoke<DebugLogEntry[]>('get_connector_debug_logs');
    debugLogs.value = logs;

    if (running) {
      connectorStatus.value = 'connecting';
      console.log('[LiveConsole] ensure_live_connector', running.id);
      const snapshot = await invoke<ConnectorSnapshot>('ensure_live_connector', {
        sessionId: running.id,
      });
      console.log('[LiveConsole] ensure result', snapshot);
      applyConnectorSnapshot(snapshot);

      const history = await invoke<LiveEvent[]>('get_session_events', {
        sessionId: running.id,
        limit: 500,
      });
      events.value = history;
      if (history.length > 0) {
        connectorStatus.value = 'connected';
        connectorStage.value = 'connected';
      }
      nextTick(() => {
        if (chatStreamRef.value) {
          chatStreamRef.value.scrollTop = chatStreamRef.value.scrollHeight;
        }
      });
    } else {
      events.value = [];
      connectorStatus.value = 'disconnected';
    }
  } catch (e) {
    console.error('[LiveConsole] loadRunningSession failed', e);
    appendDebugLog({
      ts: new Date().toISOString(),
      level: 'error',
      message: `loadRunningSession: ${String(e)}`,
    });
  }
};

const retryConnector = async () => {
  if (!activeSession.value) return;
  connectorStatus.value = 'connecting';
  try {
    const snapshot = await invoke<ConnectorSnapshot>('ensure_live_connector', {
      sessionId: activeSession.value.id,
      force: true,
    });
    applyConnectorSnapshot(snapshot);
  } catch (e) {
    console.error('[LiveConsole] retryConnector failed', e);
    appendDebugLog({
      ts: new Date().toISOString(),
      level: 'error',
      message: `retryConnector: ${String(e)}`,
    });
  }
};

const setupEventListeners = async () => {
  unlisteners.push(
    await listen<LiveEvent>('live_event_received', (event) => {
      handleNewEvent(event.payload);
    })
  );

  unlisteners.push(
    await listen<string>('session:started', async () => {
      await loadRunningSession();
    })
  );

  unlisteners.push(
    await listen<ConnectorSnapshot>('connector:snapshot', (event) => {
      applyConnectorSnapshot(event.payload);
    })
  );

  unlisteners.push(
    await listen<{ stage?: string; ok?: boolean }>('connector:health', (event) => {
      const stage = event.payload.stage;
      connectorStage.value = stage ?? null;
      if (stage === 'connected') connectorStatus.value = 'connected';
      else if (stage === 'connecting' || stage === 'reconnecting' || stage === 'room_resolve') {
        connectorStatus.value = 'connecting';
      } else if (stage === 'disconnected' || stage === 'stream_ended') {
        connectorStatus.value = 'disconnected';
      }
    })
  );

  unlisteners.push(
    await listen<DebugLogEntry>('connector:debug', (event) => {
      appendDebugLog(event.payload);
    })
  );

  unlisteners.push(
    await listen('connector:status', () => {
      connectorStatus.value = 'disconnected';
    })
  );
};

onMounted(async () => {
  await setupEventListeners();
  await loadRunningSession();
  startStatusPolling();
});

onUnmounted(() => {
  unlisteners.forEach((fn) => fn());
  if (flushTimeout) clearTimeout(flushTimeout);
  if (statusPollTimer) clearInterval(statusPollTimer);
});

const sendMockComment = async () => {
  if (!mockComment.value.trim() || !activeSession.value) return;

  try {
    await invoke('test_ingest_event', {
      sessionId: activeSession.value.id,
      comment: mockComment.value,
    });
    mockComment.value = '';
  } catch (e) {
    console.error('Lỗi:', e);
  }
};
</script>

<style scoped>
.live-console {
  padding: 1.5rem;
  height: 100%;
  box-sizing: border-box;
  overflow: hidden;
}

.main-layout {
  display: grid;
  grid-template-columns: 1fr 400px;
  gap: 1.5rem;
  height: 100%;
}

.glass-panel {
  background: var(--surface);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid var(--border);
  border-radius: 20px;
  box-shadow: var(--glass-shadow);
}

.left-panel {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-height: 0;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.left-panel::-webkit-scrollbar {
  width: 6px;
}
.left-panel::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}

.right-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  overflow: hidden;
}

.video-placeholder {
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  gap: 0.75rem;
  position: relative;
  padding: 1rem;
  background: linear-gradient(180deg, rgba(30,41,59,0.8) 0%, rgba(15,23,42,0.9) 100%);
  min-height: 420px;
}

.video-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.video-meta {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.video-meta h2 {
  margin: 0;
  font-size: 1.1rem;
}

.btn-open-live-inline {
  align-self: flex-start;
  background: rgba(59, 130, 246, 0.15);
  color: #93c5fd;
  border: 1px solid rgba(59, 130, 246, 0.35);
  padding: 0.4rem 0.85rem;
  border-radius: 10px;
  font-weight: 600;
  cursor: pointer;
}

.status-indicator {
  position: absolute;
  top: 1.5rem;
  left: 1.5rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: rgba(0,0,0,0.5);
  padding: 0.5rem 1rem;
  border-radius: 99px;
  font-weight: 600;
  font-size: 0.9rem;
}

.pulse-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.status-indicator.connecting .pulse-dot {
  background-color: #f59e0b;
  box-shadow: 0 0 0 0 rgba(245, 158, 11, 1);
  animation: pulse 2s infinite;
}

.status-indicator.disconnected .pulse-dot {
  background-color: #64748b;
  animation: none;
  box-shadow: none;
}

.status-indicator.connected .pulse-dot {
  background-color: #ef4444;
  box-shadow: 0 0 0 0 rgba(239, 68, 68, 1);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.7); }
  70% { transform: scale(1); box-shadow: 0 0 0 10px rgba(239, 68, 68, 0); }
  100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(239, 68, 68, 0); }
}

.stats-cards {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.debug-panel {
  padding: 0.75rem 1rem;
  max-height: 160px;
  overflow: hidden;
}

.debug-header {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-muted);
  margin-bottom: 0.5rem;
  text-transform: uppercase;
}

.debug-lines {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 0.72rem;
  max-height: 120px;
  overflow-y: auto;
}

.debug-line {
  display: flex;
  gap: 0.5rem;
  padding: 0.15rem 0;
  border-bottom: 1px solid rgba(255,255,255,0.04);
}

.debug-ts { color: #64748b; flex-shrink: 0; }
.debug-level { flex-shrink: 0; width: 42px; text-transform: uppercase; }
.debug-level.info { color: #60a5fa; }
.debug-level.warn { color: #fbbf24; }
.debug-level.error { color: #f87171; }
.debug-msg { color: #cbd5e1; word-break: break-word; }

.btn-retry {
  margin-top: 0.75rem;
  background: rgba(245, 158, 11, 0.15);
  color: #fbbf24;
  border: 1px solid rgba(245, 158, 11, 0.35);
  padding: 0.45rem 1rem;
  border-radius: 10px;
  font-weight: 600;
  cursor: pointer;
}

.btn-retry:hover {
  background: rgba(245, 158, 11, 0.25);
}

.stat-card {
  padding: 1.5rem;
  text-align: center;
}

.stat-card h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-muted);
  font-size: 0.95rem;
  text-transform: uppercase;
}

.stat-card .value {
  font-size: 2.5rem;
  font-weight: 800;
}

.text-green {
  color: #4ade80;
}

.chat-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.chat-header {
  padding: 1.2rem;
  border-bottom: 1px solid var(--border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.chat-header h3 {
  margin: 0;
  font-size: 1.1rem;
}

.badge {
  background: rgba(59, 130, 246, 0.2);
  color: var(--primary);
  padding: 0.2rem 0.6rem;
  border-radius: 99px;
  font-size: 0.8rem;
  font-weight: 600;
}

.chat-stream {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.chat-stream::-webkit-scrollbar {
  width: 6px;
}

.chat-stream::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 10px;
}

.chat-message {
  display: flex;
  gap: 0.8rem;
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.avatar {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, #c084fc, #3b82f6);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  color: white;
  flex-shrink: 0;
}

.message-content {
  background: rgba(15, 23, 42, 0.4);
  padding: 0.8rem 1rem;
  border-radius: 0 16px 16px 16px;
  border: 1px solid var(--border);
}

.author {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.3rem;
  gap: 1rem;
}

.name {
  font-weight: 600;
  font-size: 0.9rem;
  color: #c084fc;
}

.time {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.text {
  font-size: 0.95rem;
  line-height: 1.4;
  word-break: break-word;
}

.empty-chat {
  text-align: center;
  color: var(--text-muted);
  font-style: italic;
  margin-top: 2rem;
}

.chat-input-mock {
  padding: 1rem;
  border-top: 1px solid var(--border);
  display: flex;
  gap: 0.5rem;
}

.input-field {
  flex: 1;
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 0.8rem 1rem;
  color: var(--text-main);
  font-size: 1rem;
  outline: none;
}

.input-field:focus {
  border-color: var(--primary);
}

.btn-primary {
  background: var(--primary);
  color: white;
  border: none;
  padding: 0 1.5rem;
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover {
  background: var(--primary-hover);
}
</style>
