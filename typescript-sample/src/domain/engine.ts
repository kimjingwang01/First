import { DataRecord, ProcessedRecord, AnalysisReport } from "./models";

function lcg(seed: number): number {
  return (seed * 48271) % 0x7fffffff;
}

export function generateRecords(items: number, seed: number): DataRecord[] {
  const out: DataRecord[] = [];
  let x = Math.max(1, seed);
  const cats: DataRecord["category"][] = ["alpha", "beta", "gamma", "delta"];

  for (let i = 0; i < items; i++) {
    x = lcg(x);
    const value = (x % 10000) / 33;
    const category = cats[i % cats.length];
    const tags: string[] = [];
    if (value > 120) tags.push("high");
    if (value < 25) tags.push("low");
    if (i % 3 === 0) tags.push("periodic");

    out.push({
      id: i + 1,
      category,
      value,
      tags,
      active: i % 2 === 0,
    });
  }

  return out;
}

function classify(score: number, mode: "quick" | "normal" | "deep"): ProcessedRecord["risk"] {
  const [a, b, c] = mode === "quick" ? [55, 90, 125] : mode === "deep" ? [70, 115, 165] : [60, 100, 145];
  if (score < a) return "low";
  if (score < b) return "medium";
  if (score < c) return "high";
  return "critical";
}

export function processRecords(records: DataRecord[], mode: "quick" | "normal" | "deep"): ProcessedRecord[] {
  if (records.length === 0) throw new Error("no records");

  const baseline = records.reduce((acc, r) => acc + r.value, 0) / records.length;

  return records.map((r) => {
    const normalizedValue = (r.value / Math.max(1, baseline)) * 100;
    const categoryFactor = r.category === "alpha" ? 1.15 : r.category === "beta" ? 1.05 : r.category === "gamma" ? 0.95 : 0.88;
    const modeFactor = mode === "quick" ? 0.8 : mode === "deep" ? 1.25 : 1.0;

    let score = normalizedValue * categoryFactor * modeFactor + Math.sin(r.id * 0.173) * 7;
    const notes: string[] = [];

    if (r.tags.includes("high")) {
      score *= 1.12;
      notes.push("high tag multiplier");
    }
    if (r.tags.includes("low")) {
      score *= 0.91;
      notes.push("low tag dampener");
    }
    if (r.active) {
      score += 3.5;
      notes.push("active boost");
    }

    return {
      id: r.id,
      category: r.category,
      rawValue: r.value,
      normalizedValue,
      score,
      risk: classify(score, mode),
      notes,
    };
  });
}

export function analyze(records: ProcessedRecord[], mode: "quick" | "normal" | "deep"): AnalysisReport {
  const total = records.length;
  const sum = records.reduce((acc, r) => acc + r.score, 0);
  const minScore = Math.min(...records.map((r) => r.score));
  const maxScore = Math.max(...records.map((r) => r.score));

  const riskCounts: Record<string, number> = {};
  const categoryCounts: Record<string, number> = {};

  for (const r of records) {
    riskCounts[r.risk] = (riskCounts[r.risk] ?? 0) + 1;
    categoryCounts[r.category] = (categoryCounts[r.category] ?? 0) + 1;
  }

  const topRecords = [...records].sort((a, b) => b.score - a.score).slice(0, 5);

  return {
    mode,
    total,
    averageScore: total > 0 ? sum / total : 0,
    minScore,
    maxScore,
    riskCounts,
    categoryCounts,
    topRecords,
  };
}
