import * as vscode from "vscode"

export class GoboscriptTaskProvider implements vscode.TaskProvider {
  static GoboscriptType = "goboscript-build"
  private workspaceRoot: vscode.WorkspaceFolder

  constructor(workspaceRoot: vscode.WorkspaceFolder) {
    this.workspaceRoot = workspaceRoot
  }

  provideTasks(token: vscode.CancellationToken): vscode.ProviderResult<vscode.Task[]> {
    const task = new vscode.Task(
      { type: "goboscript-build" },
      this.workspaceRoot,
      "goboscript-build",
      "goboscript",
      new vscode.ShellExecution("goboscript build --compact"),
      ["goboscript"],
    )
    return [task]
  }

  resolveTask(
    task: vscode.Task,
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<vscode.Task> {
    throw new Error("Method not implemented.")
  }
}
