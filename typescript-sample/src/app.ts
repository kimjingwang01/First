export type AppMode = "quick" | "normal" | "deep";

export interface AppConfig {
  mode: AppMode;
  items: number;
  seed: number;
}

export function parseArgs(argv: string[]): AppConfig {
  let mode: AppMode = "normal";
  let items = 20;
  let seed = 1;

  for (let i = 0; i < argv.length; i++) {
    const key = argv[i];
    const next = argv[i + 1];

    if (key === "--mode" && next) {
      if (next === "quick" || next === "normal" || next === "deep") {
        mode = next;
      }
      i++;
    } else if (key === "--items" && next) {
      const parsed = Number(next);
      if (!Number.isNaN(parsed)) {
        items = Math.max(1, Math.floor(parsed));
      }
      i++;
    } else if (key === "--seed" && next) {
      const parsed = Number(next);
      if (!Number.isNaN(parsed)) {
        seed = Math.max(1, Math.floor(parsed));
      }
      i++;
    }
  }

  return { mode, items, seed };
}

export function validateConfig(config: AppConfig): void {
  if (config.items <= 0) {
    throw new Error("items must be > 0");
  }
  if (config.items > 100_000) {
    throw new Error("items is too large");
  }
}
