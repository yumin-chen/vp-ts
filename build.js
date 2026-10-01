import { NapiCli } from "@napi-rs/cli";
import { existsSync, rmSync } from "node:fs";

async function run() {
  const args = process.argv.slice(2);
  const isRelease = args.includes("--release") || args.includes("-r");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("--cross-compile") || args.includes("-x");
  const useCross = args.includes("--use-cross");

  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const target = targetIdx !== -1 && args[targetIdx + 1] ? args[targetIdx + 1] : undefined;

  const nativeIndex = "./native/index.js";
  if (existsSync(nativeIndex)) {
    rmSync(nativeIndex);
  }

  const cli = new NapiCli();
  await cli.build({
    platform: true,
    esm: true,
    release: isRelease,
    target,
    useNapiCross,
    crossCompile,
    useCross,
    outputDir: "native",
  });
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
