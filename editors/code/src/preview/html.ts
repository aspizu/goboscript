import * as vscode from "vscode"
import htmlSource from "./index.html"

export function buildHtml(webview: vscode.Webview, extensionUri: vscode.Uri): string {
  const script = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, "dist", "preview.js"))
  const stylesheet = webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, "dist", "preview.css"))
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
  return htmlSource
    .replace("{{csp}}", () => csp)
    .replace("{{tailwind}}", () => stylesheet.toString())
    .replace("{{script}}", () => script.toString())
}
