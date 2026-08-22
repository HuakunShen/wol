# WakeOnLan

JSR: https://jsr.io/@hk/wol/

## Node / Bun

```ts
import { wakeOnLan } from "wol-js";

// Resolves once the packet has been handed to the OS; rejects on bad input.
await wakeOnLan("70:a8:d3:b8:8c:8d", "255.255.255.255", 9);
```

CLI:

```bash
npx wol-js 70:a8:d3:b8:8c:8d --ip 255.255.255.255 --port 9
```

## Deno

The Deno implementation uses `Deno.listenDatagram`, which is an unstable API.
Run with `--unstable-net`, or add `{"unstable": ["net"]}` to your `deno.json`.

```ts
import { wakeOnLan } from "jsr:@hk/wol/deno";

await wakeOnLan("70:a8:d3:b8:8c:8d", "255.255.255.255", 9);
```

The `jsr:@hk/wol` entrypoint uses `node:dgram` through Deno's node compatibility
layer and needs no unstable flag.

The CLI (`deno-cli.ts`) is not part of the published package; run it from a
checkout with `deno task cli 70:a8:d3:b8:8c:8d`.

## MAC address formats

Both entrypoints accept 12 hex characters (`aabbccddeeff`) or six hex pairs
separated by `.`, `:` or `-`. Anything else throws instead of silently sending a
malformed packet.

## Development

```bash
bun install
bun run build   # emits dist/mod.js, dist/cli.js and the .d.ts files
deno check src/deno-mod.ts deno-cli.ts
```
