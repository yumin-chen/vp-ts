import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { execSync } from "node:child_process";
import * as yaml from "js-yaml";

interface Step {
  name?: string;
  run: string;
}

interface Job {
  name?: string;
  steps: Step[];
}

interface Workflow {
  name?: string;
  jobs: Record<string, Job>;
}

function loadWorkflow(): Workflow {
  const ciYmlPath = path.resolve(process.cwd(), "ci.yml");
  if (!fs.existsSync(ciYmlPath)) {
    throw new Error(`Local CI configuration file not found at ${ciYmlPath}`);
  }
  const content = fs.readFileSync(ciYmlPath, "utf8");
  return yaml.load(content) as Workflow;
}

function runJob(jobId: string, job: Job) {
  console.log(`\n========================================`);
  console.log(`🏃 Executing Local CI Job: ${job.name || jobId} [${jobId}]`);
  console.log(`========================================\n`);

  for (const step of job.steps) {
    if (step.name) {
      console.log(`🔹 Step: ${step.name}`);
    }
    console.log(`   $ ${step.run}`);
    try {
      execSync(step.run, { stdio: "inherit", env: process.env });
      console.log(`✅ Step completed successfully.\n`);
    } catch (err: any) {
      console.error(`❌ Step failed with exit code ${err.status || 1}`);
      process.exit(err.status || 1);
    }
  }

  console.log(`✨ Job '${job.name || jobId}' passed successfully!\n`);
}

function main() {
  const args = process.argv.slice(2);
  const targetJob = args[0]?.toLowerCase();

  const workflow = loadWorkflow();
  const jobs = workflow.jobs || {};

  console.log(`🚀 Starting Local CI Workflow: ${workflow.name || "Local CI"}`);

  if (targetJob && targetJob !== "all") {
    const job = jobs[targetJob];
    if (!job) {
      console.error(
        `❌ Specified job '${targetJob}' not found in ci.yml. Available jobs: ${Object.keys(jobs).join(", ")}`,
      );
      process.exit(1);
    }
    runJob(targetJob, job);
  } else {
    for (const [jobId, job] of Object.entries(jobs)) {
      runJob(jobId, job);
    }
  }

  console.log(`🎉 Local CI Workflow execution completed successfully!`);
}

main();
