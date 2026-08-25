import dgram from "node:dgram";
import { Buffer } from "node:buffer";
import { assertValidPort, normalizeMac } from "./mac.ts";

/**
 * Construct a Wake-on-LAN magic packet from a MAC address.
 *
 * The packet is 6 bytes of 0xFF followed by the MAC address repeated 16 times,
 * for a total of 102 bytes.
 *
 * @param mac The MAC address, either as 12 hex characters or as six hex pairs
 * separated by ".", ":" or "-".
 * @returns The 102-byte magic packet.
 * @throws {Error} If the MAC address is malformed.
 */
export function createMagicPacket(mac: string): Buffer {
  const macBytes = Buffer.from(normalizeMac(mac), "hex");
  // Buffer.from(..., "hex") truncates instead of throwing on bad input, so the
  // length is verified rather than assumed.
  if (macBytes.length !== 6) {
    throw new Error(`Invalid MAC address: ${JSON.stringify(mac)}`);
  }

  const packet = Buffer.alloc(102, 0xff);
  for (let i = 0; i < 16; i++) {
    macBytes.copy(packet, 6 + i * 6);
  }
  return packet;
}

/**
 * The library entrypoint to send a Wake-on-LAN magic packet.
 *
 * The returned promise resolves only once the packet has actually been handed
 * to the operating system and the socket has been closed, and rejects if the
 * MAC address, the port, or the send itself is invalid.
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

  return await new Promise<void>((resolve, reject) => {
    const socket = dgram.createSocket("udp4");
    let settled = false;

    const finish = (err?: Error | null) => {
      if (settled) {
        return;
      }
      settled = true;
      const done = () => (err ? reject(err) : resolve());
      try {
        socket.close(done);
      } catch {
        // The socket was never bound, or is already closed.
        done();
      }
    };

    socket.on("error", finish);

    socket.bind(() => {
      try {
        socket.setBroadcast(true);
      } catch (err) {
        finish(err as Error);
        return;
      }

      socket.send(magicPacket, 0, magicPacket.length, port, ip, finish);
    });
  });
}
