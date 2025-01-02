import { emojis } from "discord-emoji-converter"

class Emoji {
    code: string

    constructor() {
        this.code = ""
    }

    declareList<T>(name: string, items: T[]) {
        let result = `list ${name} = [\n`

        for (let i = 0; i < items.length; i++) {
            const item = items[i]
            result += `    ${JSON.stringify(item)}${
                i === items.length - 1 ? "" : ","
            }\n`
        }
        result += "];\n"
        this.code += result
    }

    generateCode() {
        this.declareList("emoji_names", Object.keys(emojis))
        this.declareList("emojis", Object.values(emojis))
    }
}

const emoji_header = `
%define EMOJI(NAME) emojis[(NAME) in emoji_names]
`.slice(1)

const processor = new Emoji()
processor.code += emoji_header
processor.generateCode()
await Bun.write("std/emoji.gs", processor.code)
