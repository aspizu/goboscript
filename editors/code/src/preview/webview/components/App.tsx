import { useCallback, useState } from "preact/hooks"
import { useProject } from "../hooks/useProject"
import type { Logger } from "../lib/logger"
import { player } from "../lib/player"
import { Console, type ConsoleMessage } from "./Console"
import { Overlay } from "./Overlay"
import { Stage } from "./Stage"
import { Toolbar } from "./Toolbar"

export function App({ logger }: { logger: Logger }) {
  const { status, load, fail } = useProject(logger)
  const [messages, setMessages] = useState<ConsoleMessage[]>([])

  const reload = useCallback(() => {
    if (player !== undefined) {
      void load(player)
    }
  }, [load])

  const greenFlag = useCallback(() => {
    player?.greenFlag()
  }, [])

  const stop = useCallback(() => {
    player?.stopAll()
  }, [])

  return (
    <>
      <Toolbar
        ready={status.kind === "ready"}
        loading={status.kind === "loading"}
        onReload={reload}
        onGreenFlag={greenFlag}
        onStop={stop}
      />
      <Stage onReady={reload} onError={fail}>
        <Overlay status={status} />
      </Stage>
      <Console messages={messages} onClear={() => setMessages([])} />
    </>
  )
}
