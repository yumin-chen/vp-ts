#!/usr/bin/env node

import { readFileSync } from "node:fs";
import process from "node:process";
import { NapiCli } from "@napi-rs/cli";

const pkg = JSON.parse(readFileSync(new URL("./package.json", import.meta.url), "utf8"));
const targets = pkg.napi?.targets || [];

function parseArgs() {
  const args = process.argv.slice(2);
  let targetFilter = null;

  const targetIdx = args.indexOf("--target");
  if (targetIdx !== -1 && args[targetIdx + 1]) {
    targetFilter = args[targetIdx + 1];
  } else {
    const targetEq = args.find((a) => a.startsWith("--target="));
    if (targetEq) {
      targetFilter = targetEq.split("=")[1];
    }
  }

  const release = args.includes("--release") || args.includes("-r");
  const dryRun = args.includes("--dry-run");
  const useCrossFlag = args.includes("--use-cross");
  const crossCompileFlag = args.includes("--cross-compile") || args.includes("-x");
  const useNapiCrossFlag = args.includes("--use-napi-cross");
  const targetAllFlag = args.includes("--target-all") || args.includes("--all");

  const buildAll = targetAllFlag || (useCrossFlag && !targetFilter);

  return {
    targetFilter,
    release,
    dryRun,
    buildAll,
    useCrossFlag,
    crossCompileFlag,
    useNapiCrossFlag,
  };
}

async function runCrossBuild() {
  const {
    targetFilter,
    release,
    dryRun,
    buildAll,
    useCrossFlag,
    crossCompileFlag,
    useNapiCrossFlag,
  } = parseArgs();

  const cli = new NapiCli();

  let targetsToBuild = [];

  if (targetFilter) {
    targetsToBuild = [targetFilter];
  } else if (buildAll) {
    targetsToBuild = targets;
  } else {
    // Default local build for current host platform
    targetsToBuild = [undefined];
  }

  console.log(`\n🚀 Starting Local Build Pipeline (${targetsToBuild.length} target(s))...`);

  for (const target of targetsToBuild) {
    let useNapiCross = false;
    let crossCompile = false;
    let useCross = false;
    let flagStr = "";

    if (target) {
      const isLinuxGlibc =
        target.includes("unknown-linux-gnu") || target.includes("linux-gnueabihf");

      if (useNapiCrossFlag || (isLinuxGlibc && !useCrossFlag && !crossCompileFlag)) {
        useNapiCross = true;
        flagStr = " --use-napi-cross";
      } else if (useCrossFlag) {
        useCross = true;
        flagStr = " --use-cross";
      } else {
        crossCompile = true;
        flagStr = " -x";
      }
    }

    const cmd =
      `node build.mjs${target ? ` --target ${target}` : ""}${flagStr}${release ? " --release" : ""}`.trim();
    console.log(`\n⚙️  Building target: ${target || "host"}`);
    console.log(`   Command: ${cmd}`);

    if (dryRun) {
      console.log(`   [Dry Run] Skipped execution.`);
      continue;
    }

    try {
      const { task } = await cli.build({
        platform: true,
        esm: true,
        release,
        target,
        outputDir: "./dist",
        useNapiCross,
        crossCompile,
        useCross,
      });
      await task;
      console.log(`✅ Target ${target || "host"} built successfully.`);
    } catch (err) {
      console.error(`❌ Target ${target || "host"} failed to build:`, err.message);
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  console.log(`\n✨ Build process complete!`);
}

runCrossBuild().catch((err) => {
  console.error("Fatal error during build:", err);
  process.exit(1);
});
