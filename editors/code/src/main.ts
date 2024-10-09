import * as vscode from "vscode"
import { GoboscriptTaskProvider } from "./goboscript-task-provider"

let goboscriptTaskProvider: vscode.Disposable | undefined

export function activate(context: vscode.ExtensionContext): void {
  const workspaceRoot =
    vscode.workspace.workspaceFolders && vscode.workspace.workspaceFolders.length > 0 ?
      vscode.workspace.workspaceFolders[0]
    : undefined
  if (!workspaceRoot) return
  goboscriptTaskProvider = vscode.tasks.registerTaskProvider(
    GoboscriptTaskProvider.GoboscriptType,
    new GoboscriptTaskProvider(workspaceRoot),
  )
}
