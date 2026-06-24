const { WebcastPushConnection } = require('tiktok-live-connector');
import { emitHealth, emitError, emitEvent } from './protocol';
import { mapCommentEvent } from './mappers';

export class TikTokClient {
    private connection: any | null = null;
    private isShuttingDown = false;

    constructor(private username: string, private sessionId: string) {}

    public async connect() {
        emitHealth("room_resolve", true);
        
        this.connection = new WebcastPushConnection(this.username, {
            processInitialData: false,
            enableExtendedGiftInfo: false,
            enableWebsocketUpgrade: true,
            requestPollingIntervalMs: 2000,
            clientParams: {
                "app_language": "vi-VN",
                "device_platform": "web"
            }
        });

        this.setupEvents();

        try {
            emitHealth("connecting", true);
            await this.connection.connect();
            emitHealth("connected", true);
        } catch (err: any) {
            emitError("websocket_connect", "CONNECT_FAILED", err.message || "Failed to connect");
            this.handleDisconnect();
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
        
        this.connection.on('streamEnd', (actionId: any) => {
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
        }, 5000); // 5s reconnect
    }

    public stop() {
        this.isShuttingDown = true;
        if (this.connection) {
            try {
                this.connection.disconnect();
            } catch(e) {}
        }
        process.exit(0);
    }
}
