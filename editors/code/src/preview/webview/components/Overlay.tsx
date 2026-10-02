import { status } from "../lib/project"

export function Overlay() {
  const current = status.value
  if (current.kind !== "error") {
    return null
  }
  return (
    <div class="absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 p-4 text-center">
      <h1 class="text-[15px] font-semibold">{current.title}</h1>
      {current.detail !== "" && (
        <p class="whitespace-pre-wrap text-xs opacity-80">{current.detail}</p>
      )}
    </div>
  )
}
