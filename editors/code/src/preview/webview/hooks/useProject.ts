import { useCallback, useEffect, useRef, useState } from "preact/hooks"
import { deserializeResult } from "../../result"
import { base64ToBytes, requestProject } from "../lib/host"
import type { Logger } from "../lib/logger"

export type Status =
  | { kind: "loading" }
  | { kind: "ready" }
  | { kind: "error"; title: string; detail: string }

const LOAD_TIMEOUT_MS = 30_000

export function useProject(logger: Logger) {
  const [status, setStatus] = useState<Status>({ kind: "loading" })
  const generation = useRef(0)

  const fail = useCallback(
    (title: string, detail: string, log?: string) => {
      setStatus({ kind: "error", title, detail })
      logger.error(log ?? `${title}: ${detail}`)
    },
    [logger],
  )

  const load = useCallback(
    async (player: Scaffolding) => {
      const current = ++generation.current
      const timeout = new Promise<never>((_, reject) => {
        setTimeout(
          () => reject(new Error(`timed out after ${LOAD_TIMEOUT_MS / 1000}s`)),
          LOAD_TIMEOUT_MS,
        )
      })
      setStatus({ kind: "loading" })
      player.stopAll()
      try {
        const project = deserializeResult(await Promise.race([requestProject(), timeout]))
        if (generation.current !== current) {
          return
        }
        if (project.isErr()) {
          setStatus({ kind: "error", title: "Failed to read file", detail: project.error })
          return
        }
        await Promise.race([player.loadProject(base64ToBytes(project.value)), timeout])
        if (generation.current !== current) {
          return
        }
        setStatus({ kind: "ready" })
      } catch (err) {
        if (generation.current !== current) {
          return
        }
        fail(
          "Failed to load project",
          err instanceof Error ? err.message : String(err),
          `load failed: ${err instanceof Error ? err.stack : String(err)}`,
        )
      }
    },
    [fail],
  )

  useEffect(() => {
    const report = (event: Event) => {
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
    const events = ["error", "unhandledrejection"] as const
    for (const type of events) {
      window.addEventListener(type, report)
    }
    return () => {
      for (const type of events) {
        window.removeEventListener(type, report)
      }
    }
  }, [fail])

  return { status, load, fail }
}
