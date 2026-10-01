import * as vscode from "vscode"
import { buildHtml } from "./html"

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

    const sendProject = async () => {
      try {
        const bytes = await vscode.workspace.fs.readFile(document.uri)
        const base64 = Buffer.from(bytes).toString("base64")
        await webview.postMessage({ type: "project", data: base64 })
      } catch (err) {
        console.error(`goboscript preview: failed to read ${document.uri.fsPath}: ${err}`)
        await webview.postMessage({
          type: "error",
          message: `failed to read ${document.uri.fsPath}: ${err}`,
        })
      }
    }

    const subscription = webview.onDidReceiveMessage((message: { type?: string; level?: string; message?: string }) => {
      if (message?.type === "log") {
        const line = `goboscript preview ${document.uri.fsPath}: ${message.message ?? ""}`
        if (message.level === "warn") console.warn(line)
        else if (message.level === "error") console.error(line)
        else console.log(line)
        return
      }
      if (message?.type === "ready" || message?.type === "reload") {
        void sendProject()
      }
    })
    webviewPanel.onDidDispose(() => subscription.dispose())
  }
}

interface Sb3Document extends vscode.CustomDocument {}
