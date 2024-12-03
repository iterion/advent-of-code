const std = @import("std");
const aoc2024 = @import("aoc2024/root.zig");

pub fn main() !void {
    std.debug.print("Advent of Code in Zig!\n", .{});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    const answer1 = try aoc2024.day01.partOne();
    try stdout.print("day 1 - part 1: {d}\n", .{answer1});
    const answer2 = try aoc2024.day01.partTwo();
    try stdout.print("day 1 - part 2: {d}\n", .{answer2});
    try stdout.print("day 2 - part 1: {d}\n", .{try aoc2024.day02.partOne()});
    try stdout.print("day 2 - part 2: {d}\n", .{try aoc2024.day02.partTwo()});
    try stdout.print("day 3 - part 1: {d}\n", .{try aoc2024.day03.partOne()});
    try stdout.print("day 3 - part 2: {d}\n", .{try aoc2024.day03.partTwo()});
    try bw.flush(); // Don't forget to flush!
}
