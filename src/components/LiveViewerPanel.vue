<template>
  <div class="live-viewer-panel">
    <div class="viewer-toggle-row">
      <label class="toggle-label">
        <input type="checkbox" :checked="prefs.watchLiveEnabled" @change="onToggleWatch" />
        <span>Xem live TikTok</span>
      </label>
      <label v-if="prefs.watchLiveEnabled" class="toggle-label toggle-label--sub">
        <input type="checkbox" v-model="prefs.autoOpenOnSessionStart" />
        <span>Tự mở khi bắt đầu phiên</span>
      </label>
    </div>

    <div v-if="!prefs.watchLiveEnabled" class="viewer-off">
      <p>Chỉ dùng luồng comment để chốt đơn. Bật tuỳ chọn phía trên nếu muốn xem hình live.</p>
    </div>

    <div v-else class="viewer-on">
      <div class="viewer-preview">
        <div class="preview-icon">📺</div>
        <p class="preview-title">Xem live trong cửa sổ TikTok</p>
        <p class="preview-desc">
          Có thể đăng nhập bằng <strong>bất kỳ tài khoản TikTok</strong> (không cần trùng tài khoản bán hàng).
          Sau khi đăng nhập, app mở thẳng phiên live <strong>@{{ username }}</strong>.
        </p>
      </div>

      <div class="viewer-actions">
        <button class="btn-primary" type="button" :disabled="!username" @click="loginAndWatch">
          Đăng nhập & xem live
        </button>
        <button class="btn-secondary" type="button" :disabled="!username" @click="watchDirect">
          Xem live (đã đăng nhập)
        </button>
        <button class="btn-ghost" type="button" @click="closeViewer">Đóng cửa sổ live</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { useLiveViewerPrefs } from '../composables/useLiveViewerPrefs';

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
  gap: 0.85rem;
  width: 100%;
}

.viewer-toggle-row {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem 1.25rem;
}

.toggle-label {
  display: inline-flex;
  align-items: center;
  gap: 0.45rem;
  font-size: 0.92rem;
  font-weight: 600;
  cursor: pointer;
  user-select: none;
}

.toggle-label--sub {
  font-weight: 500;
  color: var(--text-muted);
}

.viewer-off,
.viewer-on {
  border: 1px dashed rgba(148, 163, 184, 0.25);
  border-radius: 14px;
  padding: 1rem;
}

.viewer-off p {
  margin: 0;
  color: var(--text-muted);
  font-size: 0.9rem;
  line-height: 1.5;
}

.viewer-preview {
  text-align: center;
  margin-bottom: 0.85rem;
}

.preview-icon {
  font-size: 2rem;
  margin-bottom: 0.35rem;
}

.preview-title {
  margin: 0 0 0.35rem;
  font-weight: 700;
}

.preview-desc {
  margin: 0;
  color: var(--text-muted);
  font-size: 0.88rem;
  line-height: 1.45;
}

.viewer-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.btn-primary,
.btn-secondary,
.btn-ghost {
  border-radius: 10px;
  padding: 0.55rem 0.95rem;
  font-weight: 600;
  cursor: pointer;
  border: 1px solid transparent;
}

.btn-primary {
  background: var(--primary);
  color: #fff;
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: rgba(59, 130, 246, 0.15);
  color: #93c5fd;
  border-color: rgba(59, 130, 246, 0.35);
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-ghost {
  background: transparent;
  color: var(--text-muted);
  border-color: var(--border);
}
</style>
