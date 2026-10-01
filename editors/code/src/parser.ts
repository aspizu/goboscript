export type Severity = "error" | "warning"

export interface ParsedDiagnostic {
  severity: Severity
  message: string
  path: string | undefined
  line: number | undefined
  column: number | undefined
  helps: string[]
}

// oxlint-disable-next-line no-control-regex
const ANSI_PATTERN = /\x1b(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])/g

export function stripAnsi(text: string): string {
  return text.replace(ANSI_PATTERN, "")
}

const HEADLINE_PATTERN = /^(error|warning): (.*)$/
const LOCATION_PATTERN = /^\s*-->\s+(.*)$/
const PATH_LINE_COLUMN_PATTERN = /^(.*):(\d+):(\d+)$/
const HELP_FOOTER_PATTERN = /^\s*=\s*help: ?(.*)$/

export function parseDiagnostics(stderr: string): ParsedDiagnostic[] {
  const diagnostics: ParsedDiagnostic[] = []
  let current: ParsedDiagnostic | undefined
  for (const rawLine of stderr.split(/\r?\n/)) {
    const line = stripAnsi(rawLine)
    const headline = HEADLINE_PATTERN.exec(line)
    if (headline) {
      if (current) {
        diagnostics.push(current)
      }
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
    if (!current) {
      continue
    }
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
  if (current) {
    diagnostics.push(current)
  }
  return diagnostics
}
