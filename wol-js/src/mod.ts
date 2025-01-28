import dgram from "dgram";
import { Buffer } from "node:buffer";

/**
 * Construct a Wake-on-LAN magic packet from a MAC address.
 * @param mac
 * @returns
 */
export function createMagicPacket(mac: string): Buffer {
  if (mac.length !== 12) {
    throw new Error("MAC address has wrong length");
  }
  const macRepeat = mac.repeat(16);
  const macBytes = Buffer.from(macRepeat, "hex");
  const packet = Buffer.alloc(102, 0xff);
  macBytes.copy(packet, 6);
  return packet;
}

/**
 * The library entrypoint to send a Wake-on-LAN magic packet.
 * @param mac
 * @param ip
 * @param port
 */
export async function wakeOnLan(
  mac: string,
  ip: string,
  port: number = 9
): Promise<void> {
  if (mac.length === 12) {
    // regular case
  } else if (mac.length === 17) {
    const matched = /^([0-9A-Fa-f]{2}[\\.:-]){5}([0-9A-Fa-f]{2})$/.test(mac);
    if (matched) {
      mac = mac.replace(/[^0-9A-Fa-f]/g, "");
    } else {
      throw new Error("MAC address invalid format");
    }
  } else {
    throw new Error("MAC address wrong length");
  }

  const magicPacket = createMagicPacket(mac);

  const socket = dgram.createSocket("udp4");
  socket.bind(() => {
    socket.setBroadcast(true);

    // Send the broadcast message
    socket.send(magicPacket, 0, magicPacket.length, port, ip, (err) => {
      if (err) {
        console.error("Error sending broadcast:", err);
      } else {
        // console.log("Broadcast message sent!");
      }

      // Close the socket
      socket.close();
    });
  });
}
