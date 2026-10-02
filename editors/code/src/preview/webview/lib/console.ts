import { computed, signal } from "@preact/signals"

export type ConsoleLevel = "error" | "warning" | "info" | "log"

export interface ConsoleMessage {
  level: ConsoleLevel
  message: string
  origin: string
}

const SEVERITY: Record<ConsoleLevel, number> = { log: 0, info: 1, warning: 2, error: 3 }

export const messages = signal<ConsoleMessage[]>([])
export const severity = signal<ConsoleLevel>("log")
export const visibleMessages = computed(() =>
  messages.value.filter((message) => SEVERITY[message.level] >= SEVERITY[severity.value]),
)
