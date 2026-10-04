import { execSync } from "node:child_process";
import process from "node:process";

function runCommand(command: string) {
  console.log(`\n▶ Running: ${command}`);
  try {
    execSync(command, { stdio: "inherit", env: process.env });
    console.log(`✔ Success: ${command}`);
  } catch (err) {
    console.error(`✖ Failed: ${command}`);
    process.exit(1);
  }
}

async function main() {
  const args = process.argv.slice(2);
  const isCheck = args.includes("--check") || args.includes("check");
  const isBuild = args.includes("--build") || args.includes("build");
  const isTest = args.includes("--test") || args.includes("test");
  const isCrossBuild = args.includes("--cross-build") || args.includes("cross-build");
  const isPrepublish = args.includes("--prepublish") || args.includes("prepublish");
  const isPublish = args.includes("--publish") || args.includes("publish");

  const runAll =
    !isCheck && !isBuild && !isTest && !isCrossBuild && !isPrepublish && !isPublish;

  console.log("🚀 Starting Local CI Job Orchestrator...");

  if (runAll || isCheck) {
    runCommand("npm run ci:check");
  }

  if (runAll || isBuild) {
    runCommand("npm run ci:build");
  }

  if (runAll || isTest) {
    runCommand("npm run ci:test");
  }

  if (isCrossBuild) {
    runCommand("node build.mjs --use-cross");
  }

  if (runAll || isPrepublish) {
    runCommand("npm run test:prepublish");
  }

  if (isPublish) {
    runCommand("npm run publish");
  }

  console.log("\n✨ Local CI Job Completed Successfully!");
}

main().catch((err) => {
  console.error("Fatal CI orchestrator error:", err);
  process.exit(1);
});
