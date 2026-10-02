import { spawn } from "node:child_process"
import type { CompilerFailure, CompilerResolution } from "./compiler"
import { parseDiagnostics, type ParsedDiagnostic } from "./parser"

export type Crash =
  | { kind: "timeout" }
  | { kind: "signal"; signal: string }
  | { kind: "exit"; code: number }

export interface BuildOutcome {
  parsed: ParsedDiagnostic[]
  savedDoc: string | undefined
  stderr: string
  error: string | undefined
  failure: CompilerFailure | undefined
  crash: Crash | undefined
}

export interface ProjectRunnerOptions {
  debounceMs: number
  timeoutMs: number
  resolveCompiler: () => CompilerResolution
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
    let timedOut = false
    let timeout: NodeJS.Timeout | undefined
    const finish = (result: {
      error?: string
      failure?: CompilerFailure
      crash?: Crash
      parsed?: ParsedDiagnostic[]
    }) => {
      if (done) {
        return
      }
      done = true
      if (timeout) {
        clearTimeout(timeout)
      }
      this.inFlight = false
      const abnormal = result.error !== undefined || result.failure !== undefined
      this.options.onResult({
        parsed: result.parsed ?? (abnormal ? [] : parseDiagnostics(stderr)),
        savedDoc,
        stderr,
        error: result.error,
        failure: result.failure,
        crash: abnormal ? undefined : result.crash,
      })
      if (this.dirty) {
        this.dirty = false
        this.run()
      }
    }
    const { command, failure } = this.options.resolveCompiler()
    if (failure !== undefined) {
      finish({ failure })
      return
    }
    const child = spawn(command, ["build"], {
      cwd: this.projectDir,
      stdio: ["ignore", "pipe", "pipe"],
    })
    timeout = setTimeout(() => {
      timedOut = true
      child.kill()
    }, this.options.timeoutMs)
    child.stdout.on("data", () => {})
    child.stderr.setEncoding("utf8")
    child.stderr.on("data", (chunk: string) => {
      stderr += chunk
    })
    child.on("error", (err) => finish({ error: `failed to spawn ${command}: ${err.message}` }))
    child.on("close", (code, signal) => {
      const parsed = parseDiagnostics(stderr)
      if (signal !== null) {
        finish(
          timedOut
            ? { crash: { kind: "timeout" }, parsed }
            : { crash: { kind: "signal", signal }, parsed },
        )
      } else if (code !== null && code !== 0 && parsed.length === 0) {
        finish({ crash: { kind: "exit", code }, parsed })
      } else {
        finish({ parsed })
      }
    })
  }
}
