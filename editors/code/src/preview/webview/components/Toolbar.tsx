import { Play, RotateCw, Square } from "lucide-preact"
import { IconButton } from "./IconButton"

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
      <IconButton title="Start the project" disabled={!ready} onClick={onGreenFlag}>
        <Play class="h-4 w-4" />
      </IconButton>
      <IconButton title="Stop the project" disabled={!ready} onClick={onStop}>
        <Square class="h-4 w-4" />
      </IconButton>
      <IconButton
        title="Reload the project from the .sb3 file on disk"
        onClick={onReload}
        disabled={loading}
        class="ml-auto"
      >
        <RotateCw class={loading ? "h-4 w-4 animate-spin" : "h-4 w-4"} />
      </IconButton>
    </div>
  )
}
