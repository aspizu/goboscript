import { existsSync } from "node:fs"
import { delimiter, extname, isAbsolute, join, resolve } from "node:path"

export type CompilerFailure = "not-found-on-path" | "relative-path"

export type CompilerResolution =
  | { command: string; failure: undefined }
  | { command: undefined; failure: CompilerFailure }

export function resolveCompiler(configured: string, cwd: string): CompilerResolution {
  if (configured !== "") {
    if (!isAbsolute(configured)) {
      return { command: undefined, failure: "relative-path" }
    }
    return { command: withExeExtension(configured), failure: undefined }
  }
  if (process.platform !== "win32") {
    return { command: "goboscript", failure: undefined }
  }
  const command = searchPath("goboscript.exe", cwd)
  if (command === undefined) {
    return { command: undefined, failure: "not-found-on-path" }
  }
  return { command, failure: undefined }
}

function withExeExtension(path: string): string {
  if (process.platform !== "win32" || extname(path) !== "") {
    return path
  }
  return existsSync(`${path}.exe`) ? `${path}.exe` : path
}

function searchPath(name: string, cwd: string): string | undefined {
  for (const dir of (process.env.PATH ?? "").split(delimiter)) {
    if (dir === "" || isSamePath(resolve(dir), resolve(cwd))) {
      continue
    }
    const candidate = join(dir, name)
    if (existsSync(candidate)) {
      return candidate
    }
  }
  return undefined
}

function isSamePath(a: string, b: string): boolean {
  if (process.platform === "win32") {
    return a.toLowerCase() === b.toLowerCase()
  }
  return a === b
}
