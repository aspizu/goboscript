import type { LogLevel } from "../messages"

export class Logger {
  constructor(private readonly post: (message: unknown) => void) {}

  log(message: string): void {
    this.send("log", message)
  }

  warn(message: string): void {
    this.send("warn", message)
  }

  error(message: string): void {
    this.send("error", message)
  }

  private send(level: LogLevel, message: string): void {
    this.post({ type: "log", level, message })
  }
}
