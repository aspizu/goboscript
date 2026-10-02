import { err, ok, type Result as NeverthrowResult } from "neverthrow"

export type Result<T> = { ok: true; value: T } | { ok: false; error: string }

export function serializeResult<T>(result: NeverthrowResult<T, string>): Result<T> {
  return result.isOk() ? { ok: true, value: result.value } : { ok: false, error: result.error }
}

export function deserializeResult<T>(result: Result<T>): NeverthrowResult<T, string> {
  return result.ok ? ok(result.value) : err(result.error)
}
