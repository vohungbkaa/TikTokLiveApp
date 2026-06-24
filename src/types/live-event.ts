export interface LiveEvent {
  id: string;
  sessionId: string;
  sequenceNo: number;
  source: string;
  platformEventId?: string | null;
  userId?: string | null;
  uniqueId?: string | null;
  displayName?: string | null;
  comment?: string | null;
  tsPlatform?: string | null;
  tsReceived: string;
  rawJson?: string | null;
}
