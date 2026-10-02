import { useCallback, useState } from "preact/hooks"
import { useProject } from "../hooks/useProject"
import type { Logger } from "../lib/logger"
import { player } from "../lib/player"
import { Console, type ConsoleMessage } from "./Console"
import { Overlay } from "./Overlay"
import { Stage } from "./Stage"
import { Toolbar } from "./Toolbar"

const DUMMY_MESSAGES: ConsoleMessage[] = [
  { level: "info", message: "Project started", origin: "Stage" },
  { level: "log", message: "Loaded 3 sprites and 12 sounds", origin: "Loader" },
  { level: "log", message: "score is now 5", origin: "Score Keeper" },
  {
    level: "warning",
    message: "list 'leaderboard' has 11 items, expected at most 10",
    origin: "leaderboard",
  },
  {
    level: "error",
    message: "could not connect to cloud variables, retrying in 5 seconds",
    origin: "Cloud",
  },
  { level: "log", message: "Player touched the edge at (218, -14)", origin: "Player" },
]

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
    setMessages((previous) => [...previous, ...DUMMY_MESSAGES])
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
