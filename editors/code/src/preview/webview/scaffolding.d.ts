// The bundled TurboWarp Scaffolding player exposes no types for its prebuilt
// bundles. This mirrors the shape of the package from its own `types.d.ts`
// and README, typed against `@turbowarp/types` (the scratch-vm typings).
// When bundled, the UMD wrapper assigns its exports object to `module.exports`,
// so a default import yields `{ Scaffolding, CloudVariables, Packages }`.
//
// This file is an ambient script (no imports/exports): the declarations are
// visible to every file in `tsconfig.webview.json`.

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
  const bundle: {
    Scaffolding: new () => Scaffolding
  }
  export default bundle
}
