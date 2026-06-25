<template>
  <div class="live-viewer-panel">
    <div class="viewer-toggle-row">
      <label class="switch-label">
        <div class="switch">
          <input type="checkbox" :checked="prefs.watchLiveEnabled" @change="onToggleWatch" />
          <span class="slider round"></span>
        </div>
        <span class="switch-text">Bật xem hình Live</span>
      </label>
    </div>

    <div v-if="!prefs.watchLiveEnabled" class="state-off">
      <div class="state-icon-bg">
        <MonitorPlay :size="32" class="icon-muted" />
      </div>
      <p class="state-desc">Đang chạy ở chế độ <strong>tiết kiệm tài nguyên</strong>. <br/>Chỉ hút comment chốt đơn, không tải video.</p>
    </div>

    <div v-else class="state-on">
      <label class="checkbox-option">
        <input type="checkbox" v-model="prefs.autoOpenOnSessionStart" class="custom-checkbox" />
        <span>Tự động bật cửa sổ khi tạo phiên mới</span>
      </label>

      <div class="info-card">
        <p class="info-desc">
          Bạn có thể đăng nhập bằng <strong>bất kỳ tài khoản TikTok nào</strong> để xem.
          Hệ thống sẽ tự động điều hướng tới luồng live của <strong>@{{ username }}</strong>.
        </p>
      </div>

      <div class="action-grid">
        <button class="btn-action primary" :disabled="!username" @click="loginAndWatch">
          <LogIn :size="18" /> Đăng nhập & Xem
        </button>
        <button class="btn-action secondary" :disabled="!username" @click="watchDirect">
          <ExternalLink :size="18" /> Xem ngay (Đã đăng nhập)
        </button>
        <button class="btn-action danger-ghost" @click="closeViewer">
          <XCircle :size="18" /> Đóng cửa sổ live
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { useLiveViewerPrefs } from '../composables/useLiveViewerPrefs';
import { MonitorPlay, LogIn, ExternalLink, XCircle } from 'lucide-vue-next';

const props = defineProps<{
  username: string;
}>();

const emit = defineEmits<{
  (e: 'watch-opened'): void;
}>();

const { prefs, setWatchLiveEnabled } = useLiveViewerPrefs();

const openViewer = async (loginFirst: boolean) => {
  if (!props.username) return;
  try {
    await invoke('open_live_viewer', {
      username: props.username,
      loginFirst,
    });
    emit('watch-opened');
  } catch (e) {
    console.error('[LiveViewerPanel] open_live_viewer failed', e);
  }
};

const loginAndWatch = () => openViewer(true);
const watchDirect = () => openViewer(false);

const onToggleWatch = async (event: Event) => {
  const enabled = (event.target as HTMLInputElement).checked;
  setWatchLiveEnabled(enabled);
  if (!enabled) {
    await closeViewer();
  }
};

const closeViewer = async () => {
  try {
    await invoke('close_live_viewer');
  } catch (e) {
    console.error('[LiveViewerPanel] close_live_viewer failed', e);
  }
};

defineExpose({ loginAndWatch, watchDirect, closeViewer });
</script>

<style scoped>
.live-viewer-panel {
  display: flex;
  flex-direction: column;
  flex: 1;
  padding: 1rem;
  padding-top: 1.5rem;
}

.viewer-toggle-row {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 2rem;
  padding-right: 0.5rem;
}

.switch-label {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
  user-select: none;
}

.switch-text {
  font-weight: 600;
  font-size: 0.95rem;
  color: #e2e8f0;
}

/* Custom Switch */
.switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
}
.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}
.slider {
  position: absolute;
  cursor: pointer;
  top: 0; left: 0; right: 0; bottom: 0;
  background-color: rgba(148, 163, 184, 0.3);
  transition: .3s;
}
.slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
}
.slider.round {
  border-radius: 24px;
}
.slider.round:before {
  border-radius: 50%;
}
input:checked + .slider {
  background-color: #3b82f6;
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
}
input:checked + .slider:before {
  transform: translateX(20px);
}

.state-off {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  gap: 1.25rem;
}

.state-icon-bg {
  width: 72px;
  height: 72px;
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: inset 0 2px 10px rgba(0,0,0,0.3);
}

.icon-muted {
  color: #475569;
}

.state-desc {
  color: #94a3b8;
  font-size: 0.95rem;
  line-height: 1.6;
  max-width: 280px;
  margin: 0;
}

.state-desc strong {
  color: #e2e8f0;
}

.state-on {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  flex: 1;
}

.checkbox-option {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
  font-size: 0.92rem;
  color: #cbd5e1;
  padding: 0.85rem 1rem;
  background: rgba(15, 23, 42, 0.4);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 12px;
  transition: all 0.2s;
}

.checkbox-option:hover {
  background: rgba(255, 255, 255, 0.08);
  border-color: rgba(255, 255, 255, 0.1);
}

.custom-checkbox {
  width: 18px;
  height: 18px;
  accent-color: #3b82f6;
  cursor: pointer;
}

.info-card {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.1) 0%, rgba(37, 99, 235, 0.05) 100%);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 12px;
  padding: 1.15rem;
  border-left: 4px solid #3b82f6;
}

.info-desc {
  margin: 0;
  color: #93c5fd;
  font-size: 0.9rem;
  line-height: 1.5;
}

.info-desc strong {
  color: #bfdbfe;
}

.action-grid {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  padding-bottom: 0.5rem;
}

.btn-action {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.6rem;
  padding: 0.85rem;
  border-radius: 12px;
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  border: 1px solid transparent;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.btn-action:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-action.primary {
  background: linear-gradient(135deg, #3b82f6, #2563eb);
  color: white;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.25);
  border: 1px solid rgba(96, 165, 250, 0.3);
}

.btn-action.primary:not(:disabled):hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
  background: linear-gradient(135deg, #60a5fa, #3b82f6);
}

.btn-action.secondary {
  background: rgba(59, 130, 246, 0.1);
  color: #93c5fd;
  border-color: rgba(59, 130, 246, 0.25);
}

.btn-action.secondary:not(:disabled):hover {
  background: rgba(59, 130, 246, 0.18);
  border-color: rgba(59, 130, 246, 0.4);
}

.btn-action.danger-ghost {
  background: transparent;
  color: #f87171;
  border-color: rgba(248, 113, 113, 0.2);
  margin-top: 0.5rem;
}

.btn-action.danger-ghost:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.4);
  color: #fca5a5;
}
</style>
