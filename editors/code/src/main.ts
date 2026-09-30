import * as vscode from "vscode";
import { spawnSync } from "node:child_process";
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

function serverAvailable(): boolean {
  const result = spawnSync("goboscript", ["lsp"], { input: "", timeout: 2000 });
  return result.error === undefined && result.status === 0;
}

export function activate(_context: vscode.ExtensionContext) {
  if (!serverAvailable()) {
    void vscode.window.showWarningMessage(
      "`goboscript lsp` is not available. Install or update the `goboscript` compiler to get diagnostics and completions. Syntax highlighting is unaffected.",
    );
    return;
  }
  const serverOptions: ServerOptions = {
    command: "goboscript",
    args: ["lsp"],
  };
  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { language: "goboscript", scheme: "file" },
      { language: "goboscript", scheme: "untitled" },
    ],
    outputChannelName: "goboscript",
  };
  client = new LanguageClient("goboscript", "goboscript", serverOptions, clientOptions);
  void client.start();
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
