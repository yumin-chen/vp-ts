import { NapiCli } from "@napi-rs/cli";
import { readFileSync, writeFileSync, copyFileSync, existsSync } from "node:fs";

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
    outputDir: "native",
  });

  const soPath = "./target/x86_64-unknown-linux-gnu/debug/libmy_addon_native.so";
  if (existsSync(soPath)) {
    copyFileSync(soPath, "./native/my-addon.linux-x64-gnu.node");
    copyFileSync(soPath, "./native/my-addon.node");
  }

  const indexPath = "./native/index.js";
  if (existsSync(indexPath)) {
    let content = readFileSync(indexPath, "utf-8");
    content = content.replaceAll(
      "{ add }",
      "{ JsContainer, JsContainerHandle, JsContainerRestOptions, JsGetOrCreateResult, JsImageHandle, JsVolumeHandle }",
    );
    writeFileSync(indexPath, content, "utf-8");
  }
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
