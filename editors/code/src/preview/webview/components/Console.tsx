import { useSignal } from "@preact/signals"
import { useLayoutEffect, useRef } from "preact/hooks"
import { Ban, CircleX, Info, TriangleAlert } from "lucide-preact"
import {
  messages,
  severity,
  visibleMessages,
  type ConsoleLevel,
  type ConsoleMessage,
} from "../lib/console"
import { IconButton } from "./IconButton"

const FILTERS: { severity: ConsoleLevel; label: string }[] = [
  { severity: "log", label: "All levels" },
  { severity: "info", label: "Info and up" },
  { severity: "warning", label: "Warnings and up" },
  { severity: "error", label: "Errors only" },
]

export function Console() {
  const visible = visibleMessages.value
  const list = useRef<HTMLUListElement>(null)
  const atBottom = useSignal(true)

  useLayoutEffect(() => {
    const element = list.current
    if (element !== null && atBottom.peek()) {
      element.scrollTop = element.scrollHeight
    }
  }, [visible])

  return (
    <section class="flex min-h-24 min-w-0 flex-1 flex-col">
      <header class="flex flex-none items-center gap-1 border-b border-vscode-border px-1.5 py-1">
        <h2 class="px-1 text-xs font-medium">Console</h2>
        <div class="ml-auto flex items-center gap-1">
          <select
            title="Filter by level"
            class="h-6 cursor-pointer rounded-[5px] bg-vscode-dropdown px-1.5 text-xs text-vscode-foreground"
            value={severity.value}
            onChange={(event) => {
              severity.value = event.currentTarget.value as ConsoleLevel
            }}
          >
            {FILTERS.map(({ severity: value, label }) => (
              <option value={value}>{label}</option>
            ))}
          </select>
          <IconButton title="Clear the console" onClick={() => (messages.value = [])}>
            <Ban class="h-4 w-4" />
          </IconButton>
        </div>
      </header>
      <ul
        ref={list}
        class="min-h-0 flex-1 overflow-y-auto font-mono text-xs leading-5"
        onScroll={() => {
          const element = list.current
          if (element !== null) {
            atBottom.value = element.scrollHeight - element.scrollTop - element.clientHeight < 1
          }
        }}
      >
        {visible.map((message, index) => (
          <Row key={index} message={message} />
        ))}
      </ul>
    </section>
  )
}

function Row({ message }: { message: ConsoleMessage }) {
  return (
    <li class={`flex items-start gap-2 px-2 py-0.5 ${levelBackground(message.level)}`}>
      <span class="flex h-5 w-3.5 flex-none items-center justify-center">
        <LevelIcon level={message.level} />
      </span>
      <span
        class={`min-w-0 flex-1 wrap-break-word whitespace-pre-wrap ${levelText(message.level)}`}
      >
        {message.message}
      </span>
      <span class="flex-none text-vscode-muted">{message.origin}</span>
    </li>
  )
}

function levelBackground(level: ConsoleLevel): string {
  switch (level) {
    case "error": {
      return "bg-vscode-error/10 hover:bg-vscode-error/20"
    }
    case "warning": {
      return "bg-vscode-warning/10 hover:bg-vscode-warning/20"
    }
    case "info":
    case "log": {
      return "hover:bg-vscode-hover"
    }
  }
}

function LevelIcon({ level }: { level: ConsoleLevel }) {
  switch (level) {
    case "error": {
      return <CircleX class="h-3.5 w-3.5 text-vscode-error" />
    }
    case "warning": {
      return <TriangleAlert class="h-3.5 w-3.5 text-vscode-warning" />
    }
    case "info": {
      return <Info class="h-3.5 w-3.5 text-vscode-info" />
    }
    case "log": {
      return null
    }
  }
}

function levelText(level: ConsoleLevel): string {
  switch (level) {
    case "error": {
      return "text-vscode-error"
    }
    case "warning": {
      return "text-vscode-warning"
    }
    case "info":
    case "log": {
      return ""
    }
  }
}
