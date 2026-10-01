import { Scaffolding as ScaffoldingConstructor } from "@turbowarp/scaffolding/with-music"

const api = acquireVsCodeApi()

api.postMessage({ type: "ready" })

for (const event of ["error", "unhandledrejection"] as const) {
  window.addEventListener(event, (ev: Event) => {
    const detail = ev instanceof ErrorEvent ? ev.message : ev instanceof PromiseRejectionEvent ? String(ev.reason) : String(ev)
    const where = ev instanceof ErrorEvent ? ` (${ev.filename}:${ev.lineno}:${ev.colno})` : ""
    reportError(`${event}: ${detail}${where}`)
  })
}

type HostMessage =
  | { type: "project"; data: string }
  | { type: "error"; message: string }

declare function acquireVsCodeApi(): { postMessage(message: unknown): void }

let pendingProject: string | undefined
let player: Scaffolding | undefined

window.addEventListener("message", (event: MessageEvent<HostMessage>) => {
  const message = event.data
  if (message?.type === "project") {
    api.postMessage({ type: "log", message: `received project (${message.data.length} base64 chars)` })
    pendingProject = message.data
    void load()
  } else if (message?.type === "error") {
    api.postMessage({ type: "log", message: `host read failed: ${message.message}` })
    showOverlay("Failed to read file", message.message)
  }
})

try {
  const stage = requiredElement<HTMLElement>("#stage")
  const scaffolding = new ScaffoldingConstructor()
  scaffolding.resizeMode = "preserve-ratio"
  scaffolding.setup()
  scaffolding.appendTo(stage)
  requiredElement<HTMLButtonElement>("#reload").addEventListener("click", () => api.postMessage({ type: "reload" }))
  requiredElement<HTMLButtonElement>("#green-flag").addEventListener("click", () => scaffolding.greenFlag())
  requiredElement<HTMLButtonElement>("#stop").addEventListener("click", () => scaffolding.stopAll())
  player = scaffolding
} catch (err) {
  reportError(`player init failed: ${err instanceof Error ? err.stack : String(err)}`)
  showOverlay("Failed to initialize player", err instanceof Error ? err.message : String(err))
}

let generation = 0

if (pendingProject !== undefined) void load()

async function load(): Promise<void> {
  const base64 = pendingProject
  if (base64 === undefined || player === undefined) return
  const current = ++generation
  setControlsEnabled(false)
  showLoading()
  player.stopAll()
  const timeout = new Promise<never>((_, reject) => {
    setTimeout(() => reject(new Error("timed out after 30s")), 30_000)
  })
  try {
    await Promise.race([player.loadProject(base64ToBytes(base64)), timeout])
    if (generation !== current) return
    hideOverlay()
    setControlsEnabled(true)
  } catch (err) {
    if (generation !== current) return
    const message = err instanceof Error ? err.message : String(err)
    showOverlay("Failed to load project", message)
    api.postMessage({ type: "log", message: `load failed: ${err instanceof Error ? err.stack : String(err)}` })
  }
}

function reportError(message: string): void {
  showOverlay("Player crashed", message)
  api.postMessage({ type: "webview-error", message })
}

function setControlsEnabled(ready: boolean): void {
  requiredElement<HTMLButtonElement>("#green-flag").disabled = !ready
  requiredElement<HTMLButtonElement>("#stop").disabled = !ready
}

function showOverlay(title: string, detail: string): void {
  requiredElement<HTMLDivElement>("#spinner").hidden = true
  const titleElement = requiredElement<HTMLElement>("#overlay-title")
  const detailElement = requiredElement<HTMLElement>("#overlay-detail")
  titleElement.textContent = title
  detailElement.textContent = detail
  titleElement.hidden = false
  detailElement.hidden = detail === ""
  requiredElement<HTMLDivElement>("#overlay").hidden = false
}

function showLoading(): void {
  requiredElement<HTMLElement>("#overlay-title").hidden = true
  requiredElement<HTMLElement>("#overlay-detail").hidden = true
  requiredElement<HTMLDivElement>("#spinner").hidden = false
  requiredElement<HTMLDivElement>("#overlay").hidden = false
}

function hideOverlay(): void {
  requiredElement<HTMLDivElement>("#overlay").hidden = true
}

function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64)
  const bytes = new Uint8Array(binary.length)
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i)
  return bytes
}

function requiredElement<T extends HTMLElement>(selector: string): T {
  const element = document.querySelector(selector)
  if (!(element instanceof HTMLElement)) {
    throw new Error(`preview: missing element ${selector}`)
  }
  return element as T
}
