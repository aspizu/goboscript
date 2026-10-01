import type { HostMessage } from "../../messages"
import type { Result } from "../../result"

declare function acquireVsCodeApi(): { postMessage(message: unknown): void }

const api = acquireVsCodeApi()

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

export function postToHost(message: unknown): void {
  api.postMessage(message)
}

export function requestProject(): Promise<Result<string>> {
  return new Promise((resolve) => {
    const id = nextRequestId++
    pendingRequests.set(id, resolve)
    api.postMessage({ type: "loadProject", id })
  })
}

export function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64)
  const bytes = new Uint8Array(binary.length)
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i)
  }
  return bytes
}
