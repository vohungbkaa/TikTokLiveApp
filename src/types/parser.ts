export interface ParsedIntent {
  intent: "Buy" | "Cancel" | "Question" | "Noise";
  sku: string | null;
  quantity: number;
  variants: string[];
  confidenceScore: number;
  needsReview: boolean;
  normalizedText: string;
  matchedRules: string[];
}
