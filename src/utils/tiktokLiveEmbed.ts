import { LogicalPosition, LogicalSize } from '@tauri-apps/api/dpi';
import { Webview } from '@tauri-apps/api/webview';
import { getCurrentWindow } from '@tauri-apps/api/window';

export const EMBED_LABEL = 'tiktok-live-embed';

export function buildLiveUrls(username: string, loginFirst: boolean) {
  const clean = username.replace(/^@/, '').split(/[/?]/)[0];
  const liveUrl = `https://www.tiktok.com/@${clean}/live`;
  if (loginFirst) {
    return {
      url: `https://www.tiktok.com/login?redirect_url=${encodeURIComponent(liveUrl)}`,
      liveUrl,
    };
  }
  return { url: liveUrl, liveUrl };
}

async function readBounds(mountEl: HTMLElement) {
  const rect = mountEl.getBoundingClientRect();
  return {
    x: Math.round(rect.left),
    y: Math.round(rect.top),
    width: Math.max(Math.round(rect.width), 280),
    height: Math.max(Math.round(rect.height), 220),
  };
}

let resizeObserver: ResizeObserver | null = null;
let observedMount: HTMLElement | null = null;
let activeWebview: Webview | null = null;

async function syncBounds(mountEl: HTMLElement) {
  const webview = activeWebview ?? (await Webview.getByLabel(EMBED_LABEL));
  if (!webview) return;
  const bounds = await readBounds(mountEl);
  await webview.setPosition(new LogicalPosition(bounds.x, bounds.y));
  await webview.setSize(new LogicalSize(bounds.width, bounds.height));
}

function attachResizeObserver(mountEl: HTMLElement) {
  detachResizeObserver();
  observedMount = mountEl;
  resizeObserver = new ResizeObserver(() => {
    void syncBounds(mountEl);
  });
  resizeObserver.observe(mountEl);
  window.addEventListener('resize', onWindowResize);
}

function onWindowResize() {
  if (observedMount) void syncBounds(observedMount);
}

function detachResizeObserver() {
  resizeObserver?.disconnect();
  resizeObserver = null;
  observedMount = null;
  window.removeEventListener('resize', onWindowResize);
}

export async function openEmbeddedLive(
  mountEl: HTMLElement,
  username: string,
  loginFirst: boolean,
): Promise<void> {
  const { url } = buildLiveUrls(username, loginFirst);
  const bounds = await readBounds(mountEl);
  const appWindow = getCurrentWindow();

  await closeEmbeddedLive();

  const webview = new Webview(appWindow, EMBED_LABEL, {
    url,
    x: bounds.x,
    y: bounds.y,
    width: bounds.width,
    height: bounds.height,
    focus: true,
  });

  await new Promise<void>((resolve, reject) => {
    webview.once('tauri://created', () => resolve());
    webview.once('tauri://error', (event) => {
      reject(new Error(String(event.payload ?? 'Failed to create embedded webview')));
    });
  });

  activeWebview = webview;
  attachResizeObserver(mountEl);
  await webview.show();
}

export async function closeEmbeddedLive(): Promise<void> {
  detachResizeObserver();
  const existing = activeWebview ?? (await Webview.getByLabel(EMBED_LABEL));
  activeWebview = null;
  if (existing) {
    await existing.close();
  }
}

export async function isEmbeddedLiveOpen(): Promise<boolean> {
  if (activeWebview) return true;
  return (await Webview.getByLabel(EMBED_LABEL)) !== null;
}
