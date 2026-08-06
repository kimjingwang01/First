import { DataRecord } from "../domain/models";

export class InMemoryRepository {
  private records: DataRecord[] = [];

  store(batch: DataRecord[]): void {
    if (batch.length === 0) {
      throw new Error("empty batch");
    }

    for (const item of batch) {
      if (item.id <= 0) {
        throw new Error("invalid id");
      }
      this.records.push(item);
    }
  }

  all(): DataRecord[] {
    return [...this.records];
  }

  byCategory(category: DataRecord["category"]): DataRecord[] {
    return this.records.filter((r) => r.category === category);
  }

  activeCount(): number {
    return this.records.filter((r) => r.active).length;
  }
}
