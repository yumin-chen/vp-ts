import { NapiCli } from '@napi-rs/cli'

import fs from 'node:fs'
import path from 'node:path'

export async function build(options = {}) {
  const cli = new NapiCli()
  const res = await cli.build({
    platform: true,
    esm: true,
    outputDir: '.napi-build',
    ...options,
  })
  if (res && res.task) {
    await res.task
  }
  if (fs.existsSync('.napi-build')) {
    const files = fs.readdirSync('.napi-build')
    for (const file of files) {
      fs.copyFileSync(path.join('.napi-build', file), path.join('.', file))
    }
  }
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
