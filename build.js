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
    outputDir: "dist",
    esm: true,
    release: isRelease,
    target,
    useNapiCross,
    crossCompile,
    useCross,
  });

  const distDir = path.resolve(process.cwd(), "dist");
  if (fs.existsSync(distDir)) {
    const distFiles = fs.readdirSync(distDir);
    for (const file of distFiles) {
      if (file.endsWith(".js") || file.endsWith(".d.ts") || file.endsWith(".node")) {
        const srcPath = path.join(distDir, file);
        const destPath = path.resolve(process.cwd(), file);
        fs.copyFileSync(srcPath, destPath);
        console.log(`Copied ${srcPath} -> ${destPath}`);
      }
    }
  }
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
