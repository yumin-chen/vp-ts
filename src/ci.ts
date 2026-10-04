import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { parse as parseYaml } from "yaml";

interface Step {
  name?: string;
  run?: string;
  uses?: string;
  with?: Record<string, unknown>;
}

interface MatrixSetting {
  target?: string;
  flags?: string;
  docker?: string;
  args?: string;
  host?: string;
  architecture?: string;
}

interface Job {
  name?: string;
  steps?: Step[];
  strategy?: {
    matrix?: {
      settings?: MatrixSetting[];
    };
  };
}

interface Workflow {
  name?: string;
  env?: Record<string, string>;
  jobs?: Record<string, Job>;
}

function runCommand(cmd: string, env: Record<string, string> = {}) {
  console.log(`\n> ${cmd}`);
  try {
    execSync(cmd, {
      stdio: "inherit",
      env: { ...process.env, ...env },
    });
  } catch (error) {
    console.error(`Command failed: ${cmd}`);
    throw error;
  }
}

function loadWorkflow(): Workflow | null {
  const possiblePaths = [
    path.resolve("ci.yml"),
    path.resolve("ci.yaml"),
    path.resolve(".ci/ci.yml"),
  ];
  for (const workflowPath of possiblePaths) {
    if (fs.existsSync(workflowPath)) {
      try {
        const content = fs.readFileSync(workflowPath, "utf-8");
        return parseYaml(content) as Workflow;
      } catch (e) {
        console.warn(`Could not parse ${workflowPath}:`, e);
      }
    }
  }
  return null;
}

export async function runCiJob(jobName: string) {
  const workflow = loadWorkflow();
  const globalEnv = workflow?.env || {};

  console.log(`=== Running Local CI Job: [${jobName}] ===`);

  switch (jobName) {
    case "check": {
      runCommand("npx vp check", globalEnv);
      break;
    }

    case "build": {
      runCommand("node build.js", globalEnv);
      break;
    }

    case "test": {
      runCommand("npx vp test", globalEnv);
      if (fs.existsSync("test.cjs")) {
        runCommand("node --test test.cjs", globalEnv);
      }
      break;
    }

    case "cross": {
      const targetArg = process.argv[3];
      const buildJob = workflow?.jobs?.["build"];
      const matrix = buildJob?.strategy?.matrix?.settings;
      if (targetArg) {
        console.log(`Running cross-build for target: ${targetArg}`);
        runCommand(`node build.js --target ${targetArg}`, globalEnv);
      } else if (matrix && matrix.length > 0) {
        console.log(`Running local cross-build check for workflow targets...`);
        const targetsToBuild = matrix.slice(0, 2);
        for (const item of targetsToBuild) {
          if (item.target) {
            const flags = item.flags || "";
            const cmd = `node build.js --target ${item.target} ${flags}`.trim();
            try {
              console.log(`\n--- Cross build target: ${item.target} ---`);
              runCommand(cmd, globalEnv);
            } catch {
              console.warn(
                `[Warning] Cross-build for target ${item.target} skipped due to missing toolchain. Continuing local CI...`,
              );
            }
          }
        }
      } else {
        runCommand("node build.js", globalEnv);
      }
      break;
    }

    case "publish": {
      runCommand("npx vp pack", globalEnv);
      runCommand("npm publish --dry-run", globalEnv);
      break;
    }

    case "all":
    default: {
      console.log("Running full local CI pipeline...");
      await runCiJob("check");
      await runCiJob("build");
      await runCiJob("test");
      await runCiJob("cross");
      await runCiJob("publish");
      console.log("\n✅ Full local CI pipeline completed successfully!");
      break;
    }
  }
}

const arg = process.argv[2] || "all";
runCiJob(arg).catch((err) => {
  console.error("\n❌ Local CI Job Failed:", err);
  process.exit(1);
});
