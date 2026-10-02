import { existsSync, readFileSync } from "node:fs"
import { dirname, isAbsolute, join, resolve } from "node:path"
import * as vscode from "vscode"
import { resolveCompiler, type CompilerFailure, type CompilerResolution } from "./compiler"
import type { ParsedDiagnostic } from "./parser"
import { Sb3PreviewProvider } from "./preview/provider"
import { ProjectRunner, type Crash } from "./runner"

const DEBOUNCE_MS = 250
const BUILD_TIMEOUT_MS = 60_000

export function activate(context: vscode.ExtensionContext): void {
  const collection = vscode.languages.createDiagnosticCollection("goboscript")
  const runners = new Map<string, ProjectRunner>()
  const reportedUris = new Map<string, Set<string>>()
  const notified = new Set<CompilerFailure>()
  const projectRoots = new Map<string, string>()
  const compilerResolutions = new Map<string, CompilerResolution>()

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration("goboscript.compilerPath")) {
        notified.clear()
        compilerResolutions.clear()
      }
    }),
    vscode.window.registerCustomEditorProvider(
      "goboscript.sb3Preview",
      new Sb3PreviewProvider(context.extensionUri),
      { supportsMultipleEditorsPerDocument: false },
    ),
    collection,
    vscode.workspace.onDidSaveTextDocument((document) => {
      if (document.languageId !== "goboscript" || document.uri.scheme !== "file") {
        return
      }
      const projectDir = lookupProjectRoot(projectRoots, document.uri.fsPath)
      if (projectDir === undefined) {
        return
      }
      let runner = runners.get(projectDir)
      if (runner === undefined) {
        runner = new ProjectRunner(projectDir, {
          debounceMs: DEBOUNCE_MS,
          timeoutMs: BUILD_TIMEOUT_MS,
          resolveCompiler: () => resolveCompilerCached(compilerResolutions, projectDir),
          onResult: ({ parsed, savedDoc, stderr, error, failure, crash }) => {
            if (error !== undefined) {
              console.error(`goboscript ${projectDir}: ${error}`)
            }
            if (failure !== undefined) {
              notifyFailure(notified, failure, error)
              return
            }
            if (crash !== undefined) {
              console.error(`goboscript ${projectDir} ${crashCause(crash)}\n${stderr}`)
              notifyCrash(crash)
            }
            publish(collection, reportedUris, projectDir, parsed, savedDoc)
          },
        })
        runners.set(projectDir, runner)
        context.subscriptions.push(runner)
      }
      runner.schedule(document.uri.fsPath)
    }),
  )
}

export function deactivate(): void {}

const FAILURE_MESSAGES: Record<CompilerFailure, string> = {
  "not-found-on-path":
    "goboscript was not found on PATH. Set goboscript.compilerPath to an absolute path.",
  "relative-path": "goboscript.compilerPath must be an absolute path.",
  "spawn-failed": "Could not start the compiler. Check goboscript.compilerPath and permissions.",
}

function notifyFailure(
  notified: Set<CompilerFailure>,
  failure: CompilerFailure,
  detail?: string,
): void {
  if (notified.has(failure)) {
    return
  }
  notified.add(failure)
  const message =
    detail === undefined ? FAILURE_MESSAGES[failure] : `${FAILURE_MESSAGES[failure]} ${detail}`
  void vscode.window.showErrorMessage(message, "Configure").then((choice) => {
    if (choice === "Configure") {
      void vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "goboscript.compilerPath",
      )
    }
  })
}

const ISSUE_URL = "https://github.com/aspizu/goboscript/issues"

function crashCause(crash: Crash): string {
  switch (crash.kind) {
    case "timeout": {
      return `timed out after ${BUILD_TIMEOUT_MS / 1000}s`
    }
    case "signal": {
      return `crashed (${crash.signal})`
    }
    case "exit": {
      return `exited with code ${crash.code} and reported nothing`
    }
  }
}

function notifyCrash(crash: Crash): void {
  void vscode.window
    .showErrorMessage(`goboscript is cooked 💀: ${crashCause(crash)}`, "Open Issue")
    .then((choice) => {
      if (choice === "Open Issue") {
        void vscode.env.openExternal(vscode.Uri.parse(ISSUE_URL))
      }
    })
}

