import { rm } from "node:fs/promises";

const shared = {
  target: "node",
  format: "esm",
  // citty is a runtime dependency; leave it to the consumer's node_modules.
  external: ["citty"],
  outdir: "./dist",
} as const;

await rm("./dist", { recursive: true, force: true });

const library = await Bun.build({
  ...shared,
  entrypoints: ["./src/mod.ts"],
});

const cli = await Bun.build({
  ...shared,
  entrypoints: ["./cli.ts"],
  banner: "#!/usr/bin/env node",
});

for (const result of [library, cli]) {
  if (!result.success) {
    for (const log of result.logs) {
      console.error(log);
    }
    process.exit(1);
  }
}

console.log("built dist/mod.js and dist/cli.js");
