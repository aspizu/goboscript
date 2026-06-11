const eslintConfigPrettier = require("eslint-config-prettier")
const typescriptEslintPlugin = require("@typescript-eslint/eslint-plugin")
const typescriptEslintParser = require("@typescript-eslint/parser")

module.exports = [
  eslintConfigPrettier,
  {
    files: ["src/**/*.ts"],
    languageOptions: {
      ecmaVersion: 6,
      parser: typescriptEslintParser,
      parserOptions: {
        project: true,
        sourceType: "module",
        tsconfigRootDir: __dirname,
      },
    },
    plugins: {
      "@typescript-eslint": typescriptEslintPlugin,
    },
    rules: {
      camelcase: ["error"],
      eqeqeq: ["error", "always", { null: "ignore" }],
      curly: ["error", "multi-line"],
      "no-console": ["error", { allow: ["warn", "error"] }],
      "prefer-const": "error",
      "@typescript-eslint/no-unnecessary-type-assertion": "error",
      "@typescript-eslint/no-floating-promises": "error",
      "@typescript-eslint/consistent-type-imports": [
        "error",
        {
          prefer: "type-imports",
          fixStyle: "inline-type-imports",
        },
      ],
      "@typescript-eslint/no-import-type-side-effects": "error",
    },
  },
]
