import type { BuildOptions } from "esbuild"
import { build, context } from "esbuild"
import tailwindPlugin from "esbuild-plugin-tailwindcss"

const watch = process.argv.includes("--watch")

const extensionOptions: BuildOptions = {
  bundle: true,
  platform: "node",
  target: "node24",
  format: "cjs",
  sourcemap: true,
  external: ["vscode"],
  loader: { ".html": "text", ".css": "text" },
  logLevel: "info",
}

// The `.sb3` preview webview. Scaffolding ships as a single prebuilt
// browser bundle, so this is an inline of ~4 MB, not a real transform.
const previewOptions: BuildOptions = {
  bundle: true,
  platform: "browser",
  target: "es2022",
  format: "iife",
  sourcemap: false,
  minify: false,
  logLevel: "info",
  plugins: [tailwindPlugin()],
}

async function main(): Promise<void> {
  if (watch) {
    const ctx = await context({
      ...extensionOptions,
      entryPoints: ["src/extension.ts"],
      outfile: "dist/extension.js",
      minify: false,
    })
    await ctx.watch()
    return
  }
  await build({
    ...extensionOptions,
    entryPoints: ["src/extension.ts"],
    outfile: "dist/extension.js",
    minify: true,
  })
  await build({
    ...previewOptions,
    entryPoints: ["src/preview/webview/main.ts"],
    outfile: "dist/preview.js",
  })
}

main().catch((error) => {
  console.error(error)
  process.exit(1)
})
