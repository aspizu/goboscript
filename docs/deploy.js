import { $ } from "bun";
import * as path from "node:path";
await $`uv run --with zensical zensical build`;
const files = (
  await Array.fromAsync($`find site -type f -name '*.html'`.lines())
).filter(Boolean);
for (const file of files) {
  const text = await Bun.file(file).text();
  const newText = text.replace(
    /<head>/,
    `<head><base href="/goboscript/docs/${path.dirname(file).slice("site/".length)}/">`,
  );
  await Bun.write(file, newText);
}
