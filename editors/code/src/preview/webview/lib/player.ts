import { Scaffolding as ScaffoldingConstructor } from "@turbowarp/scaffolding/with-music"
import { messages } from "./console"

export interface PlayerInitFailure {
  detail: string
  log: string
}

let player: Scaffolding | undefined
let playerInitFailure: PlayerInitFailure | undefined

try {
  const scaffolding = new ScaffoldingConstructor()
  scaffolding.resizeMode = "preserve-ratio"
  scaffolding.setup()
  for (const name of ["log", "warn", "error"] as const) {
    scaffolding.vm.addAddonBlock({
      procedureCode: `\u200b\u200b${name}\u200b\u200b %s`,
      arguments: ["message"],
      hidden: true,
      callback: ({ message }, { target }) => {
        const origin = target.isStage ? "Stage" : target.getName()
        messages.value = [
          ...messages.peek(),
          {
            level: name === "warn" ? "warning" : name,
            message: String(message),
            origin: target.isOriginal ? origin : `${origin} (clone)`,
          },
        ]
      },
    })
  }
  player = scaffolding
} catch (err) {
  playerInitFailure = {
    detail: err instanceof Error ? err.message : String(err),
    log: `player init failed: ${err instanceof Error ? err.stack : String(err)}`,
  }
}

export { player, playerInitFailure }
