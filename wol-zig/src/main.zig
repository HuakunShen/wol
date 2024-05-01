const std = @import("std");
const lib = @import("root.zig");

pub fn createMagicPacket(mac_addr: [6]u8) [102]u8 {
    return [_]u8{0xff} ** 6 ++ mac_addr ** 16;
}

pub fn main() !void {
    // read argv[2] from command line
    // defer std.process.argsFree(args);
    const args = try std.process.argsAlloc(std.heap.page_allocator);
    defer std.process.argsFree(std.heap.page_allocator, args);

    // Print each argument
    for (args, 0..) |arg, index| {
        std.debug.print("Argument {d}: {s}\n", .{ index, arg });
    }

    const arg_len = args.len;
    //     std.os.ErrNoMemory => {
    //         std.debug.print("Out of memory\n", .{});
    //         return err.OutOfMemory;
    //     },
    // };
    std.debug.print("arg_len: {}\n", .{arg_len});
    if (arg_len < 2) {
        return;
    }
    // get last argument from command line
    const mac_addr_str = args[args.len - 1];
    if (mac_addr_str.len != 12 and mac_addr_str.len != 17) {
        std.debug.print("Invalid MAC address length\n", .{});
        return;
    }
    // remove all colon from mac_addr_str
    var i: usize = 0;
    var j: usize = 0;
    var mac_addr_bytes: [6]u8 = undefined;
    while (i < mac_addr_str.len) {
        if (mac_addr_str[i] != ':') {
            // mac_addr[j] = std.fmt.parseUnsigned(u8, mac_addr_str[i .. i + 1], 16);
            std.debug.print("{s}\n", .{mac_addr_str[i .. i + 2]});
            const x = try std.fmt.parseUnsigned(u8, mac_addr_str[i .. i + 2], 16);
            // std.debug.print("x: {}\n", .{x});
            mac_addr_bytes[j] = x;
            j += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    // decode mac_addr from hex string
    // var number = try std.fmt.parseUnsigned(u8, mac_addr, 16);
    _ = createMagicPacket(mac_addr_bytes);
    // const UDP_PAYLOADSIZE = 65507;
    // var buf: [UDP_PAYLOADSIZE]u8 = .{};

    const sockfd = try std.os.socket(std.os.AF.INET, std.os.SOCK.DGRAM | std.os.SOCK.CLOEXEC, 0);
    defer std.os.closeSocket(sockfd);
    const udp = try std.net.UdpSocket.init(.{});
    defer udp.deinit();
    const addr = try std.net.Address.resolveIp("255.255.255.255", 9);
    try std.os.connect(sockfd, &addr.any, addr.getOsSockLen());

    // send magic packet to broadcast address
    // const udp_socket = try std.net.
    // .udp.Socket.init(std.testing.allocator, .IPv4);
    // defer udp_socket.deinit();
    // const broadcast_addr = try std.net.parseIpAddr("255.255.255.255");
    // const broadcast_sockaddr = try std.net.udp.SocketAddress.init(broadcast_addr, 9);
    // try udp_socket.send(broadcast_sockaddr, magic_packet);

    // // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    // std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // // stdout is for the actual output of your application, for example if you
    // // are implementing gzip, then only the compressed bytes should be sent to
    // // stdout, not any debugging messages.
    // const stdout_file = std.io.getStdOut().writer();
    // var bw = std.io.bufferedWriter(stdout_file);
    // const stdout = bw.writer();

    // try stdout.print("Run `zig build test` to run the tests.\n", .{});

    // try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
