import fs from "node:fs/promises";
import path from "node:path";
import { NapiCli } from "@napi-rs/cli";

const origMkdtemp = fs.mkdtemp;
fs.mkdtemp = function (prefix, options) {
  if (typeof prefix === "string" && prefix.startsWith("/.")) {
    prefix = path.join("/tmp", prefix.slice(1));
  }
  return origMkdtemp.call(this, prefix, options);
};

async function run() {
  const args = process.argv.slice(2);
  const isRelease = args.includes("--release") || args.includes("-r");
  const useNapiCross = args.includes("--use-napi-cross");
  const crossCompile = args.includes("--cross-compile") || args.includes("-x");
  const useCross = args.includes("--use-cross");

  const targetIdx = args.findIndex((a) => a === "--target" || a === "-t");
  const target = targetIdx !== -1 && args[targetIdx + 1] ? args[targetIdx + 1] : undefined;

  const cli = new NapiCli();
  await cli.build({
    platform: true,
    esm: true,
    release: isRelease,
    target,
    useNapiCross,
    crossCompile,
    useCross,
  });
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
