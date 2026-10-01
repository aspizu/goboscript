import assert from "node:assert/strict"
import { describe, it } from "node:test"
import { parseDiagnostics, stripAnsi, type ParsedDiagnostic } from "./parser"

function errorHeadline(message: string): string {
  return `\x1b[1m\x1b[91merror\x1b[0m: \x1b[1m${message}\x1b[0m`
}

function warningHeadline(message: string): string {
  return `\x1b[1m\x1b[93mwarning\x1b[0m: \x1b[1m${message}\x1b[0m`
}

function location(path: string, line?: number, column?: number): string {
  const suffix = line === undefined ? "" : `:${line}:${column}`
  return ` \x1b[1m\x1b[94m-->\x1b[0m ${path}${suffix}`
}

function helpFooter(text: string): string {
  return ` \x1b[1m\x1b[94m= \x1b[0m\x1b[1m\x1b[96mhelp\x1b[0m: ${text}`
}

/** Fixture captured from a real build of a project with a syntax error. */
const SYNTAX_ERROR = [
  errorHeadline(
    "unrecognized token, expected one of `COSTUMES`, `SOUNDS`, `PROC`, `FUNC`",
  ),
  location("stage.gs", 1, 1),
  "  \x1b[1m\x1b[94m|\x1b[0m",
  "\x1b[1m\x1b[94m1 |\x1b[0m when_flag_clicked()",
  "  \x1b[1m\x1b[94m|\x1b[0m \x1b[1m\x1b[91m^^^^^^^^^^^^^^^^^^^\x1b[0m",
  "  \x1b[1m\x1b[94m|\x1b[0m",
  "",
  "Finished in 8.145208ms",
].join("\n")

/** Fixture captured from a real build of a sprite with no costumes. */
const NO_COSTUMES = [
  errorHeadline("no costumes"),
  location("main.gs"),
  " \x1b[1m\x1b[94m|\x1b[0m",
  " \x1b[1m\x1b[94m|\x1b[0m",
  helpFooter("if this is a header, move it inside a directory such as `lib/`"),
  "",
  "Emitted 249 blocks",
  "Finished in 2.761625ms",
].join("\n")

/** The frontend's catch-all path prints a bare headline with no snippet. */
const ANYHOW_ERROR = "error: failed to read stage.gs\n"

/** The `CommandFailed` diagnostic prints the command's indented stderr. */
const COMMAND_FAILED = [
  errorHeadline("command failed"),
  location("main.gs", 2, 5),
  "  \x1b[1m\x1b[94m|\x1b[0m",
  "\x1b[1m\x1b[94m2 |\x1b[0m $run_shell cargo build",
  "  \x1b[1m\x1b[94m|\x1b[0m     \x1b[1m\x1b[91m^^^^^^^^^^^^^^^^^^^^^^^^\x1b[0m",
  "  \x1b[1m\x1b[94m|\x1b[0m",
  "\x1b[1m\x1b[91mstderr\x1b[0m\x1b[1m:\x1b[0m",
  "    error: could not compile `goboscript` due to 1 previous error",
  "    warning: unused variable: `x`",
].join("\n")

const base = {
  path: undefined,
  line: undefined,
  column: undefined,
  helps: [] as string[],
}

describe("stripAnsi", () => {
  it("removes escape sequences", () => {
    assert.equal(stripAnsi("\x1b[1m\x1b[91merror\x1b[0m: hi"), "error: hi")
  })
})

describe("parseDiagnostics", () => {
  it("parses a located error", () => {
    assert.deepEqual(parseDiagnostics(SYNTAX_ERROR), [
      {
        ...base,
        severity: "error",
        message:
          "unrecognized token, expected one of `COSTUMES`, `SOUNDS`, `PROC`, `FUNC`",
        path: "stage.gs",
        line: 1,
        column: 1,
      },
    ] satisfies ParsedDiagnostic[])
  })

  it("parses a located warning", () => {
    const stderr = [
      warningHeadline("unknown costume format"),
      location("main.gs", 12, 34),
    ].join("\n")
    assert.deepEqual(parseDiagnostics(stderr), [
      {
        ...base,
        severity: "warning",
        message: "unknown costume format",
        path: "main.gs",
        line: 12,
        column: 34,
      },
    ] satisfies ParsedDiagnostic[])
  })

  it("keeps the location of a diagnostic without line and column", () => {
    const [diagnostic] = parseDiagnostics(NO_COSTUMES)
    assert.equal(diagnostic.severity, "error")
    assert.equal(diagnostic.message, "no costumes")
    assert.equal(diagnostic.path, "main.gs")
    assert.equal(diagnostic.line, undefined)
    assert.equal(diagnostic.column, undefined)
    assert.deepEqual(diagnostic.helps, [
      "if this is a header, move it inside a directory such as `lib/`",
    ])
  })

  it("keeps diagnostics without any location", () => {
    assert.deepEqual(parseDiagnostics(ANYHOW_ERROR), [
      { ...base, severity: "error", message: "failed to read stage.gs" },
    ] satisfies ParsedDiagnostic[])
  })

  it("supports paths containing spaces and colons", () => {
    const stderr = [
      errorHeadline("boom"),
      location("My Project: sprites/main 2.gs", 3, 7),
    ].join("\n")
    assert.deepEqual(parseDiagnostics(stderr), [
      {
        ...base,
        severity: "error",
        message: "boom",
        path: "My Project: sprites/main 2.gs",
        line: 3,
        column: 7,
      },
    ] satisfies ParsedDiagnostic[])
  })

  it("ignores the indented stderr of CommandFailed diagnostics", () => {
    assert.equal(parseDiagnostics(COMMAND_FAILED).length, 1)
  })

  it("parses multiple diagnostics", () => {
    const stderr = [
      errorHeadline("first"),
      location("stage.gs", 1, 1),
      errorHeadline("second"),
      location("main.gs", 52, 22),
      warningHeadline("third"),
      location("main.gs", 8, 3),
      helpFooter("try `var` instead"),
      "Finished in 2.759ms",
    ].join("\n")
    assert.deepEqual(
      parseDiagnostics(stderr).map(({ message, path, helps }) => ({
        message,
        path,
        helps,
      })),
      [
        { message: "first", path: "stage.gs", helps: [] },
        { message: "second", path: "main.gs", helps: [] },
        { message: "third", path: "main.gs", helps: ["try `var` instead"] },
      ],
    )
  })

  it("ignores non-diagnostic output", () => {
    assert.deepEqual(
      parseDiagnostics("Emitted 249 blocks\nFinished in 2.761625ms\n"),
      [],
    )
  })
})
