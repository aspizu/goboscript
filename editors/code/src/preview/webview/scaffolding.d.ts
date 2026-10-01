// The bundled TurboWarp Scaffolding player exposes no types for its prebuilt
// bundles. This mirrors the shape of the package from its own `types.d.ts`
// and README, typed against `@turbowarp/types` (the scratch-vm typings).
// The prebuilt UMD bundle exports the class as a named export; under
// esbuild's CJS interop the default import is undefined because webpack
// marks the entry module with `__esModule`.

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
