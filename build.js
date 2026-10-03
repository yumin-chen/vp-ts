import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { NapiCli } from "@napi-rs/cli";
import { disableCache, getEnv, ignoreInput, ignoreOutput } from "@voidzero-dev/vite-task-client";

// Register cache dependencies and ignores for Vite Task runner
ignoreInput(".xwin");
ignoreOutput("target");
ignoreOutput("dist");

function getTargetFlags(target) {
  if (
    target.includes("unknown-linux-gnu") ||
    target.includes("gnueabihf") ||
    target.includes("powerpc64le") ||
    target.includes("s390x")
  ) {
    return { useNapiCross: true, crossCompile: false };
  }
  return { useNapiCross: false, crossCompile: true };
}

function parseArgs() {
  const args = process.argv.slice(2);
  const isRelease =
    args.includes("--release") || args.includes("-r") || getEnv("RELEASE") === "true";
  const isDryRun = args.includes("--dry-run") || getEnv("DRY_RUN") === "true";

  const isCrossAll =
    args.includes("--use-cross") ||
    args.includes("--cross") ||
    args.includes("--target-all") ||
    getEnv("CROSS_BUILD") === "true";

  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const targetFilter =
    targetIdx !== -1 && args[targetIdx + 1] ? args[targetIdx + 1] : getEnv("TARGET");

  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("--cross-compile") || args.includes("-x");
  const useCross = args.includes("--use-cross");

  return {
    isRelease,
    isDryRun,
    isCrossAll,
    targetFilter,
    useNapiCross,
    crossCompile,
    useCross,
  };
}

function getTargetsFromPackageJson() {
  try {
    const pkg = JSON.parse(fs.readFileSync("package.json", "utf8"));
    return pkg.napi && Array.isArray(pkg.napi.targets) ? pkg.napi.targets : [];
  } catch {
    return [];
  }
}

async function buildSingleTarget(target, options) {
  const { isRelease, useNapiCross, crossCompile, useCross, isDryRun } = options;
  const targetFlags = getTargetFlags(target);

  const finalNapiCross = useNapiCross || targetFlags.useNapiCross;
  const finalCrossCompile = crossCompile || targetFlags.crossCompile;

  console.log(`\n⚙️  Building target: ${target}`);
  console.log(
    `   Flags: release=${isRelease}, napiCross=${finalNapiCross}, crossCompile=${finalCrossCompile}`,
  );

  if (isDryRun) {
    console.log(`   [Dry Run] Skipped execution for ${target}.`);
    return;
  }

  const cli = new NapiCli();
  await cli.build({
    platform: true,
    esm: true,
    release: isRelease,
    target,
    useNapiCross: finalNapiCross,
    crossCompile: finalCrossCompile,
    useCross,
    outputDir: "dist",
  });

  if (fs.existsSync("dist")) {
    for (const file of fs.readdirSync("dist")) {
      if (file.endsWith(".node")) {
        fs.copyFileSync(path.join("dist", file), file);
      }
    }
  }
  console.log(`✅ Target ${target} built successfully.`);
}

async function run() {
  const options = parseArgs();
  const allTargets = getTargetsFromPackageJson();

  if (options.isCrossAll) {
    console.log(
      `\n🚀 Starting Local Cross-Build Pipeline for all ${allTargets.length} package.json target(s)...`,
    );
    const targetsToBuild = options.targetFilter
      ? allTargets.filter((t) => t === options.targetFilter)
      : allTargets;

    for (const target of targetsToBuild) {
      try {
        await buildSingleTarget(target, options);
      } catch (err) {
        console.error(`❌ Target ${target} failed to build:`, err.message);
        disableCache();
        if (!options.isDryRun) process.exit(1);
      }
    }
    console.log(`\n✨ Cross-build pipeline finished!`);
    return;
  }

  if (options.targetFilter) {
    await buildSingleTarget(options.targetFilter, options);
    return;
  }

  // Single target default host build
  const cli = new NapiCli();
  if (!options.isDryRun) {
    await cli.build({
      platform: true,
      esm: true,
      release: options.isRelease,
      target: undefined,
      useNapiCross: options.useNapiCross,
      crossCompile: options.crossCompile,
      useCross: options.useCross,
      outputDir: "dist",
    });

    if (fs.existsSync("dist")) {
      for (const file of fs.readdirSync("dist")) {
        if (file.endsWith(".node")) {
          fs.copyFileSync(path.join("dist", file), file);
        }
      }
    }
  } else {
    console.log(`[Dry Run] Default host build skipped.`);
  }
}

void run().catch((err) => {
  console.error("Fatal build error:", err);
  disableCache();
  process.exit(1);
});
