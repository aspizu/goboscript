import { Console } from "./Console"
import { Overlay } from "./Overlay"
import { Stage } from "./Stage"
import { Toolbar } from "./Toolbar"

export function App() {
  return (
    <>
      <Toolbar />
      <Stage>
        <Overlay />
      </Stage>
      <Console />
    </>
  )
}
