import type { ComponentChild } from "preact"

export function IconButton({
  title,
  disabled,
  active,
  onClick,
  class: className,
  children,
}: {
  title: string
  disabled?: boolean
  active?: boolean
  onClick: () => void
  class?: string
  children: ComponentChild
}) {
  return (
    <button
      type="button"
      title={title}
      disabled={disabled}
      onClick={onClick}
      class={`flex h-6 w-6 items-center justify-center rounded-[5px] bg-transparent p-0 text-xs text-vscode-icon transition-transform duration-75 enabled:cursor-pointer enabled:hover:bg-vscode-hover enabled:active:scale-95 enabled:active:bg-vscode-active disabled:cursor-default disabled:opacity-40 ${active ? "bg-vscode-active" : ""} ${className ?? ""}`}
    >
      {children}
    </button>
  )
}
