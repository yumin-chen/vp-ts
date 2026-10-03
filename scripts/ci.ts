import { execSync } from "node:child_process";
import console from "node:console";
import process from "node:process";

function run(cmd: string) {
  console.log(`\n> ${cmd}`);
  execSync(cmd, { stdio: "inherit" });
}

try {
  console.log("=== Running Automated Local CI Checks ===");
  run("npm run build");
  run("npm test");
  run("npm run check");
  run("npm run lint");
  console.log("\n✔ All Local CI checks completed successfully!");
} catch (error) {
  console.error("\n❌ CI Check failed:", error);
  process.exit(1);
}
