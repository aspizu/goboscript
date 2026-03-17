import Ajv from "ajv"
import betterAjvErrors from "better-ajv-errors"
import definitions from "./sb3_definitions.json"
import schema from "./sb3_schema.json"

const ajv = new Ajv({strict: false})
ajv.addSchema(definitions)

const filename: string = process.argv[2] || "-"
const json: string =
    filename == "-" ? await Bun.stdin.text() : await Bun.file(filename).text()
let data: unknown
try {
    data = JSON.parse(json)
} catch (error: any) {
    console.error(`error: \`${filename}\` -- ${error.message}`)
    process.exit(1)
}
const validate = ajv.compile(schema)
const valid = validate(data)

if (!valid) {
    const output = betterAjvErrors(schema, data, validate.errors!, {json})
    console.log(output)
    process.exit(1)
}
