import { NapiCli, createBuildCommand, readNapiConfig } from "@napi-rs/cli";

async function run() {
  const args = process.argv.slice(2);
  const buildCommand = createBuildCommand(args);
  const cli = new NapiCli();
  const options = buildCommand.getOptions();

  let cargoOptions = buildCommand.cargoOptions ?? [];

  const featureIdx = args.findIndex((a) => a === "--features" || a === "-F");
  if (featureIdx !== -1 && args[featureIdx + 1] && !cargoOptions.includes("--features")) {
    cargoOptions = ["--features", args[featureIdx + 1], ...cargoOptions];
  }

  const isAll = args.includes("--all") || args.includes("--all-targets");

  if (isAll) {
    const config = await readNapiConfig(process.cwd() + "/package.json");
    const targets = config?.targets ?? [];
    for (const target of targets) {
      const targetTriple =
        typeof target === "string"
          ? target
          : typeof target === "object" &&
              target !== null &&
              "triple" in target &&
              typeof target.triple === "string"
            ? target.triple
            : JSON.stringify(target);
      console.log(`Building target: ${targetTriple}`);
      await cli.build({
        ...options,
        platform: options.platform ?? true,
        esm: options.esm ?? true,
        cargoOptions,
        target: targetTriple,
        useNapiCross: true,
        outputDir: options.outputDir ?? "build",
      });
    }
  } else {
    const buildOptions = {
      ...options,
      platform: options.platform ?? true,
      esm: options.esm ?? true,
      cargoOptions,
      outputDir: options.outputDir ?? "build",
    };
    await cli.build(buildOptions);
  }
}

void run().catch((err) => {
  console.error(err);
  process.exit(1);
});
