import { Play, RotateCw, Square } from "lucide-preact"
import { player, running } from "../lib/player"
import { loadProject, status } from "../lib/project"
import { IconButton } from "./IconButton"

export function Toolbar() {
  const ready = status.value.kind === "ready"
  const loading = status.value.kind === "loading"
  return (
    <div class="flex flex-none gap-1 border-b border-vscode-border px-1.5 py-1">
      <IconButton
        title={running.value ? "Restart the project" : "Start the project"}
        disabled={!ready}
        active={running.value}
        onClick={() => player?.greenFlag()}
      >
        <Play class="h-4 w-4" />
      </IconButton>
      <IconButton title="Stop the project" disabled={!ready} onClick={() => player?.stopAll()}>
        <Square class="h-4 w-4" />
      </IconButton>
      <IconButton
        title="Reload the project from the .sb3 file on disk"
        onClick={() => void loadProject()}
        disabled={loading}
        class="ml-auto"
      >
        <RotateCw class={loading ? "h-4 w-4 animate-spin" : "h-4 w-4"} />
      </IconButton>
    </div>
  )
}
