import { generateRecords, processRecords, analyze } from "../domain/engine";
import { InMemoryRepository } from "../io/repository";

function assert(condition: unknown, message: string): void {
  if (!condition) {
    throw new Error(`Assertion failed: ${message}`);
  }
}

function testGenerateAndProcess(): void {
  const records = generateRecords(12, 7);
  assert(records.length === 12, "generateRecords should create exact count");

  const processed = processRecords(records, "normal");
  assert(processed.length === 12, "processRecords should keep count");

  const report = analyze(processed, "normal");
  assert(report.total === 12, "report.total should be 12");
  assert(report.topRecords.length > 0, "topRecords should not be empty");
}

function testRepository(): void {
  const repo = new InMemoryRepository();
  const sample = generateRecords(8, 3);
  repo.store(sample);

  assert(repo.all().length === 8, "repo all length");
  assert(repo.activeCount() > 0, "active count should be positive");
  assert(repo.byCategory("alpha").length > 0, "alpha category should exist");
}

function run(): void {
  const tests: Array<{ name: string; fn: () => void }> = [
    { name: "generate and process", fn: testGenerateAndProcess },
    { name: "repository", fn: testRepository },
  ];

  let passed = 0;
  for (const t of tests) {
    try {
      t.fn();
      console.log(`PASS: ${t.name}`);
      passed++;
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`FAIL: ${t.name} - ${message}`);
      process.exitCode = 1;
    }
  }

  console.log(`${passed}/${tests.length} tests passed`);
  if (passed !== tests.length) {
    process.exit(1);
  }
}

run();
