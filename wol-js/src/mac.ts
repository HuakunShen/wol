/**
 * Shared MAC address and port validation, used by both the Node and the Deno
 * entrypoints so the two implementations accept exactly the same input.
 */

/** A bare MAC address: 12 hexadecimal characters, no separators. */
const MAC_BARE = /^[0-9A-Fa-f]{12}$/;

/** A separated MAC address: six hex pairs joined by ".", ":" or "-". */
const MAC_SEPARATED = /^([0-9A-Fa-f]{2}[.:-]){5}[0-9A-Fa-f]{2}$/;

/**
 * Normalize a MAC address to a bare 12-character hex string.
 *
 * Accepts either 12 hex characters (`"aabbccddeeff"`) or six hex pairs
 * separated by ".", ":" or "-" (`"aa:bb:cc:dd:ee:ff"`).
 *
 * @param mac The MAC address to normalize.
 * @returns The MAC address as 12 hexadecimal characters.
 * @throws {Error} If the MAC address is not one of the accepted formats.
 */
export function normalizeMac(mac: string): string {
  // `RegExp.test` stringifies its argument, so a non-string whose string form
  // looks like a MAC address would otherwise be returned unchanged and go on to
  // produce an all-zero packet.
  if (typeof mac === "string") {
    if (MAC_BARE.test(mac)) {
      return mac;
    }
    if (MAC_SEPARATED.test(mac)) {
      return mac.replace(/[.:-]/g, "");
    }
  }
  throw new Error(
    `Invalid MAC address: ${JSON.stringify(mac)}. ` +
      `Expected 12 hex characters, optionally separated by ".", ":" or "-".`
  );
}

/**
 * Convert a MAC address to its six raw bytes.
 *
 * @param mac The MAC address, in any format accepted by {@link normalizeMac}.
 * @returns Exactly six bytes.
 * @throws {Error} If the MAC address is not one of the accepted formats.
 */
export function macToBytes(mac: string): number[] {
  const normalized = normalizeMac(mac);
  const bytes: number[] = [];
  for (let i = 0; i < normalized.length; i += 2) {
    bytes.push(parseInt(normalized.slice(i, i + 2), 16));
  }
  if (bytes.length !== 6) {
    throw new Error(`Invalid MAC address: ${JSON.stringify(mac)}`);
  }
  return bytes;
}

/**
 * Validate a UDP port number.
 *
 * @param port The port to validate.
 * @throws {Error} If the port is not an integer in the range 1-65535.
 */
export function assertValidPort(port: number): void {
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`Invalid port: ${port}. Expected an integer between 1 and 65535.`);
  }
}

/**
 * Parse a port number given as a command-line string.
 *
 * Unlike a bare `parseInt`, this rejects trailing garbage ("9abc") and
 * non-numeric input ("abc") instead of silently returning 9 or NaN.
 *
 * @param value The raw command-line value.
 * @returns The parsed port.
 * @throws {Error} If the value is not an integer in the range 1-65535.
 */
export function parsePort(value: string): number {
  if (typeof value !== "string" || !/^[0-9]+$/.test(value.trim())) {
    throw new Error(`Invalid port: ${JSON.stringify(value)}. Expected an integer between 1 and 65535.`);
  }
  const port = Number(value.trim());
  assertValidPort(port);
  return port;
}
