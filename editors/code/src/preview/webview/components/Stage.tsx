import type { ComponentChild } from "preact"
import { useEffect, useRef } from "preact/hooks"
import { player, playerInitFailure } from "../lib/player"
import { fail, loadProject } from "../lib/project"

export function Stage({ children }: { children: ComponentChild }) {
  const container = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const element = container.current
    if (element === null) {
      return
    }
    if (player === undefined) {
      if (playerInitFailure !== undefined) {
        fail("Failed to initialize player", playerInitFailure.detail, playerInitFailure.log)
      }
      return
    }
    player.appendTo(element)
    void loadProject()
  }, [])

  return (
    <div
      ref={container}
      class="relative aspect-4/3 min-h-0 border-b border-vscode-border *:items-start!"
    >
      {children}
    </div>
  )
}
