import { Scaffolding as ScaffoldingConstructor } from "@turbowarp/scaffolding/with-music"

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
  player = scaffolding
} catch (err) {
  playerInitFailure = {
    detail: err instanceof Error ? err.message : String(err),
    log: `player init failed: ${err instanceof Error ? err.stack : String(err)}`,
  }
}

export { player, playerInitFailure }