function lookupProjectRoot(roots: Map<string, string>, file: string): string | undefined {
  const cached = roots.get(file)
  if (cached !== undefined) {
    return cached
  }
  const root = findProjectRoot(file)
  if (root !== undefined) {
    roots.set(file, root)
  }
  return root
}

function resolveCompilerCached(
  resolutions: Map<string, CompilerResolution>,
  projectDir: string,
): CompilerResolution {
  const cached = resolutions.get(projectDir)
  if (cached !== undefined && cached.failure === undefined) {
    return cached
  }
  const resolution = resolveCompiler(
    vscode.workspace
      .getConfiguration("goboscript", vscode.Uri.file(projectDir))
      .get("compilerPath", ""),
    projectDir,
  )
  if (resolution.failure === undefined) {
    resolutions.set(projectDir, resolution)
  }
  return resolution
}

function findProjectRoot(file: string): string | undefined {
  let dir = dirname(file)
  while (true) {
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

function publish(
  collection: vscode.DiagnosticCollection,
  reportedUris: Map<string, Set<string>>,
  projectDir: string,
  parsed: ParsedDiagnostic[],
  savedDoc: string | undefined,
): void {
  const byUri = new Map<string, vscode.Diagnostic[]>()
  const linesByFsPath = new Map<string, string[] | null>()
  for (const item of parsed) {
    const uri = resolveUri(item, projectDir, savedDoc)
    const range = computeRange(item, uri.fsPath, linesByFsPath)
    const diagnostic = new vscode.Diagnostic(
      range,
      item.message,
      item.severity === "error"
        ? vscode.DiagnosticSeverity.Error
        : vscode.DiagnosticSeverity.Warning,
    )
    diagnostic.source = "goboscript"
    if (item.helps.length > 0) {
      diagnostic.relatedInformation = item.helps.map(
        (help) => new vscode.DiagnosticRelatedInformation(new vscode.Location(uri, range), help),
      )
    }
    const key = uri.toString()
    const list = byUri.get(key)
    if (list) {
      list.push(diagnostic)
    } else {
      byUri.set(key, [diagnostic])
    }
  }
  const previous = reportedUris.get(projectDir)
  if (previous) {
    for (const uri of previous) {
      if (!byUri.has(uri)) {
        collection.delete(vscode.Uri.parse(uri))
      }
    }
  }
  for (const [uri, diagnostics] of byUri) {
    collection.set(vscode.Uri.parse(uri), diagnostics)
  }
  reportedUris.set(projectDir, new Set(byUri.keys()))
}

function resolveUri(
  item: ParsedDiagnostic,
  projectDir: string,
  savedDoc: string | undefined,
): vscode.Uri {
  if (item.path === undefined) {
    return vscode.Uri.file(savedDoc ?? resolve(projectDir, "stage.gs"))
  }
  const path = isAbsolute(item.path) ? item.path : resolve(projectDir, item.path)
  return vscode.Uri.file(path)
}

function readLines(fsPath: string): string[] | null {
  try {
    return readFileSync(fsPath, "utf8").split(/\r?\n/)
  } catch {
    return null
  }
}

function computeRange(
  item: ParsedDiagnostic,
  fsPath: string,
  linesByFsPath: Map<string, string[] | null>,
): vscode.Range {
  if (item.line === undefined || item.column === undefined) {
    return new vscode.Range(0, 0, 0, 0)
  }
  let lines = linesByFsPath.get(fsPath)
  if (lines === undefined) {
    lines = readLines(fsPath)
    linesByFsPath.set(fsPath, lines)
  }
  if (lines === null) {
    return new vscode.Range(0, 0, 0, 0)
  }
  const line = Math.min(Math.max(item.line - 1, 0), lines.length - 1)
  const column = Math.max(item.column - 1, 0)
  const position = new vscode.Position(line, utf16ColumnOfCodePoints(lines[line], column))
  return new vscode.Range(position, position)
}

function utf16ColumnOfCodePoints(text: string, codePoints: number): number {
  let units = 0
  for (let i = 0; i < text.length && codePoints > 0; codePoints--) {
    const width = text.codePointAt(i)! > 0xffff ? 2 : 1
    units += width
    i += width
  }
  return units
}
