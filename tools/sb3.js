"use strict";
const fs = require("fs");
const definitions = require("./sb3_definitions.json");
const schema = require("./sb3_schema.json");
const betterAjvErrors = require("better-ajv-errors").default;
const Ajv = require("ajv");
const ajv = new Ajv({ strict: false });
ajv.addSchema(definitions);
let jsonData;
if (process.argv[2]) {
  jsonData = fs.readFileSync(process.argv[2]).toString();
} else {
  // stdio
  jsonData = fs.readFileSync(0).toString();
}
const data = JSON.parse(jsonData);
const validate = ajv.compile(schema);
const valid = validate(data);
if (!valid) {
  const output = betterAjvErrors(ajv.schema, data, validate.errors, {
    json: jsonData,
  });
  console.log(output);
}
