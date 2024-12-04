const std = @import("std");
const first_example: []const u8 =
    \\3   4
    \\4   3
    \\2   5
    \\1   3
    \\3   9
    \\3   3
;
const first_example_answer: i64 = 11;
const second_example_answer: i64 = 31;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();
const ArrayList = std.ArrayList;

test "part one" {
    const answer = try solvePartOne(first_example);
    try std.testing.expectEqual(answer, first_example_answer);
}

test "part two" {
    const answer = try solvePartTwo(first_example);
    try std.testing.expectEqual(answer, second_example_answer);
}

pub fn solvePartOne(input: []const u8) !i64 {
    var list1 = ArrayList(i64).init(allocator);
    var list2 = ArrayList(i64).init(allocator);
    defer list1.deinit();
    defer list2.deinit();
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |x| {
        var items = std.mem.tokenizeScalar(u8, x, ' ');
        const item1 = try std.fmt.parseInt(i64, items.next() orelse "0", 10);
        const item2 = try std.fmt.parseInt(i64, items.next() orelse "0", 10);
        try list1.append(item1);
        try list2.append(item2);
    }

    std.mem.sort(i64, list1.items, {}, std.sort.asc(i64));
    std.mem.sort(i64, list2.items, {}, std.sort.asc(i64));
    const list_size = list1.items.len;
    const distances = try allocator.alloc(i64, list_size);
    defer allocator.free(distances);

    for (list1.items, 0..) |first_item, i| {
        const second_item = list2.items[i];
        distances[i] = @as(i64, @intCast(@abs(second_item - first_item)));
    }

    var sum: i64 = 0;
    for (distances) |d| {
        sum += d;
    }

    return sum;
}

pub fn solvePartTwo(input: []const u8) !i64 {
    var list1 = ArrayList(i64).init(allocator);
    var list2 = ArrayList(i64).init(allocator);
    defer list1.deinit();
    defer list2.deinit();
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |x| {
        var items = std.mem.tokenizeScalar(u8, x, ' ');
        const item1 = try std.fmt.parseInt(i64, items.next() orelse "0", 10);
        const item2 = try std.fmt.parseInt(i64, items.next() orelse "0", 10);
        try list1.append(item1);
        try list2.append(item2);
    }

    var sum: i64 = 0;
    for (list1.items) |needle| {
        var count: i64 = 0;
        for (list2.items) |maybe_needle| {
            if (needle == maybe_needle) {
                count += 1;
            }
        }
        sum += needle * count;
    }

    return sum;
}
