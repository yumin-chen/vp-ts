import { execSync } from "node:child_process";
import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";
import process from "node:process";
import { NapiCli } from "@napi-rs/cli";

function runCmd(cmd: string) {
  console.log(`\n▶ Running: ${cmd}`);
  execSync(cmd, { stdio: "inherit", env: process.env });
}

async function runCheck() {
  console.log("\n🔍 Running CI Check Job...");
  runCmd("npx vp check --no-lint");
  console.log("✅ CI Check completed.");
}

async function runBuild() {
  console.log("\n⚙️ Running CI Build Job...");
  runCmd("node build.mjs --target-all --release");
  console.log("✅ CI Build completed.");
}

async function runTest() {
  console.log("\n🧪 Running CI Test Job...");
  runCmd("npm test");
  console.log("✅ CI Test completed.");
}

async function runPrepublish() {
  console.log("\n📦 Running CI Prepublish Job...");
  const cli = new NapiCli();
  await cli.prePublish({
    npmDir: "npm",
    dryRun: true,
  });
  console.log("✅ CI Prepublish completed.");
}

async function runPublish() {
  console.log("\n🚀 Running CI Publish Job...");
  const npmDir = join(process.cwd(), "npm");
  if (!existsSync(npmDir)) {
    console.log("npm directory not found. Preparing npm packages first...");
    const cli = new NapiCli();
    await cli.prePublish({
      npmDir: "npm",
      dryRun: false,
    });
  }

  if (existsSync(npmDir)) {
    const subdirs = readdirSync(npmDir, { withFileTypes: true })
      .filter((d) => d.isDirectory())
      .map((d) => d.name);

    for (const dir of subdirs) {
      const pkgPath = join(npmDir, dir);
      console.log(`\nPublishing package from ${pkgPath}...`);
      try {
        runCmd(`npm publish "${pkgPath}" --access public`);
      } catch (err: any) {
        console.error(`Failed to publish package at ${pkgPath}:`, err.message);
      }
    }
  }

  console.log("\nPublishing root package...");
  try {
    runCmd("npm publish --access public");
  } catch (err: any) {
    console.error("Failed to publish root package:", err.message);
  }

  console.log("✅ CI Publish completed.");
}

async function main() {
  const command = process.argv[2] || "all";

  switch (command) {
    case "check":
      await runCheck();
      break;
    case "build":
      await runBuild();
      break;
    case "test":
      await runTest();
      break;
    case "prepublish":
      await runPrepublish();
      break;
    case "publish":
      await runPublish();
      break;
    case "all":
      await runCheck();
      await runBuild();
      await runTest();
      await runPrepublish();
      break;
    default:
      console.error(`Unknown CI command: ${command}`);
      process.exit(1);
  }
}

main().catch((err) => {
  console.error("Fatal error in CI execution:", err);
  process.exit(1);
});
