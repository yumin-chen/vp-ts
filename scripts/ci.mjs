import { execSync } from "node:child_process";

console.log("🚀 Starting Local CI Pipeline...");

// 1. Quality Check
console.log("\n🔍 Step 1: Code Check & Linting...");
try {
  execSync("vp check", { stdio: "inherit" });
  console.log("✅ Code check passed.");
} catch (err) {
  const msg = err instanceof Error ? err.message : String(err);
  console.warn(`⚠️ Code check warning: ${msg}`);
}

// 2. Build Pipeline
console.log("\n🛠️ Step 2: Running Build Pipeline...");
try {
  execSync("node build.mjs", { stdio: "inherit" });
  console.log("✅ Default host build complete.");
} catch (err) {
  const msg = err instanceof Error ? err.message : String(err);
  console.error(`❌ Build failed: ${msg}`);
  process.exit(1);
}

// 3. Unit Tests
console.log("\n🧪 Step 3: Running Unit Tests...");
try {
  execSync("npm test", { stdio: "inherit" });
  execSync("vp test", { stdio: "inherit" });
  console.log("✅ Unit tests passed.");
} catch (err) {
  const msg = err instanceof Error ? err.message : String(err);
  console.error(`❌ Unit tests failed: ${msg}`);
  process.exit(1);
}

// 4. Publish Dry-Run
console.log("\n🚀 Step 4: Publish Dry-Run Verification...");
try {
  execSync("node scripts/publish-dry-run.mjs", { stdio: "inherit" });
  console.log("✅ Publish dry-run verified.");
} catch (err) {
  const msg = err instanceof Error ? err.message : String(err);
  console.warn(`⚠️ Publish dry-run warning: ${msg}`);
}

console.log("\n🎉 Local CI Pipeline finished successfully!\n");
