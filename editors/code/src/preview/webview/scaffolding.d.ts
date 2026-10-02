declare class Scaffolding {
  width: number
  height: number
  resizeMode: "preserve-ratio" | "dynamic-resize" | "stretch"
  editableLists: boolean
  shouldConnectPeripherals: boolean
  usePackagedRuntime: boolean
  vm: import("scratch-vm") & {
    addAddonBlock(options: {
      procedureCode: string
      arguments: string[]
      hidden: boolean
      callback: (args: Record<string, VM.ScratchCompatibleValue>, util: VM.BlockUtility) => void
    }): void
  }
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
