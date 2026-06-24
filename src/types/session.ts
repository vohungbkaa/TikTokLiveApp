export interface LiveSession {
  id: string;
  platform: string;
  platformSessionId?: string | null;
  title?: string | null;
  status: 'draft' | 'running' | 'paused' | 'ended';
  startedAt?: string | null;
  endedAt?: string | null;
  createdAt: string;
}

export interface CreateSessionInput {
  platform: string;
  platformSessionId?: string | null;
  title?: string | null;
}

export interface SessionProductInput {
  productId: string;
  initialStockQty: number;
  price: number;
  sellingMode: 'stock' | 'unique' | 'preorder';
}
