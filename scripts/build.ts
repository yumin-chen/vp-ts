import { NapiCli } from "@napi-rs/cli";

export interface CrossBuildOptions {
  target?: string;
  release?: boolean;
  platform?: boolean;
  useNapiCross?: boolean;
  crossCompile?: boolean;
  useCross?: boolean;
  outputDir?: string;
  cwd?: string;
}

/**
 * Executes cross-compilation build for NAPI-RS native addon using NapiCli.
 */
export async function buildAddon(options: CrossBuildOptions = {}) {
  const cli = new NapiCli();

  const buildOptions = {
    platform: true,
    release: true,
    ...options,
  };

  return await cli.build(buildOptions);
}

// Allow direct execution from CLI/scripts
if (process.argv[1]?.includes("build")) {
  const args = process.argv.slice(2);
  const options: CrossBuildOptions = {
    platform: true,
    release: true,
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (arg === "--target" || arg === "-t") {
      options.target = args[++i];
    } else if (arg === "--use-napi-cross") {
      options.useNapiCross = true;
    } else if (arg === "--cross-compile" || arg === "-x") {
      options.crossCompile = true;
    } else if (arg === "--use-cross") {
      options.useCross = true;
    } else if (arg === "--debug") {
      options.release = false;
    } else if (arg === "--output-dir" || arg === "-o") {
      options.outputDir = args[++i];
    }
  }

  buildAddon(options).catch((err) => {
    console.error("Cross build failed:", err);
    process.exit(1);
  });
}
