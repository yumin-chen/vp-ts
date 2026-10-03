import { execSync } from "node:child_process";
import process from "node:process";
import { disableCache, getEnv } from "@voidzero-dev/vite-task-client";

const isDryRun = process.argv.includes("--dry-run") || getEnv("DRY_RUN") === "true";

interface CIStep {
  name: string;
  command: string;
}

const steps: CIStep[] = [
  { name: "Code Format & Lint Check", command: "npx vp check" },
  { name: "Native Host Build", command: "node build.js" },
  { name: "Unit & Integration Tests", command: "npx vp test && node --test test.cjs" },
  { name: "Local Matrix Cross-Build (Dry Run)", command: "node build.js --use-cross --dry-run" },
];

async function runCI(): Promise<void> {
  console.log("🚀 Starting Local TypeScript CI Task Runner...\n");

  let passed = 0;

  for (let i = 0; i < steps.length; i++) {
    const step = steps[i];
    console.log(`[Step ${i + 1}/${steps.length}] ${step.name}`);
    console.log(`Command: ${step.command}`);

    if (isDryRun) {
      console.log(`   [Dry Run] Skipped execution.\n`);
      passed++;
      continue;
    }

    try {
      execSync(step.command, { stdio: "inherit", env: process.env });
      console.log(`✅ Step passed.\n`);
      passed++;
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : String(err);
      console.error(`❌ Step failed: ${step.name} (${msg})\n`);
      disableCache();
      process.exit(1);
    }
  }

  console.log(`✨ Local CI Complete! (${passed}/${steps.length} steps passed)`);
}

void runCI();
