<template>
  <div class="live-stream-player">
    <video
      ref="videoRef"
      class="stream-video"
      controls
      autoplay
      playsinline
      muted
    />

    <div v-if="playerState === 'loading'" class="stream-overlay">
      <span class="overlay-text">Đang tải video live...</span>
    </div>

    <div v-else-if="playerState === 'unavailable'" class="stream-overlay">
      <span class="overlay-text">{{ streamError || 'Không lấy được link phát trực tiếp từ TikTok.' }}</span>
      <p class="overlay-hint">Comment vẫn hoạt động. Mở cửa sổ TikTok để xem hình live.</p>
      <button class="btn-open-live" type="button" @click="openLiveWindow">Mở cửa sổ live TikTok</button>
    </div>

    <div v-else-if="playerState === 'error'" class="stream-overlay stream-overlay--error">
      <span class="overlay-text">{{ playerError }}</span>
      <button class="btn-open-live" type="button" @click="openLiveWindow">Mở cửa sổ live TikTok</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import Hls from 'hls.js';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  username: string;
  hlsUrl?: string | null;
  livePageUrl?: string | null;
  streamError?: string | null;
}>();

const videoRef = ref<HTMLVideoElement | null>(null);
const playerState = ref<'loading' | 'playing' | 'unavailable' | 'error'>('loading');
const playerError = ref('');
let hls: Hls | null = null;

const destroyPlayer = () => {
  if (hls) {
    hls.destroy();
    hls = null;
  }
  if (videoRef.value) {
    videoRef.value.removeAttribute('src');
    videoRef.value.load();
  }
};

const attachStream = async (url: string) => {
  const video = videoRef.value;
  if (!video) return;

  destroyPlayer();
  playerState.value = 'loading';
  playerError.value = '';

  if (Hls.isSupported()) {
    hls = new Hls({
      enableWorker: true,
      lowLatencyMode: true,
    });
    hls.loadSource(url);
    hls.attachMedia(video);
    hls.on(Hls.Events.MANIFEST_PARSED, () => {
      playerState.value = 'playing';
      video.play().catch(() => {
        playerState.value = 'playing';
      });
    });
    hls.on(Hls.Events.ERROR, (_event, data) => {
      if (!data.fatal) return;
      playerState.value = 'error';
      playerError.value = 'Không phát được HLS stream.';
      destroyPlayer();
    });
    return;
  }

  if (video.canPlayType('application/vnd.apple.mpegurl')) {
    video.src = url;
    video.addEventListener(
      'loadedmetadata',
      () => {
        playerState.value = 'playing';
        video.play().catch(() => {
          playerState.value = 'playing';
        });
      },
      { once: true },
    );
    video.addEventListener(
      'error',
      () => {
        playerState.value = 'error';
        playerError.value = 'Trình duyệt không phát được stream này.';
      },
      { once: true },
    );
    return;
  }

  playerState.value = 'error';
  playerError.value = 'Trình duyệt không hỗ trợ HLS.';
};

const syncFromProps = () => {
  if (props.hlsUrl) {
    attachStream(props.hlsUrl);
    return;
  }
  destroyPlayer();
  playerState.value = 'unavailable';
};

watch(
  () => props.hlsUrl,
  () => syncFromProps(),
);

onMounted(() => syncFromProps());
onUnmounted(() => destroyPlayer());

const openLiveWindow = async () => {
  try {
    await invoke('open_live_viewer', { username: props.username });
  } catch (e) {
    const fallback = props.livePageUrl || `https://www.tiktok.com/@${props.username.replace(/^@/, '')}/live`;
    window.open(fallback, '_blank');
    console.error('[LiveStreamPlayer] open_live_viewer failed', e);
  }
};
</script>

<style scoped>
.live-stream-player {
  position: relative;
  width: 100%;
  flex: 1;
  min-height: 280px;
  border-radius: 16px;
  overflow: hidden;
  background: #000;
}

.stream-video {
  width: 100%;
  height: 100%;
  min-height: 280px;
  object-fit: contain;
  background: #000;
}

.stream-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding: 1.5rem;
  text-align: center;
  background: linear-gradient(180deg, rgba(15, 23, 42, 0.35) 0%, rgba(15, 23, 42, 0.85) 100%);
}

.stream-overlay--error {
  background: rgba(69, 10, 10, 0.75);
}

.overlay-text {
  color: #e2e8f0;
  font-weight: 600;
}

.overlay-hint {
  margin: 0;
  color: #94a3b8;
  font-size: 0.9rem;
  max-width: 320px;
}

.btn-open-live {
  margin-top: 0.25rem;
  background: rgba(59, 130, 246, 0.2);
  color: #93c5fd;
  border: 1px solid rgba(59, 130, 246, 0.45);
  padding: 0.55rem 1rem;
  border-radius: 10px;
  font-weight: 600;
  cursor: pointer;
}

.btn-open-live:hover {
  background: rgba(59, 130, 246, 0.32);
}
</style>
