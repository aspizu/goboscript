import type { ComponentChild } from "preact"
import { useEffect, useRef } from "preact/hooks"
import { player, playerInitFailure } from "../lib/player"

export function Stage({
  onReady,
  onError,
  children,
}: {
  onReady: () => void
  onError: (title: string, detail: string, log: string) => void
  children: ComponentChild
}) {
  const container = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const element = container.current
    if (element === null) {
      return
    }
    if (player === undefined) {
      if (playerInitFailure !== undefined) {
        onError("Failed to initialize player", playerInitFailure.detail, playerInitFailure.log)
      }
      return
    }
    player.appendTo(element)
    onReady()
  }, [onReady, onError])

  return (
    <div class="relative min-h-0 flex-1">
      <div
        ref={container}
        // The player centers the stage vertically; align it to the top instead.
        // The `!` wins over the player's own stylesheet, which loads after ours.
        class="absolute inset-x-0 top-0 aspect-4/3 max-h-full border-b border-vscode-border *:[align-items:flex-start]!"
      />
      {children}
    </div>
  )
}
