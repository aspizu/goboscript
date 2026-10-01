import { render } from "preact"
import { App } from "./components/App"
import { postToHost } from "./lib/host"
import { Logger } from "./lib/logger"
import "./styles.css"

const root = document.getElementById("root")
if (root === null) {
  throw new Error("preview: missing #root element")
}

render(<App logger={new Logger(postToHost)} />, root)
