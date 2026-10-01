import type { Status } from "../hooks/useProject"

export function Overlay({ status }: { status: Status }) {
  if (status.kind === "ready") {
    return null
  }
  return (
    <div class="z-10 flex flex-col items-center justify-center gap-2 p-4 text-center">
      {status.kind === "loading" && (
        <div class="h-6 w-6 animate-spin rounded-full border-2 border-t-vscode-foreground border-vscode-widget-border" />
      )}
      {status.kind === "error" && (
        <>
          <h1 class="text-[15px] font-semibold">{status.title}</h1>
          {status.detail !== "" && (
            <p class="whitespace-pre-wrap text-xs opacity-80">{status.detail}</p>
          )}
        </>
      )}
    </div>
  )
}
