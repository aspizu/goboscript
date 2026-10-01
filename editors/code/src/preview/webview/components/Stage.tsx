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
    <div class="relative min-h-0 flex-1 *:absolute *:inset-0">
      <div ref={container} />
      {children}
    </div>
  )
}
