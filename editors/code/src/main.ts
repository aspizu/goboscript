import * as vscode from "vscode";
import {
  LanguageClient,
  type LanguageClientOptions,
  type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(_context: vscode.ExtensionContext) {
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
