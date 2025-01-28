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
  let macBytes: number[];
  if (mac.length === 12) {
    // Convert raw hex string to bytes
    macBytes = mac.match(/.{2}/g)!.map((byte) => parseInt(byte, 16));
  } else if (mac.length === 17) {
    const matched = /^([0-9A-Fa-f]{2}[\\.:-]){5}([0-9A-Fa-f]{2})$/.test(mac);
    if (matched) {
      // Remove separators and convert to bytes
      mac = mac.replace(/[^0-9A-Fa-f]/g, "");
      macBytes = mac.match(/.{2}/g)!.map((byte) => parseInt(byte, 16));
    } else {
      throw new Error("MAC address invalid format");
    }
  } else {
    throw new Error("MAC address wrong length");
  }
  const magicPacket = new Uint8Array(102);

  // Fill the first 6 bytes with 0xFF
  magicPacket.fill(0xff, 0, 6);

  // Repeat the MAC address 16 times
  for (let i = 0; i < 16; i++) {
    magicPacket.set(macBytes, 6 + i * 6);
  }

  // Create a UDP socket
  const socket = Deno.listenDatagram({
    transport: "udp",
    hostname: "0.0.0.0",
    port: 0, // Let the OS choose a port
  });

  // Send the magic packet to the broadcast address
  // deno-lint-ignore
  await socket.send(magicPacket, { transport: "udp", hostname: ip, port });
  socket.close();
}
