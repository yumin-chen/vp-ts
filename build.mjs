import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { createBuildCommand, NapiCli } from "@napi-rs/cli";

const pkgPath = path.resolve(process.cwd(), "package.json");
const pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
const pkgNapiTargets = pkg.napi?.targets || [
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

  const buildAll = args.includes("--target-all") || args.includes("--all");
  const isRelease = args.includes("--release") || args.includes("-r");
  const dryRun = args.includes("--dry-run");

  const useCross = args.includes("--use-cross");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("-x") || args.includes("--cross-compile");

  let targetFilter = null;
  const targetIdx = args.indexOf("--target");
  if (targetIdx !== -1 && args[targetIdx + 1] && !args[targetIdx + 1].startsWith("-")) {
    targetFilter = args[targetIdx + 1];
  } else {
    const shortTargetIdx = args.indexOf("-t");
    if (
      shortTargetIdx !== -1 &&
      args[shortTargetIdx + 1] &&
      !args[shortTargetIdx + 1].startsWith("-")
    ) {
      targetFilter = args[shortTargetIdx + 1];
    }
  }

  const filteredArgs = args.filter(
    (arg) =>
      arg !== "--target-all" &&
      arg !== "--all" &&
      arg !== "--dry-run" &&
      arg !== "--target" &&
      arg !== "-t" &&
      arg !== targetFilter,
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

  let targetsToBuild = [];

  if (targetFilter) {
    targetsToBuild = [targetFilter];
  } else if (buildAll) {
    targetsToBuild = pkgNapiTargets;
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
        target.includes("linux-gnu"));
    const effectiveUseNapiCross = useNapiCross || (target ? isNapiCrossTarget : false);
    const effectiveCrossCompile = crossCompile || (target ? !isNapiCrossTarget : false);

    console.log(`\n⚙️  Building target: ${target || "default host"}`);
    if (dryRun) {
      const activeFlags = [
        effectiveUseNapiCross ? "--use-napi-cross" : "",
        effectiveCrossCompile ? "-x" : "",
        useCross ? "--use-cross" : "",
        isRelease ? "--release" : "",
      ]
        .filter(Boolean)
        .join(" ");
      console.log(
        `   [Dry Run] napi build ${target ? `--target ${target}` : ""} ${activeFlags}`.trim(),
      );
      continue;
    }

    try {
      const buildOpts = {
        ...options,
        cwd: options.cwd || process.cwd(),
        outputDir: options.outputDir || "./dist",
        platform: true,
        release: isRelease,
        cargoOptions: buildCommand.cargoOptions,
      };

      if (target) {
        buildOpts.target = target;
      }
      if (useCross) buildOpts.useCross = true;
      if (effectiveUseNapiCross) buildOpts.useNapiCross = true;
      if (effectiveCrossCompile) buildOpts.crossCompile = true;

      const { task } = await cli.build(buildOpts);
      if (task) {
        await task;
      }
      console.log(`✅ Target ${target || "default host"} built successfully.`);
    } catch (err) {
      console.error(`❌ Target ${target || "default host"} failed to build:`, err.message || err);
      if (!buildAll) {
        process.exit(1);
      }
    }
  }

  console.log(`\n✨ Build process complete!`);
}

runBuild().catch((err) => {
  console.error("Fatal error during build:", err);
  process.exit(1);
});
