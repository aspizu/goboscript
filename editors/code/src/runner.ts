import { spawn } from "node:child_process"
import { parseDiagnostics, type ParsedDiagnostic } from "./parser"

export interface BuildOutcome {
  parsed: ParsedDiagnostic[]
  savedDoc: string | undefined
  error: string | undefined
}

export interface ProjectRunnerOptions {
  debounceMs: number
  timeoutMs: number
  onResult: (outcome: BuildOutcome) => void
}

export class ProjectRunner {
  private timer: NodeJS.Timeout | undefined
  private inFlight = false
  private dirty = false
  private savedDoc: string | undefined

  constructor(
    public readonly projectDir: string,
    private readonly options: ProjectRunnerOptions,
  ) {}

  schedule(savedDoc: string): void {
    this.savedDoc = savedDoc
    if (this.inFlight) {
      this.dirty = true
      return
    }
    if (this.timer) {
      clearTimeout(this.timer)
    }
    this.timer = setTimeout(() => {
      this.timer = undefined
      this.run()
    }, this.options.debounceMs)
  }

  dispose(): void {
    if (this.timer) {
      clearTimeout(this.timer)
    }
    this.timer = undefined
  }

  private run(): void {
    this.inFlight = true
    const savedDoc = this.savedDoc
    let stderr = ""
    let done = false
    const child = spawn("goboscript", ["build"], {
      cwd: this.projectDir,
      stdio: ["ignore", "pipe", "pipe"],
    })
    const timeout = setTimeout(() => child.kill(), this.options.timeoutMs)
    const finish = (error: string | undefined) => {
      if (done) {
        return
      }
      done = true
      clearTimeout(timeout)
      this.inFlight = false
      this.options.onResult({
        parsed: error ? [] : parseDiagnostics(stderr),
        savedDoc,
        error,
      })
      if (this.dirty) {
        this.dirty = false
        this.run()
      }
    }
    child.stdout.on("data", () => {})
    child.stderr.setEncoding("utf8")
    child.stderr.on("data", (chunk: string) => {
      stderr += chunk
    })
    child.on("error", (err) => finish(`failed to spawn goboscript: ${err.message}`))
    child.on("close", (_code, signal) => {
      if (signal) {
        finish(`goboscript killed by ${signal} after ${this.options.timeoutMs}ms`)
      } else {
        finish(undefined)
      }
    })
  }
}
