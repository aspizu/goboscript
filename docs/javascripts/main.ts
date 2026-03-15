import {createCssVariablesTheme, createHighlighter} from "shiki"
import goboscriptGrammar from "./goboscript.tmGrammar.json"

// @ts-ignore
import scratchblocks from "scratchblocks"

const cssVars = createCssVariablesTheme({
    name: "css-variables",
    variablePrefix: "--shiki-",
    variableDefaults: {},
    fontStyle: true,
})

let highlighter: Awaited<ReturnType<typeof createHighlighter>> | undefined
async function getHighlighter() {
    if (highlighter) return highlighter
    highlighter = await createHighlighter({
        themes: [cssVars],
        langs: ["bash", "javascript", "json", goboscriptGrammar as any],
    })
    return highlighter
}

async function highlightAll() {
    document.querySelectorAll<HTMLPreElement>("pre>code").forEach(async (block) => {
        if (block.dataset.highlighted == "true") {
            return
        }
        block.dataset.highlighted = "true"
        // Detect language from class like "language-html"
        const langClass = [...block.classList].find((c) => c.startsWith("language-"))
        const lang = langClass && langClass.slice("language-".length)
        if (!lang || lang == "scratchblocks") return

        const code = block.textContent ?? ""

        const highlighted = (await getHighlighter()).codeToHtml(code, {
            lang,
            theme: "css-variables",
        })

        if (block.parentElement) {
            block.parentElement.outerHTML = highlighted
        }
    })
}

declare global {
    const document$: any
}

document$.subscribe(() => {
    highlightAll()
    scratchblocks.appendStyles()
    for (const element of document.querySelectorAll<HTMLDivElement>(
        ".language-scratchblocks",
    )) {
        if (element.dataset.rendered === "true") continue

        const renderOptions = {
            style: "scratch3",
            inline: false,
            languages: ["en"],
            scale: 0.8,
            ...scratchblocks,
        }

        const code = renderOptions.read(element, renderOptions)
        const doc = renderOptions.parse(code, renderOptions)
        const svg = renderOptions.render(doc, renderOptions)

        renderOptions.replace(element, svg, doc, renderOptions)
        element.dataset.rendered = "true"
    }
})
