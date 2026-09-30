const fs = require('node:fs')

const cliPath = 'node_modules/@napi-rs/cli/dist/cli.js'
if (fs.existsSync(cliPath)) {
  let content = fs.readFileSync(cliPath, 'utf8')
  content = content.replace(
    'dirname(finalOutputDir)',
    '(dirname(finalOutputDir) === "/" ? finalOutputDir : dirname(finalOutputDir))'
  )
  fs.writeFileSync(cliPath, content)
}
