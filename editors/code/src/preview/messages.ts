import type { Result } from "./result"

export type LogLevel = "log" | "warn" | "error"

export interface LoadProjectRequest {
  type: "loadProject"
  id: number
}

export interface LogNotification {
  type: "log"
  level: LogLevel
  message: string
}

export type WebviewMessage = LoadProjectRequest | LogNotification

export interface LoadProjectResponse {
  type: "loadProjectResult"
  id: number
  result: Result<string>
}

export type HostMessage = LoadProjectResponse
