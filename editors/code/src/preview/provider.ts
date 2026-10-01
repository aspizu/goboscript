import * as vscode from "vscode"
import { buildHtml } from "./html"

export class Sb3PreviewProvider implements vscode.CustomReadonlyEditorProvider<Sb3Document> {
  constructor(
    private readonly extensionUri: vscode.Uri,
    private readonly log: (line: string) => void,
  ) {}

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
        const delivered = await webview.postMessage({ type: "project", data: base64 })
        this.log(`sb3 preview ${document.uri.fsPath}: sent project (${base64.length} base64 chars), delivered=${delivered}`)
      } catch (err) {
        await webview.postMessage({
          type: "error",
          message: `failed to read ${document.uri.fsPath}: ${err}`,
        })
      }
    }

    const subscription = webview.onDidReceiveMessage((message: { type?: string; message?: string }) => {
      if (message?.type === "webview-error" || message?.type === "log") {
        this.log(`sb3 preview ${document.uri.fsPath}: ${message.message ?? "unknown error"}`)
        return
      }
      if (message?.type === "ready" || message?.type === "reload") {
        this.log(`sb3 preview ${document.uri.fsPath}: ${message.type}, sending project`)
        void sendProject()
      }
    })
    webviewPanel.onDidDispose(() => subscription.dispose())
  }
}

interface Sb3Document extends vscode.CustomDocument {}
