import fs from 'node:fs'
import path from 'node:path'
import { NapiCli } from '@napi-rs/cli'

export async function build(options = {}) {
  const cli = new NapiCli()
  const outputDir = options.outputDir || 'npm'

  if (!fs.existsSync('dist')) {
    fs.mkdirSync('dist', { recursive: true })
  }

  // If useCross or cross flag is set and no specific target, read targets from package.json
  if ((options.useCross || options.cross) && !options.target) {
    const pkgJson = JSON.parse(fs.readFileSync('package.json', 'utf-8'))
    const targets = pkgJson.napi?.targets || []
    console.log(`Cross-building for ${targets.length} targets:`, targets.join(', '))

    for (const target of targets) {
      console.log(`Building target: ${target}...`)
      try {
        await cli.build({
          platform: true,
          esm: true,
          outputDir,
          ...options,
          target,
          useCross: false,
        })
      } catch (err) {
        console.warn(`Warning: failed to build target ${target}:`, err.message || err)
      }
    }
  } else {
    await cli.build({
      platform: true,
      esm: true,
      outputDir,
      ...options,
    })
  }

  // Copy built .node, .wasm, .cjs, and .mjs files from npm/ to dist/ and root directory
  if (fs.existsSync('npm')) {
    const files = fs.readdirSync('npm')
    for (const file of files) {
      const ext = path.extname(file)
      if (
        ext === '.node' ||
        ext === '.wasm' ||
        ext === '.cjs' ||
        ext === '.mjs' ||
        file.endsWith('.d.ts') ||
        file.endsWith('.d.cts')
      ) {
        const srcPath = path.join('npm', file)
        if (fs.statSync(srcPath).isFile()) {
          fs.copyFileSync(srcPath, path.join('.', file))
          fs.copyFileSync(srcPath, path.join('dist', file))
        }
      }
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
  const useCross = args.includes('--use-cross') || args.includes('--cross')

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
