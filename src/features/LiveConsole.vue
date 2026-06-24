<template>
  <div class="live-console">
    <div class="main-layout">
      <!-- Left side: Video/Stats Placeholder -->
      <div class="left-panel">
        <div class="glass-panel video-placeholder">
          <div class="status-indicator" :class="connectorStatus">
            <span class="pulse-dot"></span> {{ statusLabel }}
          </div>
          <h2 v-if="activeSession">{{ activeSession.title || activeSession.platformSessionId || 'Phiên Live' }}</h2>
          <h2 v-else>Chưa có phiên live đang chạy</h2>
          <p class="muted" v-if="activeSession">
            @{{ liveUsername }} · {{ activeSession.status }}
          </p>
          <p class="muted" v-else>
            Vào tab "Cấu Hình Live" để tạo và bắt đầu phiên.
          </p>
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
import type { LiveEvent } from '../types/live-event';
import type { LiveSession } from '../types/session';

const events = ref<LiveEvent[]>([]);
const activeSession = ref<LiveSession | null>(null);
const connectorStatus = ref<'connected' | 'connecting' | 'disconnected'>('disconnected');
const chatStreamRef = ref<HTMLElement | null>(null);
const mockComment = ref('');
const unlisteners: Array<() => void> = [];

const liveUsername = computed(() => {
  const raw = activeSession.value?.platformSessionId || '';
  return raw.replace(/^https?:\/\/(www\.)?tiktok\.com\/@/i, '').replace(/^@/, '').split(/[/?]/)[0] || raw;
});

const statusLabel = computed(() => {
  if (!activeSession.value) return 'Chưa kết nối';
  if (connectorStatus.value === 'connected') return 'Đang Phát Trực Tiếp';
  if (connectorStatus.value === 'connecting') return 'Đang kết nối...';
  return 'Mất kết nối';
});

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
  buffer.push(event);
  if (!flushTimeout) {
    flushTimeout = window.setTimeout(flushBuffer, 100);
  }
};

const loadRunningSession = async () => {
  try {
    const sessions = await invoke<LiveSession[]>('get_sessions');
    const running = sessions.find((s) => s.status === 'running') || null;
    activeSession.value = running;

    if (running) {
      connectorStatus.value = 'connecting';
      const history = await invoke<LiveEvent[]>('get_session_events', {
        sessionId: running.id,
        limit: 500,
      });
      events.value = history;
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
    console.error('Lỗi khi tải phiên live:', e);
  }
};

onMounted(async () => {
  await loadRunningSession();

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
    await listen<{ stage?: string; ok?: boolean }>('connector:health', (event) => {
      const stage = event.payload.stage;
      if (stage === 'connected') connectorStatus.value = 'connected';
      else if (stage === 'connecting' || stage === 'reconnecting') connectorStatus.value = 'connecting';
      else if (stage === 'disconnected' || stage === 'stream_ended') connectorStatus.value = 'disconnected';
    })
  );

  unlisteners.push(
    await listen('connector:status', () => {
      connectorStatus.value = 'disconnected';
    })
  );
});

onUnmounted(() => {
  unlisteners.forEach((fn) => fn());
  if (flushTimeout) clearTimeout(flushTimeout);
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
}

.video-placeholder {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  position: relative;
  background: linear-gradient(180deg, rgba(30,41,59,0.8) 0%, rgba(15,23,42,0.9) 100%);
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
