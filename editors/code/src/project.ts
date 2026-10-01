import { existsSync } from "node:fs"
import { dirname, join } from "node:path"

/**
 * Walks up from `file` and returns the closest directory containing a goboscript
 * project (`stage.gs` or `goboscript.toml`), or `undefined` if there is none.
 */
export function findProjectRoot(file: string): string | undefined {
  let dir = dirname(file)
  for (;;) {
    if (existsSync(join(dir, "stage.gs")) || existsSync(join(dir, "goboscript.toml"))) {
      return dir
    }
    const parent = dirname(dir)
    if (parent === dir) return undefined
    dir = parent
  }
}
