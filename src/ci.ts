#!/usr/bin/env node

import { execSync } from "node:child_process";
import process from "node:process";

interface Stage {
  name: string;
  command: string;
}

const STAGES: Stage[] = [
  {
    name: "1. Code Check & Format Verification",
    command: "npx vp check",
  },
  {
    name: "2. Unit Testing",
    command: "npx vp test",
  },
  {
    name: "3. Package Bundle Verification",
    command: "npx vp pack",
  },
  {
    name: "4. Host Native Build",
    command: "node build.ts",
  },
  {
    name: "5. Cross-Build Matrix Dry-Run",
    command: "node scripts/cross-build.ts --all --dry-run",
  },
];

async function runLocalCI() {
  console.log("==================================================");
  console.log("🛠️   Running Local CI Job Pipeline");
  console.log("==================================================\n");

  const startTime = Date.now();

  for (const stage of STAGES) {
    console.log(`\n▶️ Executing Stage: ${stage.name}`);
    console.log(`   Command: ${stage.command}`);
    try {
      execSync(stage.command, { stdio: "inherit", env: process.env });
      console.log(`✅ Stage "${stage.name}" passed successfully.`);
    } catch {
      console.error(`❌ Stage "${stage.name}" failed.`);
      process.exit(1);
    }
  }

  const duration = ((Date.now() - startTime) / 1000).toFixed(2);
  console.log("\n==================================================");
  console.log(`🎉 All Local CI Stages Passed successfully in ${duration}s!`);
  console.log("==================================================\n");
}

runLocalCI().catch((err) => {
  console.error("Unhandled error in local CI pipeline:", err);
  process.exit(1);
});
