
import { assertValidPort, macToBytes } from "./mac.ts";

/**
 * Build a Wake-on-LAN magic packet from a MAC address.
 *
 * The packet is 6 bytes of 0xFF followed by the MAC address repeated 16 times,
 * for a total of 102 bytes.
 *
 * @param mac The MAC address, either as 12 hex characters or as six hex pairs
 * separated by ".", ":" or "-".
 * @returns The 102-byte magic packet.
 * @throws {Error} If the MAC address is malformed.
 */
export function createMagicPacket(mac: string): Uint8Array {
  const macBytes = macToBytes(mac);
  if (macBytes.length !== 6) {
    throw new Error(`Invalid MAC address: ${JSON.stringify(mac)}`);
  }

  const magicPacket = new Uint8Array(102);
  magicPacket.fill(0xff, 0, 6);
  for (let i = 0; i < 16; i++) {
    magicPacket.set(macBytes, 6 + i * 6);
  }
  return magicPacket;
}

/**
 * The library entrypoint to send a Wake-on-LAN magic packet.
 *
 * Requires Deno's unstable net API: run with `--unstable-net`, or add
 * `{"unstable": ["net"]}` to your `deno.json`.
 *
 * @param mac The MAC address of the machine to wake.
 * @param ip The broadcast address to send to, e.g. "255.255.255.255".
 * @param port The UDP port, 9 by default.
 */
export async function wakeOnLan(
  mac: string,
  ip: string,
  port: number = 9
): Promise<void> {
  const magicPacket = createMagicPacket(mac);
  assertValidPort(port);

  if (typeof Deno.listenDatagram !== "function") {
    throw new Error(
      "Deno.listenDatagram is unavailable. It is an unstable API: run Deno " +
        'with --unstable-net, or add {"unstable": ["net"]} to your deno.json.'
    );
  }

  const socket = Deno.listenDatagram({
    transport: "udp",
    hostname: "0.0.0.0",
    port: 0, // Let the OS choose a port.
  });

  try {
    await socket.send(magicPacket, { transport: "udp", hostname: ip, port });
  } finally {
    socket.close();
  }
}
