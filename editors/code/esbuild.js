const esbuild = require("esbuild")

const watch = process.argv.includes("--watch")
const tests = process.argv.includes("--tests")

/** @type {import("esbuild").BuildOptions} */
const options = {
  bundle: true,
  platform: "node",
  target: "node24",
  format: "cjs",
  sourcemap: true,
  external: ["vscode"],
  logLevel: "info",
}

async function main() {
  if (watch) {
    const context = await esbuild.context({
      ...options,
      entryPoints: ["src/extension.ts"],
      outfile: "dist/extension.js",
      minify: false,
    })
    await context.watch()
    return
  }
  await esbuild.build({
    ...options,
    entryPoints: ["src/extension.ts"],
    outfile: "dist/extension.js",
    minify: true,
  })
  if (tests) {
    await esbuild.build({
      ...options,
      entryPoints: ["src/parser.test.ts"],
      outfile: "dist-test/parser.test.js",
      minify: false,
    })
  }
}

main().catch((error) => {
  console.error(error)
  process.exit(1)
})
