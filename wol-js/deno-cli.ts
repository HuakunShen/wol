import { wakeOnLan } from "./src/deno-mod.ts";
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
  run({ args }) {
    wakeOnLan(args.mac, args.ip, parseInt(args.port));
  },
});

runMain(main);
