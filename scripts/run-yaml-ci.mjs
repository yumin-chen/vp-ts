#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { execSync } from "node:child_process";

console.log("🤖 Starting Local YAML CI Runner...");

const ciYmlPath = path.resolve(process.cwd(), "ci.yml");
if (!fs.existsSync(ciYmlPath)) {
  console.error("❌ ci.yml not found in working directory");
  process.exit(1);
}

const content = fs.readFileSync(ciYmlPath, "utf8");

// Minimal YAML parser for CI jobs structure
function parseYmlJobs(yamlText) {
  const jobs = [];
  let currentJob = null;
  let inSteps = false;
  let currentStep = null;

  const lines = yamlText.split("\n");
  for (let line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;

    if (line.match(/^  [a-zA-Z0-9_-]+:/) && !trimmed.startsWith("steps:")) {
      const jobName = trimmed.replace(":", "").trim();
      currentJob = { name: jobName, steps: [] };
      jobs.push(currentJob);
      inSteps = false;
      continue;
    }

    if (trimmed === "steps:") {
      inSteps = true;
      continue;
    }

    if (inSteps && currentJob) {
      if (trimmed.startsWith("- uses:")) {
        currentStep = { uses: trimmed.replace("- uses:", "").trim() };
        currentJob.steps.push(currentStep);
      } else if (trimmed.startsWith("- run:")) {
        currentStep = { run: trimmed.replace("- run:", "").trim() };
        currentJob.steps.push(currentStep);
      } else if (trimmed.startsWith("- name:")) {
        currentStep = { name: trimmed.replace("- name:", "").trim() };
        currentJob.steps.push(currentStep);
      } else if (currentStep && trimmed.startsWith("run:")) {
        currentStep.run = trimmed.replace("run:", "").trim();
      } else if (currentStep && trimmed.startsWith("uses:")) {
        currentStep.uses = trimmed.replace("uses:", "").trim();
      }
    }
  }

  return jobs;
}

const jobs = parseYmlJobs(content);
console.log(`📋 Found ${jobs.length} jobs in ci.yml`);

let passed = 0;
let total = 0;

// Default local target matrix if matrix variables are present
const defaultTargets = [
  { target: "x86_64-unknown-linux-gnu", flags: "" },
  { target: "wasm32-wasip1-threads", flags: "" },
];

for (const job of jobs) {
  console.log(`\n⚙️ Executing Job: [${job.name}]`);
  for (const step of job.steps) {
    total++;
    let stepCmd = step.run || "";
    let stepUses = step.uses || "";
    const stepLabel = step.name || stepCmd || stepUses || "Unnamed step";

    console.log(`  ▶️ Step: ${stepLabel}`);

    try {
      if (stepUses) {
        if (stepUses.includes("setup-vp")) {
          console.log(`    ℹ️ Handled action: ${stepUses} (verifying vp setup locally)`);
          execSync("npx vp --version", { stdio: "inherit" });
        } else {
          console.log(`    ℹ️ Simulated action: ${stepUses}`);
        }
      }

      if (stepCmd) {
        // Handle vp command replacement for local execution
        if (stepCmd.startsWith("vp ")) {
          stepCmd = "npx " + stepCmd;
        }

        if (stepCmd.includes("${{ matrix.settings.target }}")) {
          for (const item of defaultTargets) {
            const expandedCmd = stepCmd
              .replace(/\$\{\{\s*matrix\.settings\.target\s*\}\}/g, item.target)
              .replace(/\$\{\{\s*matrix\.settings\.flags\s*\}\}/g, item.flags);
            console.log(`    $ ${expandedCmd}`);
            execSync(expandedCmd, { stdio: "inherit" });
          }
        } else {
          console.log(`    $ ${stepCmd}`);
          execSync(stepCmd, { stdio: "inherit" });
        }
      }
      passed++;
    } catch (err) {
      console.error(`    ❌ Step failed: ${err.message}`);
    }
  }
}

console.log(`\n✅ Local YAML CI Runner finished. Passed ${passed}/${total} steps.\n`);
