import fs from 'node:fs';

for (const file of [
  'node_modules/@napi-rs/cli/dist/cli.js',
  'node_modules/@napi-rs/cli/dist/index.js'
]) {
  if (fs.existsSync(file)) {
    let content = fs.readFileSync(file, 'utf8');
    if (content.includes("join(dirname(finalOutputDir)")) {
      content = content.replaceAll(
        "join(dirname(finalOutputDir)",
        "join(dirname(finalOutputDir) === '/' ? finalOutputDir : dirname(finalOutputDir)"
      );
      fs.writeFileSync(file, content);
    }
  }
}
