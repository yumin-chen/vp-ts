import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

function log(msg) {
  console.log(`\x1b[36m[Local CI]\x1b[0m ${msg}`);
}

function runTask(cmd) {
  log(`Executing: ${cmd}`);
  execSync(cmd, { stdio: "inherit", env: process.env });
}

async function main() {
  log("Starting Local CI Pipeline using Vite Task Orchestrator...");

  const workflowPath = path.resolve(".ci/workflow.yml");
  if (fs.existsSync(workflowPath)) {
    log(`Loaded workflow definition from ${workflowPath}`);
  }

  try {
    log("Stage 1/4: Running Code Quality Checks...");
    runTask("npx vp run ci:check");

    log("Stage 2/4: Running Unit Tests...");
    runTask("npx vp run ci:test");

    log("Stage 3/4: Building Native Addon...");
    runTask("npx vp run ci:build");

    log("Stage 4/4: Local CI completed successfully!");
  } catch (err) {
    console.error(`\x1b[31m[Local CI Failed]\x1b[0m ${err.message}`);
    process.exit(1);
  }
}

main();
