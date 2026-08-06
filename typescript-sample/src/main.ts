import { parseArgs, validateConfig } from "./app";
import { generateRecords, processRecords, analyze } from "./domain/engine";
import { InMemoryRepository } from "./io/repository";

function renderReport(report: ReturnType<typeof analyze>): string {
  const lines: string[] = [];
  lines.push("=== TypeScript Analysis Report ===");
  lines.push(`mode: ${report.mode}`);
  lines.push(`total: ${report.total}`);
  lines.push(`averageScore: ${report.averageScore.toFixed(4)}`);
  lines.push(`minScore: ${report.minScore.toFixed(4)}`);
  lines.push(`maxScore: ${report.maxScore.toFixed(4)}`);
  lines.push("riskCounts:");
  for (const [k, v] of Object.entries(report.riskCounts)) {
    lines.push(`  - ${k}: ${v}`);
  }
  lines.push("categoryCounts:");
  for (const [k, v] of Object.entries(report.categoryCounts)) {
    lines.push(`  - ${k}: ${v}`);
  }
  lines.push("topRecords:");
  for (const r of report.topRecords) {
    lines.push(`  - id=${r.id} category=${r.category} score=${r.score.toFixed(3)} risk=${r.risk}`);
  }
  return lines.join("\n");
}

function main() {
  try {
    const config = parseArgs(process.argv.slice(2));
    validateConfig(config);

    const records = generateRecords(config.items, config.seed);
    const repo = new InMemoryRepository();
    repo.store(records);

    const processed = processRecords(repo.all(), config.mode);
    const report = analyze(processed, config.mode);

    console.log(renderReport(report));
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`error: ${message}`);
    process.exit(1);
  }
}

main();
