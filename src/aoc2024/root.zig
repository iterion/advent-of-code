pub const day01 = @import("day01.zig");
pub const day02 = @import("day02.zig");
pub const day03 = @import("day03.zig");
pub const day04 = @import("day04.zig");
pub const day05 = @import("day05.zig");
pub const day06 = @import("day06.zig");
pub const day07 = @import("day07.zig");
pub const day08 = @import("day08.zig");
pub const day09 = @import("day09.zig");

test {
    @import("std").testing.refAllDecls(@This());
}
