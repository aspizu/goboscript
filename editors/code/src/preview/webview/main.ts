import scaffoldingBundle from "@turbowarp/scaffolding/with-music"

declare function acquireVsCodeApi(): { postMessage(message: unknown): void }

type HostMessage =
  | { type: "project"; data: string }
  | { type: "error"; message: string }

const api = acquireVsCodeApi()

const stage = requiredElement<HTMLElement>("#stage")
const overlay = requiredElement<HTMLDivElement>("#overlay")
const overlayTitle = requiredElement<HTMLElement>("#overlay-title")
const overlayDetail = requiredElement<HTMLElement>("#overlay-detail")
const reloadButton = requiredElement<HTMLButtonElement>("#reload")
const greenFlagButton = requiredElement<HTMLButtonElement>("#green-flag")
const stopButton = requiredElement<HTMLButtonElement>("#stop")

const scaffolding = new scaffoldingBundle.Scaffolding()
scaffolding.resizeMode = "preserve-ratio"
scaffolding.setup()
scaffolding.appendTo(stage)

// Each incoming project message increments this; an async load only commits
// if no newer message has arrived in the meantime.
let generation = 0

function setControlsEnabled(ready: boolean): void {
  greenFlagButton.disabled = !ready
  stopButton.disabled = !ready
}

function showOverlay(title: string, detail: string): void {
  overlayTitle.textContent = title
  overlayDetail.textContent = detail
  overlay.hidden = false
}

function hideOverlay(): void {
  overlay.hidden = true
}

async function load(base64: string): Promise<void> {
  const current = ++generation
  setControlsEnabled(false)
  showOverlay("Loading…", "")
  // Stop any running threads before swapping the project out.
  scaffolding.stopAll()
  try {
    await scaffolding.loadProject(base64ToBytes(base64))
    if (generation !== current) return
    hideOverlay()
    setControlsEnabled(true)
  } catch (err) {
    if (generation !== current) return
    showOverlay("Failed to load project", err instanceof Error ? err.message : String(err))
  }
}

reloadButton.addEventListener("click", () => api.postMessage({ type: "reload" }))
greenFlagButton.addEventListener("click", () => scaffolding.greenFlag())
stopButton.addEventListener("click", () => scaffolding.stopAll())

window.addEventListener("message", (event: MessageEvent<HostMessage>) => {
  const message = event.data
  if (message?.type === "project") void load(message.data)
  else if (message?.type === "error") {
    generation++
    setControlsEnabled(false)
    showOverlay("Failed to read file", message.message)
  }
})

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

api.postMessage({ type: "ready" })
