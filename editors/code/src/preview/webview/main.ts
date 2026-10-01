import { Scaffolding as ScaffoldingConstructor } from "@turbowarp/scaffolding/with-music"
import type { HostMessage } from "../messages"
import { deserializeResult, type Result } from "../result"
import { Logger } from "./logger"
import "./styles.css"

declare function acquireVsCodeApi(): { postMessage(message: unknown): void }

const api = acquireVsCodeApi()
const logger = new Logger(api.postMessage)

for (const event of ["error", "unhandledrejection"] as const) {
  window.addEventListener(event, (ev: Event) => {
    const detail =
      ev instanceof ErrorEvent
        ? ev.message
        : ev instanceof PromiseRejectionEvent
          ? String(ev.reason)
          : String(ev)
    const where = ev instanceof ErrorEvent ? ` (${ev.filename}:${ev.lineno}:${ev.colno})` : ""
    reportError(`${event}: ${detail}${where}`)
  })
}

let player: Scaffolding | undefined

const pendingRequests = new Map<number, (result: Result<string>) => void>()
let nextRequestId = 0

window.addEventListener("message", (event: MessageEvent<HostMessage>) => {
  const message = event.data
  switch (message.type) {
    case "loadProjectResult": {
      const resolve = pendingRequests.get(message.id)
      if (resolve === undefined) {
        break
      }
      pendingRequests.delete(message.id)
      resolve(message.result)
      break
    }
  }
})

function requestProject(): Promise<Result<string>> {
  return new Promise((resolve) => {
    const id = nextRequestId++
    pendingRequests.set(id, resolve)
    api.postMessage({ type: "loadProject", id })
  })
}

try {
  const stage = requiredElement<HTMLElement>("#stage")
  const scaffolding = new ScaffoldingConstructor()
  scaffolding.resizeMode = "preserve-ratio"
  scaffolding.setup()
  scaffolding.appendTo(stage)
  requiredElement<HTMLButtonElement>("#reload").addEventListener("click", () => void load())
  requiredElement<HTMLButtonElement>("#green-flag").addEventListener("click", () =>
    scaffolding.greenFlag(),
  )
  requiredElement<HTMLButtonElement>("#stop").addEventListener("click", () => scaffolding.stopAll())
  player = scaffolding
} catch (err) {
  reportError(`player init failed: ${err instanceof Error ? err.stack : String(err)}`)
  showOverlay("Failed to initialize player", err instanceof Error ? err.message : String(err))
}

let generation = 0

void load()

async function load(): Promise<void> {
  if (player === undefined) {
    return
  }
  const current = ++generation
  setControlsEnabled(false)
  showLoading()
  player.stopAll()
  const timeout = new Promise<never>((_, reject) => {
    setTimeout(() => reject(new Error("timed out after 30s")), 30_000)
  })
  try {
    const project = deserializeResult(await Promise.race([requestProject(), timeout]))
    if (generation !== current) {
      return
    }
    if (project.isErr()) {
      showOverlay("Failed to read file", project.error)
      return
    }
    await Promise.race([player.loadProject(base64ToBytes(project.value)), timeout])
    if (generation !== current) {
      return
    }
    hideOverlay()
    setControlsEnabled(true)
  } catch (err) {
    if (generation !== current) {
      return
    }
    const message = err instanceof Error ? err.message : String(err)
    showOverlay("Failed to load project", message)
    logger.error(`load failed: ${err instanceof Error ? err.stack : String(err)}`)
  }
}

function reportError(message: string): void {
  showOverlay("Player crashed", message)
  logger.error(message)
}

function setControlsEnabled(ready: boolean): void {
  requiredElement<HTMLButtonElement>("#green-flag").disabled = !ready
  requiredElement<HTMLButtonElement>("#stop").disabled = !ready
}

function showOverlay(title: string, detail: string): void {
  requiredElement<HTMLElement>("#spinner").hidden = true
  const titleElement = requiredElement<HTMLElement>("#overlay-title")
  const detailElement = requiredElement<HTMLElement>("#overlay-detail")
  titleElement.textContent = title
  detailElement.textContent = detail
  titleElement.hidden = false
  detailElement.hidden = detail === ""
  requiredElement<HTMLElement>("#overlay").hidden = false
}

function showLoading(): void {
  requiredElement<HTMLElement>("#overlay-title").hidden = true
  requiredElement<HTMLElement>("#overlay-detail").hidden = true
  requiredElement<HTMLElement>("#spinner").hidden = false
  requiredElement<HTMLElement>("#overlay").hidden = false
}

function hideOverlay(): void {
  requiredElement<HTMLElement>("#overlay").hidden = true
}

function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64)
  const bytes = new Uint8Array(binary.length)
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i)
  }
  return bytes
}

function requiredElement<T extends HTMLElement>(selector: string): T {
  const element = document.querySelector(selector)
  if (!(element instanceof HTMLElement)) {
    throw new Error(`preview: missing element ${selector}`)
  }
  return element as T
}
