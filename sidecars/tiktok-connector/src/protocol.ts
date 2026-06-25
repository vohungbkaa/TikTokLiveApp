export interface HealthMessage {
    type: "health";
    stage: string;
    ok: boolean;
}

export interface ErrorMessage {
    type: "error";
    stage: string;
    code: string;
    message: string;
}

export interface EventMessage {
    type: "event";
    event_type: string;
    data: any;
}

// Node buffers stdout when piped (Tauri shell). Force immediate flush per line.
const stdoutHandle = (process.stdout as NodeJS.WriteStream & { _handle?: { setBlocking?: (blocking: boolean) => void } })._handle;
if (stdoutHandle?.setBlocking) {
    stdoutHandle.setBlocking(true);
}

function emitLine(payload: object) {
    process.stdout.write(`${JSON.stringify(payload)}\n`);
}

export function emitHealth(stage: string, ok: boolean) {
    emitLine({ type: "health", stage, ok });
}

export function emitError(stage: string, code: string, message: string) {
    emitLine({ type: "error", stage, code, message });
    console.error(`[ERROR][${stage}][${code}] ${message}`);
}

export function emitEvent(event_type: string, data: any) {
    emitLine({ type: "event", event_type, data });
}

export function emitStream(data: {
    hls_url?: string;
    flv_url?: string;
    live_page_url: string;
    room_id?: string;
    source: string;
    error?: string;
}) {
    emitLine({ type: "stream", ...data });
}
