import { NapiCli } from "@napi-rs/cli";
import fs from "node:fs";
import path from "node:path";

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
    outputDir: "dist",
  });

  if (fs.existsSync("dist")) {
    const files = fs.readdirSync("dist");
    console.log("Dist files:", files);
    for (const file of files) {
      if (file.endsWith(".node") || file === "index.js" || file === "index.d.ts") {
        console.log("Copying", file, "to root");
        fs.copyFileSync(path.join("dist", file), file);
      }
    }
  }
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
