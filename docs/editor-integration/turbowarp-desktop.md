# TurboWarp Desktop

## Instant Project Reload

This will add a keyboard shortcut to instantly reload the project in
TurboWarp Desktop using ++ctrl+b++

Open the user data folder using: 
`Settings` > `Desktop Settings` > `Open User Data`

Create a file called `userscript.js` and add the following code:

```js
window.addEventListener('keyup', async (event) => {
  if (!(event.key === 'b' && event.ctrlKey)) return
  event.preventDefault()
  const id = await EditorPreload.getInitialFile()
  if (id === null) return
  const file = await EditorPreload.getFile(id)
  await vm.loadProject(file.data)
})
```

Restart TurboWarp Desktop including all open windows.

Thanks to GarboMuffin for this code.
