import { readFileSync } from "node:fs"
import { isAbsolute, resolve } from "node:path"
import * as vscode from "vscode"
import type { ParsedDiagnostic } from "./parser"
import { findProjectRoot } from "./project"
import { ProjectRunner } from "./runner"

const DEBOUNCE_MS = 250
const BUILD_TIMEOUT_MS = 60_000

export function activate(context: vscode.ExtensionContext): void {
  const channel = vscode.window.createOutputChannel("goboscript")
  const collection = vscode.languages.createDiagnosticCollection("goboscript")
  const runners = new Map<string, ProjectRunner>()
  // URIs each project last reported diagnostics for, so that diagnostics for
  // files that are no longer reported (e.g. a deleted sprite) get cleared.
  const reportedUris = new Map<string, Set<vscode.Uri>>()

  context.subscriptions.push(
    channel,
    collection,
    vscode.workspace.onDidSaveTextDocument((document) => {
      if (document.languageId !== "goboscript" || document.uri.scheme !== "file") return
      const projectDir = findProjectRoot(document.uri.fsPath)
      if (projectDir === undefined) return
      let runner = runners.get(projectDir)
      if (runner === undefined) {
        runner = new ProjectRunner(projectDir, {
          debounceMs: DEBOUNCE_MS,
          timeoutMs: BUILD_TIMEOUT_MS,
          onResult: ({ parsed, savedDoc, error }) => {
            if (error !== undefined) channel.appendLine(`${projectDir}: ${error}`)
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

/**
 * Replaces the diagnostics of `projectDir` with `parsed`, resolving printed
 * paths against the project directory, and pinning diagnostics without a
 * location to `savedDoc` (or the project's `stage.gs` as a last resort).
 */
function publish(
  collection: vscode.DiagnosticCollection,
  reportedUris: Map<string, Set<vscode.Uri>>,
  projectDir: string,
  parsed: ParsedDiagnostic[],
  savedDoc: string | undefined,
): void {
  const byUri = new Map<vscode.Uri, vscode.Diagnostic[]>()
  for (const item of parsed) {
    const uri = resolveUri(item, projectDir, savedDoc)
    const range = computeRange(item, uri)
    const diagnostic = new vscode.Diagnostic(
      range,
      item.message,
      item.severity === "error" ?
        vscode.DiagnosticSeverity.Error
      : vscode.DiagnosticSeverity.Warning,
    )
    diagnostic.source = "goboscript"
    if (item.helps.length > 0) {
      diagnostic.relatedInformation = item.helps.map(
        (help) =>
          new vscode.DiagnosticRelatedInformation(
            new vscode.Location(uri, range),
            help,
          ),
      )
    }
    const list = byUri.get(uri)
    if (list) list.push(diagnostic)
    else byUri.set(uri, [diagnostic])
  }
  const previous = reportedUris.get(projectDir)
  if (previous) {
    for (const uri of previous) {
      if (!byUri.has(uri)) collection.delete(uri)
    }
  }
  for (const [uri, diagnostics] of byUri) collection.set(uri, diagnostics)
  reportedUris.set(projectDir, new Set(byUri.keys()))
}

function resolveUri(
  item: ParsedDiagnostic,
  projectDir: string,
  savedDoc: string | undefined,
): vscode.Uri {
  if (item.path === undefined) {
    // Diagnostics without any location, e.g. "failed to read stage.gs".
    return vscode.Uri.file(savedDoc ?? resolve(projectDir, "stage.gs"))
  }
  // Standard library files outside the project are printed as absolute paths.
  const path = isAbsolute(item.path) ? item.path : resolve(projectDir, item.path)
  return vscode.Uri.file(path)
}

function computeRange(item: ParsedDiagnostic, uri: vscode.Uri): vscode.Range {
  if (item.line === undefined || item.column === undefined) {
    return new vscode.Range(0, 0, 0, 0)
  }
  let text: string
  try {
    text = readFileSync(uri.fsPath, "utf8")
  } catch {
    return new vscode.Range(0, 0, 0, 0)
  }
  const lines = text.split(/\r?\n/)
  const line = Math.min(Math.max(item.line - 1, 0), lines.length - 1)
  // Compiler columns are 1-based code point indices, VS Code positions are
  // 0-based UTF-16 code unit indices.
  const column = Math.max(item.column - 1, 0)
  const utf16Column = Array.from(lines[line]).slice(0, column).join("").length
  const position = new vscode.Position(line, utf16Column)
  return new vscode.Range(position, position)
}
