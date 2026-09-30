import { createBuildCommand, NapiCli } from "@napi-rs/cli";

const build = createBuildCommand(process.argv.slice(2));
const options = build.getOptions();
options.outputDir = options.outputDir ?? "build";
const cli = new NapiCli();

const { task } = await cli.build({
  ...options,
  cargoOptions: build.cargoOptions,
});

await task;
