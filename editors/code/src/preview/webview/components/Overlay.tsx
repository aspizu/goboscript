import type { Status } from "../hooks/useProject"

export function Overlay({ status }: { status: Status }) {
  if (status.kind !== "error") {
    return null
  }
  return (
    <div class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 p-4 text-center">
      <h1 class="text-[15px] font-semibold">{status.title}</h1>
      {status.detail !== "" && (
        <p class="whitespace-pre-wrap text-xs opacity-80">{status.detail}</p>
      )}
    </div>
  )
}
