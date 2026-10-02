import { render } from "preact"
import { App } from "./components/App"
import "./styles.css"

const root = document.getElementById("root")
if (root === null) {
  throw new Error("preview: missing #root element")
}

render(<App />, root)
