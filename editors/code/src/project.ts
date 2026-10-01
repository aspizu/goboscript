import { existsSync } from "node:fs"
import { dirname, join } from "node:path"

export function findProjectRoot(file: string): string | undefined {
  let dir = dirname(file)
  for (;;) {
    if (existsSync(join(dir, "stage.gs")) || existsSync(join(dir, "goboscript.toml"))) {
      return dir
    }
    const parent = dirname(dir)
    if (parent === dir) {
      return undefined
    }
    dir = parent
  }
}
