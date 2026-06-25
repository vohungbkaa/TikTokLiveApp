import { TikTokLiveConnection } from 'tiktok-live-connector';
import { emitHealth, emitError, emitEvent, emitStream } from './protocol';
import { mapCommentEvent } from './mappers';
import { resolveStreamInfo } from './stream';

export class TikTokClient {
    private connection: TikTokLiveConnection | null = null;
    private isShuttingDown = false;

    constructor(
        private username: string,
        private sessionId: string,
        private tiktokSessionCookie?: string,
    ) {}

    public async connect() {
        emitHealth("room_resolve", true);

        const sessionOptions = this.tiktokSessionCookie
            ? { cookie: this.tiktokSessionCookie }
            : undefined;

        this.connection = new TikTokLiveConnection(this.username, {
            processInitialData: false,
            enableExtendedGiftInfo: false,
            enableWebsocketUpgrade: true,
            requestPollingIntervalMs: 2000,
            session: sessionOptions,
            clientParams: {
                app_language: "vi-VN",
                device_platform: "web",
            },
        });

        this.setupEvents();

        try {
            emitHealth("connecting", true);
            await this.connection.connect();
            emitHealth("connected", true);
            await this.emitStreamInfo();
        } catch (err: any) {
            emitError("websocket_connect", "CONNECT_FAILED", err.message || "Failed to connect");
            this.handleDisconnect();
        }
    }

    private async emitStreamInfo() {
        if (!this.connection) return;
        try {
            const info = await resolveStreamInfo(this.connection, this.username);
            emitStream(info);
        } catch (err: any) {
            emitStream({
                live_page_url: `https://www.tiktok.com/@${this.username.replace(/^@/, '')}/live`,
                source: "error",
                error: err?.message || "Failed to resolve stream URL",
            });
        }
    }

    private setupEvents() {
        if (!this.connection) return;

        this.connection.on('chat', (data: any) => {
            const payload = mapCommentEvent(data);
            emitEvent("comment", payload);
        });

        this.connection.on('disconnected', () => {
            if (!this.isShuttingDown) {
                emitHealth("disconnected", false);
                this.handleDisconnect();
            }
        });

        this.connection.on('error', (err: any) => {
            emitError("unknown", "CONNECTION_ERROR", err.message || "Unknown error");
        });

        this.connection.on('streamEnd', () => {
            emitHealth("stream_ended", true);
            this.handleDisconnect();
        });
    }

    private handleDisconnect() {
        if (this.isShuttingDown) return;
        emitHealth("reconnecting", true);
        setTimeout(() => {
            if (!this.isShuttingDown) {
                this.connect();
            }
        }, 5000);
    }

    public stop() {
        this.isShuttingDown = true;
        if (this.connection) {
            try {
                this.connection.disconnect();
            } catch {
                // ignore
            }
        }
        process.exit(0);
    }
}
