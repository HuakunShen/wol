import { describe, expect, test } from "bun:test";
import dgram from "node:dgram";
import { Buffer } from "node:buffer";
import { createMagicPacket, wakeOnLan } from "./mod.ts";
import { normalizeMac, parsePort } from "./mac.ts";

describe("normalizeMac", () => {
  test("accepts bare and separated forms", () => {
    for (const mac of [
      "aabbccddeeff",
      "AABBCCDDEEFF",
      "aa:bb:cc:dd:ee:ff",
      "aa-bb-cc-dd-ee-ff",
      "aa.bb.cc.dd.ee.ff",
    ]) {
      expect(normalizeMac(mac).toLowerCase()).toBe("aabbccddeeff");
    }
  });

  test("rejects non-hex characters instead of truncating", () => {
    // Buffer.from(..., "hex") used to silently drop these, producing a packet
    // made mostly of the 0xff fill.
    expect(() => normalizeMac("aabbccddeezz")).toThrow(/Invalid MAC address/);
    expect(() => normalizeMac("zzzzzzzzzzzz")).toThrow(/Invalid MAC address/);
  });

  test("rejects non-string input rather than passing it through", () => {
    // RegExp.test stringifies, so 123456789012 used to slip through and yield
    // an all-zero MAC on the Deno path.
    for (const bad of [123456789012, ["aabbccddeeff"], null, undefined, {}]) {
      // @ts-expect-error deliberately bypassing the type to test a JS caller
      expect(() => normalizeMac(bad)).toThrow(/Invalid MAC address/);
    }
  });

  test("rejects wrong lengths and stray separators", () => {
    expect(() => normalizeMac("aabbccddeef")).toThrow(/Invalid MAC address/);
    expect(() => normalizeMac("aa:bb:cc:dd:ee")).toThrow(/Invalid MAC address/);
    expect(() => normalizeMac("aa\\bb\\cc\\dd\\ee\\ff")).toThrow(/Invalid MAC address/);
  });
});

describe("parsePort", () => {
  test("parses valid ports", () => {
    expect(parsePort("9")).toBe(9);
    expect(parsePort(" 65535 ")).toBe(65535);
  });

  test("rejects garbage rather than falling back to NaN or a prefix", () => {
    expect(() => parsePort("abc")).toThrow(/Invalid port/);
    expect(() => parsePort("9abc")).toThrow(/Invalid port/);
    expect(() => parsePort("0")).toThrow(/Invalid port/);
    expect(() => parsePort("65536")).toThrow(/Invalid port/);
  });
});

describe("createMagicPacket", () => {
  test("is 6 sync bytes followed by the MAC 16 times", () => {
    const packet = createMagicPacket("aa:bb:cc:dd:ee:ff");
    expect(packet.length).toBe(102);
    expect(packet.subarray(0, 6)).toEqual(Buffer.alloc(6, 0xff));
    expect(packet.subarray(6).toString("hex")).toBe("aabbccddeeff".repeat(16));
  });
});

describe("wakeOnLan", () => {
  /** Bind a UDP listener on a free loopback port and report what it receives. */
  async function listener() {
    const server = dgram.createSocket("udp4");
    let packet: Buffer | undefined;
    const received = new Promise<Buffer>((resolve, reject) => {
      server.once("message", (msg) => {
        packet = msg;
        resolve(msg);
      });
      setTimeout(() => reject(new Error("no packet after 2s")), 2000).unref();
    });
    const port = await new Promise<number>((resolve) => {
      server.bind(0, "127.0.0.1", () => resolve(server.address().port));
    });
    return {
      port,
      received,
      arrived: () => packet !== undefined,
      close: () => server.close(),
    };
  }

  test("sends a well-formed packet", async () => {
    const server = await listener();
    try {
      await wakeOnLan("aa:bb:cc:dd:ee:ff", "127.0.0.1", server.port);
      const packet = await server.received;
      expect(packet.toString("hex")).toBe(
        "ff".repeat(6) + "aabbccddeeff".repeat(16)
      );
    } finally {
      server.close();
    }
  });

  test("the packet survives an immediate process.exit after the await", async () => {
    // The regression this guards: wakeOnLan used to be `async` without awaiting
    // the dgram callbacks, so it resolved while the datagram was still queued
    // and an immediate exit dropped it. Only a real subprocess can catch that.
    const server = await listener();
    try {
      const child = Bun.spawn(
        [
          process.execPath,
          "-e",
          `import { wakeOnLan } from "${import.meta.dir}/mod.ts";
           await wakeOnLan("aa:bb:cc:dd:ee:ff", "127.0.0.1", ${server.port});
           process.exit(0);`,
        ],
        { stdout: "pipe", stderr: "pipe" }
      );
      expect(await child.exited).toBe(0);

      const packet = await server.received;
      expect(packet.toString("hex")).toBe(
        "ff".repeat(6) + "aabbccddeeff".repeat(16)
      );
    } finally {
      server.close();
    }
  });

  test("rejects when the send itself fails", async () => {
    // An IPv6 destination on a udp4 socket: the old implementation only
    // console.error'd this and resolved anyway.
    await expect(wakeOnLan("aa:bb:cc:dd:ee:ff", "::1", 9)).rejects.toThrow();
  });

  test("rejects on a malformed MAC address", async () => {
    await expect(wakeOnLan("nope", "127.0.0.1", 9)).rejects.toThrow(
      /Invalid MAC address/
    );
  });

  test("rejects a non-string MAC that stringifies to valid hex", async () => {
    await expect(
      // @ts-expect-error deliberately bypassing the type to test a JS caller
      wakeOnLan(123456789012, "127.0.0.1", 9)
    ).rejects.toThrow(/Invalid MAC address/);
  });

  test("rejects on an out-of-range port", async () => {
    await expect(
      wakeOnLan("aa:bb:cc:dd:ee:ff", "127.0.0.1", 70000)
    ).rejects.toThrow(/Invalid port/);
  });
});
