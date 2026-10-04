#!/usr/bin/env node

import { execSync } from "node:child_process";
import process from "node:process";

console.log("==========================================");
console.log("🚀 Starting Local Offline CI Job Pipeline");
console.log("==========================================");

const steps = [
  { name: "Code Quality Check", cmd: "vp run ci:check" },
  { name: "Target Cross-Build", cmd: "vp run ci:build" },
  { name: "Test Suite Execution", cmd: "vp run ci:test" },
];

let failed = false;

for (const step of steps) {
  console.log(`\n▶ Running Step: ${step.name}`);
  console.log(`  Command: ${step.cmd}`);
  try {
    execSync(step.cmd, { stdio: "inherit", env: process.env });
    console.log(`✅ Step Passed: ${step.name}`);
  } catch {
    console.error(`❌ Step Failed: ${step.name}`);
    failed = true;
    break;
  }
}

if (failed) {
  console.error("\n💥 Local CI Pipeline Failed!");
  process.exit(1);
} else {
  console.log("\n🎉 Local CI Pipeline Completed Successfully!");
}
