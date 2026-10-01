declare class Scaffolding {
  width: number
  height: number
  resizeMode: "preserve-ratio" | "dynamic-resize" | "stretch"
  editableLists: boolean
  shouldConnectPeripherals: boolean
  usePackagedRuntime: boolean
  vm: import("scratch-vm")
  appendTo(element: HTMLElement): void
  relayout(): void
  setup(): void
  setUsername(username: string): void
  loadProject(project: ArrayBuffer | Uint8Array): Promise<void>
  start(): void
  greenFlag(): void
  stopAll(): void
}

declare module "@turbowarp/scaffolding/with-music" {
  export const Scaffolding: new () => Scaffolding
}
