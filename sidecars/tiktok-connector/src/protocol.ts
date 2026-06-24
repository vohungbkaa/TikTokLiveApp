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

export function emitHealth(stage: string, ok: boolean) {
    console.log(JSON.stringify({ type: "health", stage, ok }));
}

export function emitError(stage: string, code: string, message: string) {
    console.log(JSON.stringify({ type: "error", stage, code, message }));
    // Also write to stderr for debugging
    console.error(`[ERROR][${stage}][${code}] ${message}`);
}

export function emitEvent(event_type: string, data: any) {
    console.log(JSON.stringify({ type: "event", event_type, data }));
}
