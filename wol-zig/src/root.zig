const std = @import("std");
const posix = std.posix;
const testing = std.testing;

pub const Error = error{
    InvalidMacAddress,
    InvalidPort,
};

/// Parse a MAC address into its six raw bytes.
///
/// Accepts either 12 bare hexadecimal characters ("aabbccddeeff") or six
/// hexadecimal pairs separated by '.', ':' or '-' ("aa:bb:cc:dd:ee:ff").
/// Separators are only allowed in the positions where they belong.
pub fn parseMac(mac: []const u8) Error![6]u8 {
    var hex_chars: [12]u8 = undefined;

    switch (mac.len) {
        12 => for (mac, 0..) |c, i| {
            if (!std.ascii.isHex(c)) return error.InvalidMacAddress;
            hex_chars[i] = c;
        },
        17 => {
            var j: usize = 0;
            for (mac, 0..) |c, i| {
                if (i % 3 == 2) {
                    if (c != ':' and c != '-' and c != '.') return error.InvalidMacAddress;
                } else {
                    if (!std.ascii.isHex(c)) return error.InvalidMacAddress;
                    hex_chars[j] = c;
                    j += 1;
                }
            }
        },
        else => return error.InvalidMacAddress,
    }

    var out: [6]u8 = undefined;
    for (&out, 0..) |*byte, i| {
        // Every character was checked above, so this cannot fail.
        byte.* = std.fmt.parseUnsigned(u8, hex_chars[i * 2 ..][0..2], 16) catch unreachable;
    }
    return out;
}

/// Build the 102-byte magic packet: six 0xFF bytes followed by the MAC address
/// repeated 16 times.
pub fn createMagicPacket(mac: [6]u8) [102]u8 {
    var packet: [102]u8 = undefined;
    @memset(packet[0..6], 0xff);
    for (0..16) |i| {
        @memcpy(packet[6 + i * 6 ..][0..6], &mac);
    }
    return packet;
}

/// Send a Wake-on-LAN magic packet for `mac` to `ip`:`port`.
pub fn wakeOnLan(mac: []const u8, ip: []const u8, port: u16) !void {
    if (port == 0) return error.InvalidPort;

    const packet = createMagicPacket(try parseMac(mac));
    const addr = try std.net.Address.parseIp4(ip, port);

    const sockfd = try posix.socket(posix.AF.INET, posix.SOCK.DGRAM, posix.IPPROTO.UDP);
    defer posix.close(sockfd);

    // Without SO_BROADCAST, sending to 255.255.255.255 is refused.
    try posix.setsockopt(
        sockfd,
        posix.SOL.SOCKET,
        posix.SO.BROADCAST,
        &std.mem.toBytes(@as(c_int, 1)),
    );

    _ = try posix.sendto(sockfd, &packet, 0, &addr.any, addr.getOsSockLen());
}

test "parseMac accepts bare and separated forms" {
    const want = [6]u8{ 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };
    for ([_][]const u8{
        "aabbccddeeff",
        "AABBCCDDEEFF",
        "aa:bb:cc:dd:ee:ff",
        "aa-bb-cc-dd-ee-ff",
        "aa.bb.cc.dd.ee.ff",
    }) |mac| {
        try testing.expectEqual(want, try parseMac(mac));
    }
}

test "parseMac rejects malformed input" {
    for ([_][]const u8{
        "",
        "aabbccddeef",
        "aabbccddeezz",
        "zzzzzzzzzzzz",
        "aa1bb1cc1dd1ee1ff",
        "a-a-b-b-c-c-d-d-e-e-f-f",
        "aa:bb:cc:dd:ee:ff:",
        "aa\\bb\\cc\\dd\\ee\\ff",
    }) |mac| {
        try testing.expectError(error.InvalidMacAddress, parseMac(mac));
    }
}

test "createMagicPacket layout" {
    const packet = createMagicPacket(try parseMac("aa:bb:cc:dd:ee:ff"));
    try testing.expectEqual(@as(usize, 102), packet.len);
    try testing.expectEqualSlices(u8, &[_]u8{0xff} ** 6, packet[0..6]);
    for (0..16) |i| {
        try testing.expectEqualSlices(
            u8,
            &[_]u8{ 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff },
            packet[6 + i * 6 ..][0..6],
        );
    }
}

test "wakeOnLan rejects port zero" {
    try testing.expectError(error.InvalidPort, wakeOnLan("aabbccddeeff", "127.0.0.1", 0));
}

test "wakeOnLan sends a well-formed packet" {
    const sockfd = try posix.socket(posix.AF.INET, posix.SOCK.DGRAM, posix.IPPROTO.UDP);
    defer posix.close(sockfd);

    const bind_addr = try std.net.Address.parseIp4("127.0.0.1", 0);
    try posix.bind(sockfd, &bind_addr.any, bind_addr.getOsSockLen());

    var bound: std.net.Address = undefined;
    var bound_len: posix.socklen_t = @sizeOf(std.net.Address);
    try posix.getsockname(sockfd, &bound.any, &bound_len);

    try wakeOnLan("aa:bb:cc:dd:ee:ff", "127.0.0.1", bound.getPort());

    var buf: [128]u8 = undefined;
    const n = try posix.recv(sockfd, &buf, 0);
    try testing.expectEqualSlices(u8, &createMagicPacket(try parseMac("aabbccddeeff")), buf[0..n]);
}
