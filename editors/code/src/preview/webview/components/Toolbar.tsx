import type { ComponentChild } from "preact"
import { Play, RotateCw, Square } from "lucide-preact"

export function Toolbar({
  ready,
  loading,
  onReload,
  onGreenFlag,
  onStop,
}: {
  ready: boolean
  loading: boolean
  onReload: () => void
  onGreenFlag: () => void
  onStop: () => void
}) {
  return (
    <div class="flex flex-none gap-1 border-b border-vscode-border px-1.5 py-1">
      <ToolbarButton title="Start the project" disabled={!ready} onClick={onGreenFlag}>
        <Play class="h-4 w-4" />
      </ToolbarButton>
      <ToolbarButton title="Stop the project" disabled={!ready} onClick={onStop}>
        <Square class="h-4 w-4" />
      </ToolbarButton>
      <ToolbarButton
        title="Reload the project from the .sb3 file on disk"
        onClick={onReload}
        disabled={loading}
        class="ml-auto"
      >
        <RotateCw class={loading ? "h-4 w-4 animate-spin" : "h-4 w-4"} />
      </ToolbarButton>
    </div>
  )
}

function ToolbarButton({
  title,
  disabled,
  onClick,
  class: className,
  children,
}: {
  title: string
  disabled?: boolean
  onClick: () => void
  class?: string
  children: ComponentChild
}) {
  return (
    <button
      type="button"
      title={title}
      disabled={disabled}
      onClick={onClick}
      class={`flex h-6 w-6 items-center justify-center rounded-[5px] bg-transparent p-0 text-xs text-vscode-icon transition-transform duration-75 enabled:cursor-pointer enabled:hover:bg-vscode-hover enabled:active:scale-95 enabled:active:bg-vscode-active disabled:cursor-default disabled:opacity-40 ${className ?? ""}`}
    >
      {children}
    </button>
  )
}
