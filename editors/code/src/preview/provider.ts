import * as vscode from "vscode"

/**
 * A read-only custom editor that renders a `.sb3` file with the bundled
 * TurboWarp Scaffolding player (see `webview/`).
 *
 * The player never builds or watches anything: it loads exactly the bytes on
 * disk when the editor opens, and again when Reload is pressed.
 */
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
    webview.html = buildHtml(webview, vscode.Uri.joinPath(this.extensionUri, "dist", "preview.js"))

    const sendProject = async () => {
      try {
        const bytes = await vscode.workspace.fs.readFile(document.uri)
        await webview.postMessage({
          type: "project",
          data: Buffer.from(bytes).toString("base64"),
        })
      } catch (err) {
        await webview.postMessage({
          type: "error",
          message: `failed to read ${document.uri.fsPath}: ${err}`,
        })
      }
    }

    const subscription = webview.onDidReceiveMessage((message: { type?: string }) => {
      if (message?.type === "ready" || message?.type === "reload") void sendProject()
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
    gap: 5px;
    padding: 3px 8px;
    border: 1px solid var(--vscode-button-border, transparent);
    border-radius: 3px;
    background: var(--vscode-button-background);
    color: var(--vscode-button-foreground);
    font-family: var(--vscode-font-family);
    font-size: 12px;
    cursor: pointer;
  }
  #toolbar button:disabled { opacity: 0.5; cursor: default; }
  #toolbar button:not(:disabled):hover { background: var(--vscode-button-hoverBackground); }
  #toolbar button svg { width: 14px; height: 14px; }
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
</style>
</head>
<body>
<div id="toolbar">
  <button id="reload" title="Reload the project from the .sb3 file on disk">
    <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
      <path d="M13.5 8a5.5 5.5 0 1 1-1.6-3.9" />
      <path d="M12 1.5v3h3" />
    </svg>
    Reload
  </button>
  <button id="green-flag" disabled title="Start the project">
    <svg viewBox="0 0 16 16">
      <path d="M3 1.5v13" stroke="currentColor" stroke-width="1.5" />
      <path d="M4 2.5h9l-2.5 3L13 8.5H4z" fill="#4cbf56" />
    </svg>
    Green flag
  </button>
  <button id="stop" disabled title="Stop the project">
    <svg viewBox="0 0 16 16">
      <rect x="3" y="3" width="10" height="10" rx="1.5" fill="#ec5959" />
    </svg>
    Stop
  </button>
</div>
<div id="stage">
  <div id="overlay">
    <h1 id="overlay-title">Loading…</h1>
    <p id="overlay-detail"></p>
  </div>
</div>
<script src="${script}"></script>
</body>
</html>`
}
