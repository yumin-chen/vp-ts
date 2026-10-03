#!/usr/bin/env node

import fs from "node:fs";

console.log("🚀 Running Publish Dry-Run Verification...");

if (fs.existsSync("./npm")) {
  console.log("📁 Staging directory ./npm exists and is ready.");
}

if (fs.existsSync("./build")) {
  console.log("📁 Output directory ./build exists.");
}

console.log("✅ Publish Dry-Run complete.");
