const esbuild = require("esbuild")

const watch = process.argv.includes("--watch")

/** @type {import("esbuild").BuildOptions} */
const extensionOptions = {
  bundle: true,
  platform: "node",
  target: "node24",
  format: "cjs",
  sourcemap: true,
  external: ["vscode"],
  logLevel: "info",
}

// The `.sb3` preview webview. Scaffolding ships as a single prebuilt
// browser bundle, so this is an inline of ~4 MB, not a real transform.
/** @type {import("esbuild").BuildOptions} */
const previewOptions = {
  bundle: true,
  platform: "browser",
  target: "es2022",
  format: "iife",
  sourcemap: false,
  minify: false,
  logLevel: "info",
}

async function main() {
  if (watch) {
    const context = await esbuild.context({
      ...extensionOptions,
      entryPoints: ["src/extension.ts"],
      outfile: "dist/extension.js",
      minify: false,
    })
    await context.watch()
    return
  }
  await esbuild.build({
    ...extensionOptions,
    entryPoints: ["src/extension.ts"],
    outfile: "dist/extension.js",
    minify: true,
  })
  await esbuild.build({
    ...previewOptions,
    entryPoints: ["src/preview/webview/main.ts"],
    outfile: "dist/preview.js",
  })
}

main().catch((error) => {
  console.error(error)
  process.exit(1)
})
