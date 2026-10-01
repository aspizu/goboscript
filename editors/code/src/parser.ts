/**
 * Parsing of `goboscript build` diagnostics from stderr.
 *
 * The compiler renders diagnostics with `annotate-snippets` in rustc style:
 *
 * ```text
 * error: unrecognized token, expected one of `PROC`, `FUNC`, ...
 *  --> stage.gs:1:1
 *   |
 * 1 | when_flag_clicked()
 *   | ^^^^^^^^^^^^^^^^^^
 *   |
 * ```
 *
 * Diagnostics without a span (reported as `0..0`, e.g. "no costumes") or in
 * files that are not valid UTF-8 are rendered without an annotation, and the
 * origin line then carries the path only, without `:line:column`:
 *
 * ```text
 * error: no costumes
 *  --> main.gs
 *   |
 *   |
 *   = help: if this is a header, move it inside a directory such as `lib/`
 * ```
 *
 * Errors that are not attached to a sprite (e.g. "failed to read stage.gs")
 * are printed as a bare `error: ...` headline with no `-->` line at all.
 */

export type Severity = "error" | "warning"

export interface ParsedDiagnostic {
  severity: Severity
  message: string
  /**
   * Path as printed on the `-->` line, if the diagnostic had one. Relative to
   * the directory the compiler ran in, unless absolute (e.g. standard library
   * files outside the project).
   */
  path: string | undefined
  /** 1-based line number, if the location carried one. */
  line: number | undefined
  /** 1-based code point column, if the location carried one. */
  column: number | undefined
  /** `= help: ...` footer texts belonging to this diagnostic. */
  helps: string[]
}

const ANSI_PATTERN = /\x1b(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])/g

export function stripAnsi(text: string): string {
  return text.replace(ANSI_PATTERN, "")
}

const HEADLINE_PATTERN = /^(error|warning): (.*)$/
const LOCATION_PATTERN = /^\s*-->\s+(.*)$/
const PATH_LINE_COLUMN_PATTERN = /^(.*):(\d+):(\d+)$/
const HELP_FOOTER_PATTERN = /^\s*=\s*help: ?(.*)$/

/**
 * Extracts diagnostics from the (possibly ANSI colored) stderr of
 * `goboscript build`.
 *
 * Lines that do not contribute to a diagnostic are ignored. Notably, the
 * indented stderr block printed for `CommandFailed` diagnostics cannot
 * false-match because real headlines are anchored at the start of the line.
 */
export function parseDiagnostics(stderr: string): ParsedDiagnostic[] {
  const diagnostics: ParsedDiagnostic[] = []
  let current: ParsedDiagnostic | undefined
  for (const rawLine of stderr.split(/\r?\n/)) {
    const line = stripAnsi(rawLine)
    const headline = HEADLINE_PATTERN.exec(line)
    if (headline) {
      if (current) diagnostics.push(current)
      current = {
        severity: headline[1] as Severity,
        message: headline[2].trim(),
        path: undefined,
        line: undefined,
        column: undefined,
        helps: [],
      }
      continue
    }
    if (!current) continue
    if (current.path === undefined) {
      const location = LOCATION_PATTERN.exec(line)
      if (location) {
        const rest = location[1].trim()
        const pathLineColumn = PATH_LINE_COLUMN_PATTERN.exec(rest)
        if (pathLineColumn) {
          current.path = pathLineColumn[1]
          current.line = Number(pathLineColumn[2])
          current.column = Number(pathLineColumn[3])
        } else {
          current.path = rest
        }
        continue
      }
    }
    const helpFooter = HELP_FOOTER_PATTERN.exec(line)
    if (helpFooter) {
      current.helps.push(helpFooter[1].trim())
    }
  }
  if (current) diagnostics.push(current)
  return diagnostics
}
