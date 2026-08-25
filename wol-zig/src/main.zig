const std = @import("std");
const wol = @import("wol");

const usage =
    \\usage: wol-zig <mac> [broadcast-ip] [port]
    \\
    \\  mac            MAC address, e.g. aa:bb:cc:dd:ee:ff or aabbccddeeff
    \\  broadcast-ip   broadcast address to send to (default: 255.255.255.255)
    \\  port           UDP port (default: 9)
    \\
;

pub fn main(init: std.process.Init) !u8 {
    const args = try init.minimal.args.toSlice(init.arena.allocator());

    for (args[1..]) |arg| {
        if (std.mem.eql(u8, arg, "-h") or std.mem.eql(u8, arg, "--help")) {
            std.debug.print("{s}", .{usage});
            return 0;
        }
    }

    if (args.len < 2 or args.len > 4) {
        std.debug.print("{s}", .{usage});
        return 1;
    }

    const mac = args[1];
    const ip = if (args.len > 2) args[2] else "255.255.255.255";
    const port: u16 = if (args.len > 3)
        std.fmt.parseUnsigned(u16, args[3], 10) catch {
            std.debug.print(
                "wol: invalid port '{s}': expected an integer between 1 and 65535\n",
                .{args[3]},
            );
            return 1;
        }
    else
        9;

    wol.wakeOnLan(init.io, mac, ip, port) catch |err| {
        switch (err) {
            error.InvalidMacAddress => std.debug.print(
                "wol: invalid MAC address '{s}': expected 12 hex characters, optionally separated by '.', ':' or '-'\n",
                .{mac},
            ),
            error.InvalidPort => std.debug.print(
                "wol: invalid port {d}: expected an integer between 1 and 65535\n",
                .{port},
            ),
            else => std.debug.print("wol: {s}\n", .{@errorName(err)}),
        }
        return 1;
    };

    return 0;
}
