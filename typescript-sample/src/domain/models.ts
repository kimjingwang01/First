export interface DataRecord {
  id: number;
  category: "alpha" | "beta" | "gamma" | "delta";
  value: number;
  tags: string[];
  active: boolean;
}

export interface ProcessedRecord {
  id: number;
  category: DataRecord["category"];
  rawValue: number;
  normalizedValue: number;
  score: number;
  risk: "low" | "medium" | "high" | "critical";
  notes: string[];
}

export interface AnalysisReport {
  mode: "quick" | "normal" | "deep";
  total: number;
  averageScore: number;
  minScore: number;
  maxScore: number;
  riskCounts: Record<string, number>;
  categoryCounts: Record<string, number>;
  topRecords: ProcessedRecord[];
}
