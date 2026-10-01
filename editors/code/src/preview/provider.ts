import { ResultAsync } from "neverthrow"
import * as vscode from "vscode"
import { buildHtml } from "./html"
import type { WebviewMessage } from "./messages"
import { serializeResult } from "./result"

export class Sb3PreviewProvider implements vscode.CustomReadonlyEditorProvider<Sb3Document> {
  constructor(private readonly extensionUri: vscode.Uri) {}

  async openCustomDocument(uri: vscode.Uri): Promise<Sb3Document> {
    return { uri, dispose: () => {} }
  }

  async resolveCustomEditor(
    document: Sb3Document,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken,
  ): Promise<void> {
    const webview = webviewPanel.webview
    webview.options = { enableScripts: true, localResourceRoots: [this.extensionUri] }
    webview.html = buildHtml(webview, this.extensionUri)

    const respond = async (id: number): Promise<void> => {
      const result = await ResultAsync.fromPromise(
        vscode.workspace.fs.readFile(document.uri),
        (err) => `failed to read ${document.uri.fsPath}: ${err}`,
      ).map((bytes) => Buffer.from(bytes).toString("base64"))
      if (result.isErr()) {
        console.error(`goboscript preview ${document.uri.fsPath}: ${result.error}`)
      }
      await webview.postMessage({ type: "loadProjectResult", id, result: serializeResult(result) })
    }

    const subscription = webview.onDidReceiveMessage((message: WebviewMessage) => {
      switch (message.type) {
        case "loadProject":
          void respond(message.id)
          break
        case "log": {
          const line = `goboscript preview ${document.uri.fsPath}: ${message.message}`
          switch (message.level) {
            case "warn":
              console.warn(line)
              break
            case "error":
              console.error(line)
              break
            default:
              console.log(line)
          }
          break
        }
      }
    })
    webviewPanel.onDidDispose(() => subscription.dispose())
  }
}

interface Sb3Document extends vscode.CustomDocument {}
