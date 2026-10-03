import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { createBuildCommand, NapiCli } from "@napi-rs/cli";
import { disableCache, getEnv, ignoreInput, ignoreOutput } from "@voidzero-dev/vite-task-client";

// Register cache dependencies and ignores for Vite Task runner
ignoreInput(".xwin");
ignoreOutput("target");
ignoreOutput("dist");

const DEFAULT_TARGET_MATRIX = [
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-pc-windows-msvc",
  "i686-pc-windows-msvc",
  "aarch64-pc-windows-msvc",
  "x86_64-unknown-linux-gnu",
  "aarch64-unknown-linux-gnu",
  "armv7-unknown-linux-gnueabihf",
  "x86_64-unknown-linux-musl",
  "aarch64-unknown-linux-musl",
  "aarch64-linux-android",
  "armv7-linux-androideabi",
  "x86_64-unknown-freebsd",
  "wasm32-wasip1-threads",
];

function parseArgs() {
  const args = process.argv.slice(2);

  const buildAll =
    args.includes("--target-all") ||
    args.includes("--all") ||
    args.includes("--use-cross") ||
    args.includes("--cross") ||
    getEnv("CROSS_BUILD") === "true";

  const isRelease =
    args.includes("--release") || args.includes("-r") || getEnv("RELEASE") === "true";
  const dryRun = args.includes("--dry-run") || getEnv("DRY_RUN") === "true";

  const useCross = args.includes("--use-cross") || args.includes("--cross");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("-x") || args.includes("--cross-compile");

  let targetFilter = null;
  const targetIdx = args.indexOf("--target");
  if (targetIdx !== -1 && args[targetIdx + 1]) {
    targetFilter = args[targetIdx + 1];
  } else if (getEnv("TARGET")) {
    targetFilter = getEnv("TARGET");
  }

  const filteredArgs = args.filter(
    (arg) =>
      arg !== "--target-all" &&
      arg !== "--all" &&
      arg !== "--use-cross" &&
      arg !== "--cross" &&
      arg !== "--dry-run",
  );

  return {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
    filteredArgs,
  };
}

function getTargetsFromPackageJson() {
  try {
    const pkgPath = path.resolve(process.cwd(), "package.json");
    const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
    if (pkg.napi && Array.isArray(pkg.napi.targets) && pkg.napi.targets.length > 0) {
      return pkg.napi.targets;
    }
  } catch {
    // fallback
  }
  return DEFAULT_TARGET_MATRIX;
}

async function runBuild() {
  const {
    targetFilter,
    isRelease,
    dryRun,
    buildAll,
    useCross,
    useNapiCross,
    crossCompile,
    filteredArgs,
  } = parseArgs();

  const buildCommand = createBuildCommand(filteredArgs);
  const options = buildCommand.getOptions();
  const cli = new NapiCli();

  const pkgTargets = getTargetsFromPackageJson();
  let targetsToBuild = [];

  if (targetFilter) {
    targetsToBuild = [targetFilter];
  } else if (buildAll) {
    targetsToBuild = pkgTargets;
  } else {
    targetsToBuild = [null];
  }

  console.log(`\n🚀 Starting Local Build Pipeline (${targetsToBuild.length} target(s))...`);

  for (const target of targetsToBuild) {
    const isNapiCrossTarget =
      target &&
      (target.includes("gnueabihf") ||
        target.includes("powerpc") ||
        target.includes("s390x") ||
        target.includes("unknown-linux-gnu"));

    const effectiveUseNapiCross = useNapiCross || (target ? isNapiCrossTarget : false);
    const effectiveCrossCompile = crossCompile || (target ? !isNapiCrossTarget : false);

    console.log(`\n⚙️  Building target: ${target || "default host"}`);
    if (dryRun) {
      console.log(`   [Dry Run] Skipped build execution for ${target || "default host"}.`);
      continue;
    }

    try {
      const buildOpts = {
        ...options,
        cwd: options.cwd || process.cwd(),
        outputDir: "dist",
        release: isRelease,
        cargoOptions: buildCommand.cargoOptions,
      };

      if (target) {
        buildOpts.target = target;
      }
      if (useCross) buildOpts.useCross = true;
      if (effectiveUseNapiCross) buildOpts.useNapiCross = true;
      if (effectiveCrossCompile) buildOpts.crossCompile = true;

      await cli.build(buildOpts);

      if (fs.existsSync("dist")) {
        for (const file of fs.readdirSync("dist")) {
          if (file.endsWith(".node")) {
            fs.copyFileSync(path.join("dist", file), file);
          }
        }
      }
      console.log(`✅ Target ${target || "default host"} built successfully.`);
    } catch (err) {
      console.error(`❌ Target ${target || "default host"} failed to build:`, err.message);
      disableCache();
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  console.log(`\n✨ Build process complete!`);
}

runBuild().catch((err) => {
  console.error("Fatal error during build:", err);
  disableCache();
  process.exit(1);
});
