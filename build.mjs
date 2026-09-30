import fs from 'node:fs'
import path from 'node:path'

// Redirect staging directory before importing @napi-rs/cli
const origMkdtemp = fs.promises.mkdtemp
fs.promises.mkdtemp = function (prefix, options) {
  if (typeof prefix === 'string' && prefix.startsWith('/.')) {
    prefix = path.join('/tmp', path.basename(prefix))
  }
  return origMkdtemp.call(this, prefix, options)
}

const { NapiCli } = await import('@napi-rs/cli')

export async function build(options = {}) {
  const cli = new NapiCli()
  return await cli.build({
    platform: true,
    esm: true,
    outputDir: '.',
    ...options,
  })
}

// If run directly from CLI
if (process.argv[1] && process.argv[1].endsWith('build.mjs')) {
  const args = process.argv.slice(2)
  const isRelease = args.includes('--release') || args.includes('-r')
  const targetIndex =
    args.indexOf('--target') !== -1 ? args.indexOf('--target') : args.indexOf('-t')
  const target = targetIndex !== -1 ? args[targetIndex + 1] : undefined
  const useNapiCross = args.includes('--use-napi-cross')
  const crossCompile = args.includes('--cross-compile') || args.includes('-x')
  const useCross = args.includes('--use-cross')

  build({
    release: isRelease,
    target,
    useNapiCross,
    crossCompile,
    useCross,
  }).catch((err) => {
    console.error('Build failed:', err)
    process.exit(1)
  })
}
