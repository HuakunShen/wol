import { wakeOnLan } from "./src/mod";
import { defineCommand, runMain } from "citty";
import { version } from "./package.json";

const main = defineCommand({
  meta: {
    name: "wol",
    version: version,
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
