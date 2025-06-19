document$.subscribe(() => {
    scratchblocks.appendStyles()
    for (const element of document.querySelectorAll(".scratchblocks")) {
        const htmlElement = /** @type {HTMLElement} */ (element)
        if (htmlElement.dataset.rendered === "true") continue

        /** @type {ScratchBlocksOptions} */
        const renderOptions = {
            style: "scratch3",
            inline: false,
            languages: ["en"],
            scale: 0.8,
            ...scratchblocks,
        }

        const code = renderOptions.read(htmlElement, renderOptions)
        const doc = renderOptions.parse(code, renderOptions)
        const svg = renderOptions.render(doc, renderOptions)

        renderOptions.replace(htmlElement, svg, doc, renderOptions)
        htmlElement.dataset.rendered = "true"
    }
})
