import { wakeOnLan } from "./src/deno-mod.ts";
import { parsePort } from "./src/mac.ts";
import { defineCommand, runMain } from "citty";
import pkg from "./package.json" with { type: "json" };

const main = defineCommand({
  meta: {
    name: "wol",
    version: pkg.version,
    description: "WakeOnLan CLI",
  },
  args: {
    mac: {
      type: "positional",
      description: "Mac",
      required: true,
    },
    ip: {
      type: "string",
      description: "broadcast ip",
      default: "255.255.255.255",
    },
    port: {
      type: "string",
      description: "port",
      default: "9",
    },
  },
  async run({ args }) {
    try {
      await wakeOnLan(args.mac, args.ip, parsePort(args.port));
    } catch (err) {
      console.error(`wol: ${err instanceof Error ? err.message : err}`);
      Deno.exitCode = 1;
    }
  },
});

runMain(main);
