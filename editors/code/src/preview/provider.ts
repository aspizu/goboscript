import * as vscode from "vscode"

/**
 * A read-only custom editor that renders a `.sb3` file with the bundled
 * TurboWarp Scaffolding player (see `webview/`).
 *
 * The player never builds or watches anything: it loads exactly the bytes on
 * disk when the editor opens, and again when Reload is pressed.
 */
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
    webview.html = buildHtml(webview, vscode.Uri.joinPath(this.extensionUri, "dist", "preview.js"))

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

function buildHtml(webview: vscode.Webview, scriptUri: vscode.Uri): string {
  const csp = [
    "default-src 'none'",
    `img-src ${webview.cspSource} blob: data:`,
    "media-src blob: data:",
    `script-src ${webview.cspSource}`,
    `style-src ${webview.cspSource} 'unsafe-inline'`,
    `font-src ${webview.cspSource} data:`,
    `connect-src ${webview.cspSource} data: blob:`,
    `worker-src ${webview.cspSource} blob: data:`,
  ].join("; ")
  const script = webview.asWebviewUri(scriptUri)
  return `<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8" />
<meta http-equiv="Content-Security-Policy" content="${csp}" />
<style>
  html, body {
    height: 100%;
    margin: 0;
    padding: 0;
    background: var(--vscode-editor-background);
    color: var(--vscode-editor-foreground);
    font-family: var(--vscode-font-family);
  }
  body { display: flex; flex-direction: column; }
  #toolbar {
    flex: none;
    display: flex;
    gap: 4px;
    padding: 4px 6px;
    border-bottom: 1px solid var(--vscode-panel-border);
  }
  #toolbar button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    width: 24px;
    height: 24px;
    border: none;
    border-radius: 5px;
    background: transparent;
    color: var(--vscode-icon-foreground, var(--vscode-editor-foreground));
    font-family: var(--vscode-font-family);
    font-size: 12px;
    cursor: pointer;
  }
  #toolbar button:disabled { opacity: 0.4; cursor: default; }
  #toolbar button:not(:disabled):hover { background: var(--vscode-toolbar-hoverBackground); }
  #toolbar button:not(:disabled):active { background: var(--vscode-toolbar-activeBackground); }
  #toolbar button svg { width: 16px; height: 16px; }
  #stage { position: relative; flex: 1; min-height: 0; }
  #stage > * { position: absolute; inset: 0; }
  #overlay {
    z-index: 10;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    text-align: center;
    background: var(--vscode-editor-background);
    padding: 16px;
  }
  #overlay[hidden] { display: none; }
  #overlay h1 { font-size: 15px; font-weight: 600; margin: 0; }
  #overlay p { font-size: 12px; margin: 0; opacity: 0.8; white-space: pre-wrap; }
  #spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--vscode-editorWidget-border, rgba(128, 128, 128, 0.4));
    border-top-color: var(--vscode-editor-foreground);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  #spinner[hidden] { display: none; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
</head>
<body>
<div id="toolbar">
  <!-- Icons: Radix Icons (https://github.com/radix-ui/icons), MIT -->
  <button id="reload" title="Reload the project from the .sb3 file on disk">
    <svg viewBox="0 0 15 15" fill="none">
      <path d="M7.50037 0.850006C10.6644 0.850189 12.2943 3.06869 12.9994 4.31094L13.0004 4.31192V2.5004C13.0004 2.22425 13.2242 2.0004 13.5004 2.0004C13.7763 2.00059 14.0004 2.22438 14.0004 2.5004V5.5004C14.0002 5.77625 13.7762 6.0002 13.5004 6.0004H10.5004C10.2243 6.0004 10.0006 5.77637 10.0004 5.5004C10.0004 5.22425 10.2242 5.0004 10.5004 5.0004H12.2328L12.1215 4.79239C11.4802 3.66597 10.1107 1.85019 7.50037 1.85001C4.06019 1.85001 1.84998 4.665 1.84998 7.5004C1.85018 10.3357 4.06034 13.1498 7.50037 13.1498C9.16525 13.1497 10.5296 12.496 11.5013 11.5072L11.6927 11.3031C12.126 10.8159 12.4715 10.2575 12.7172 9.66055L12.765 9.57071C12.8948 9.37795 13.1462 9.2963 13.3695 9.38809C13.6248 9.49314 13.7468 9.78515 13.642 10.0404L13.5111 10.3373C13.2362 10.9247 12.877 11.4767 12.4398 11.9682L12.2142 12.2084C11.062 13.3807 9.44396 14.1497 7.50037 14.1498C3.43771 14.1498 0.850179 10.8149 0.849976 7.5004C0.849976 4.1858 3.43755 0.850006 7.50037 0.850006Z" fill="currentColor" />
    </svg>
  </button>
  <button id="green-flag" disabled title="Start the project">
    <svg viewBox="0 0 15 15" fill="none">
      <path d="M3.24219 2.32213C3.39223 2.23169 3.57845 2.22573 3.7334 2.30748L12.7334 7.05748C12.8974 7.14403 13 7.31443 13 7.49986C13 7.68529 12.8974 7.85569 12.7334 7.94224L3.7334 12.6922C3.57845 12.774 3.39223 12.768 3.24219 12.6776C3.09211 12.5871 3 12.4251 3 12.2499V2.74986C3 2.57461 3.09211 2.41261 3.24219 2.32213ZM4 11.4198L11.4277 7.49986L4 3.57896V11.4198Z" fill="currentColor" />
    </svg>
  </button>
  <button id="stop" disabled title="Stop the project">
    <svg viewBox="0 0 15 15" fill="none">
      <path d="M12 2C12.5523 2 13 2.44772 13 3V12C13 12.5523 12.5523 13 12 13H3C2.44772 13 2 12.5523 2 12V3C2 2.44772 2.44772 2 3 2H12ZM3 12H12V3H3V12Z" fill="currentColor" />
    </svg>
  </button>
</div>
<div id="stage">
  <div id="overlay">
    <div id="spinner"></div>
    <h1 id="overlay-title" hidden>Loading</h1>
    <p id="overlay-detail" hidden></p>
  </div>
</div>
<script src="${script}"></script>
</body>
</html>`
}
