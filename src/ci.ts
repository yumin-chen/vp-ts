import console from "node:console";
import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { getEnv } from "@voidzero-dev/vite-task-client";

interface Step {
  name: string;
  command: string;
}

interface Job {
  name: string;
  steps: Step[];
}

function parseSimpleYamlWorkflow(filePath: string): Job[] {
  if (!fs.existsSync(filePath)) {
    return [];
  }
  const content = fs.readFileSync(filePath, "utf-8");
  const jobs: Job[] = [];
  let currentJob: Job | null = null;
  let currentStep: Step | null = null;
  let inJobsSection = false;

  for (const rawLine of content.split("\n")) {
    const line = rawLine.trimEnd();

    if (line.match(/^jobs:/)) {
      inJobsSection = true;
      continue;
    }

    if (!inJobsSection) {
      continue;
    }

    if (line.match(/^  [a-zA-Z0-9_-]+:$/)) {
      if (currentJob) {
        if (currentStep) {
          currentJob.steps.push(currentStep);
          currentStep = null;
        }
        jobs.push(currentJob);
      }
      const jobKey = line.trim().slice(0, -1);
      currentJob = { name: jobKey, steps: [] };
      continue;
    }

    if (currentJob) {
      if (line.trim().startsWith("name:") && !currentStep) {
        const parts = line.split("name:");
        currentJob.name = (parts[1] ?? "").trim().replace(/^["']|["']$/g, "");
      } else if (line.trim().startsWith("- run:")) {
        if (currentStep) {
          currentJob.steps.push(currentStep);
        }
        const parts = line.split("- run:");
        const cmd = (parts[1] ?? "").trim().replace(/^["']|["']$/g, "");
        currentStep = { name: cmd, command: cmd };
      }
    }
  }

  if (currentJob) {
    if (currentStep) {
      currentJob.steps.push(currentStep);
    }
    jobs.push(currentJob);
  }

  return jobs;
}

export function runCI() {
  console.log("🚀 Starting Local CI Runner...");
  const isCI = getEnv("CI") || process.env["CI"];
  if (isCI) {
    console.log("ℹ️ Environment: CI mode enabled");
  }

  const workflowPath = ".github/workflows/ci.yml";
  const yamlJobs = parseSimpleYamlWorkflow(workflowPath);

  const jobsToRun: Job[] =
    yamlJobs.length > 0
      ? yamlJobs
      : [
          {
            name: "check",
            steps: [{ name: "vp check", command: "vp check src/" }],
          },
          {
            name: "build",
            steps: [{ name: "build", command: "node build.mjs && vp pack" }],
          },
          {
            name: "test",
            steps: [{ name: "test", command: "node --test test.cjs && vp test" }],
          },
          {
            name: "publish",
            steps: [{ name: "publish", command: "vp pack" }],
          },
        ];

  let hasError = false;

  const binPath = path.resolve("node_modules/.bin");
  const envWithBin = {
    ...process.env,
    PATH: `${binPath}:${process.env["PATH"] ?? ""}`,
  };

  for (const job of jobsToRun) {
    console.log(`\n📋 Job: ${job.name}`);
    for (const step of job.steps) {
      console.log(`  ▶ Step: ${step.name} (${step.command})`);
      const startTime = Date.now();
      try {
        execSync(step.command, { stdio: "inherit", env: envWithBin });
        const duration = Date.now() - startTime;
        console.log(`  ✅ Step completed in ${duration}ms`);
      } catch (error) {
        console.error(`  ❌ Step failed: ${step.name}`, error);
        hasError = true;
        break;
      }
    }
    if (hasError) {
      break;
    }
  }

  if (hasError) {
    console.error("\n💥 Local CI Pipeline failed!");
    process.exit(1);
  } else {
    console.log("\n🎉 All Local CI jobs completed successfully!");
  }
}

if (process.argv[1] && process.argv[1].endsWith("ci.ts")) {
  runCI();
}
