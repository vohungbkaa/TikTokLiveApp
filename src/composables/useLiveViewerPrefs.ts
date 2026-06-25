import { ref, watch } from 'vue';

export interface LiveViewerPrefs {
  watchLiveEnabled: boolean;
  autoOpenOnSessionStart: boolean;
}

const STORAGE_KEY = 'live-viewer-prefs';

const defaultPrefs: LiveViewerPrefs = {
  watchLiveEnabled: false,
  autoOpenOnSessionStart: false,
};

function loadPrefs(): LiveViewerPrefs {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...defaultPrefs };
    return { ...defaultPrefs, ...JSON.parse(raw) };
  } catch {
    return { ...defaultPrefs };
  }
}

const prefs = ref<LiveViewerPrefs>(loadPrefs());

watch(
  prefs,
  (value) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
  },
  { deep: true },
);

export function useLiveViewerPrefs() {
  const setWatchLiveEnabled = (enabled: boolean) => {
    prefs.value.watchLiveEnabled = enabled;
    if (!enabled) {
      prefs.value.autoOpenOnSessionStart = false;
    }
  };

  return {
    prefs,
    setWatchLiveEnabled,
  };
}
