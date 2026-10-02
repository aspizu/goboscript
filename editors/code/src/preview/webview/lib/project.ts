import { signal } from "@preact/signals"
import { deserializeResult } from "../../result"
import { base64ToBytes, requestProject } from "./host"
import { logger } from "./logger"
import { player } from "./player"

export type Status =
  | { kind: "loading" }
  | { kind: "ready" }
  | { kind: "error"; title: string; detail: string }

const LOAD_TIMEOUT_MS = 30_000

export const status = signal<Status>({ kind: "loading" })
let generation = 0

export function fail(title: string, detail: string, log?: string): void {
  status.value = { kind: "error", title, detail }
  logger.error(log ?? `${title}: ${detail}`)
}

export async function loadProject(): Promise<void> {
  if (player === undefined) {
    return
  }
  const current = ++generation
  const timeout = new Promise<never>((_, reject) => {
    setTimeout(
      () => reject(new Error(`timed out after ${LOAD_TIMEOUT_MS / 1000}s`)),
      LOAD_TIMEOUT_MS,
    )
  })
  status.value = { kind: "loading" }
  player.stopAll()
  try {
    const project = deserializeResult(await Promise.race([requestProject(), timeout]))
    if (generation !== current) {
      return
    }
    if (project.isErr()) {
      status.value = { kind: "error", title: "Failed to read file", detail: project.error }
      return
    }
    await Promise.race([player.loadProject(base64ToBytes(project.value)), timeout])
    if (generation !== current) {
      return
    }
    status.value = { kind: "ready" }
  } catch (err) {
    if (generation !== current) {
      return
    }
    fail(
      "Failed to load project",
      err instanceof Error ? err.message : String(err),
      `load failed: ${err instanceof Error ? err.stack : String(err)}`,
    )
  }
}

function report(event: Event): void {
  const detail =
    event instanceof ErrorEvent
      ? event.message
      : event instanceof PromiseRejectionEvent
        ? String(event.reason)
        : String(event)
  const where =
    event instanceof ErrorEvent ? ` (${event.filename}:${event.lineno}:${event.colno})` : ""
  fail("Player crashed", detail, `${event.type}: ${detail}${where}`)
}

window.addEventListener("error", report)
window.addEventListener("unhandledrejection", report)
