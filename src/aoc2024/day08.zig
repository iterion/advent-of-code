const std = @import("std");

const example: []const u8 =
    \\............
    \\........0...
    \\.....0......
    \\.......0....
    \\....0.......
    \\......A.....
    \\............
    \\............
    \\........A...
    \\.........A..
    \\............
    \\............
;
const first_example_answer: i64 = 14;
const second_example_answer: i64 = 34;

test "part one" {
    const allocator = std.testing.allocator;
    const answer = try solvePartOne(allocator, example);
    try std.testing.expectEqual(answer, first_example_answer);
}

test "part two" {
    const allocator = std.testing.allocator;
    const answer = try solvePartTwo(allocator, example);
    try std.testing.expectEqual(answer, second_example_answer);
}

const CoordinateList = std.ArrayList(Coordinate);

const Coordinate = struct {
    x: i64,
    y: i64,
};

const Board = struct {
    signals: std.AutoHashMap(u8, CoordinateList),
    x_max: i64,
    y_max: i64,

    fn findAntinodes(self: *Board, allocator: std.mem.Allocator, include_resonance: bool) !i64 {
        var signal_iter = self.signals.iterator();
        var antinode_list = std.AutoHashMap(Coordinate, void).init(allocator);
        defer antinode_list.deinit();
        while (signal_iter.next()) |signal_entry| {
            std.debug.print("parsing {c}'s\n", .{signal_entry.key_ptr.*});
            const signal_items = signal_entry.value_ptr.*.items;
            const signal_count = signal_items.len;
            // get pairs of signals
            for (0..(signal_count - 1)) |i| {
                for ((i + 1)..signal_count) |j| {
                    const coord_1 = signal_items[i];
                    const coord_2 = signal_items[j];
                    std.debug.print("checking {d},{d} <-> {d},{d}\n", .{ coord_1.x, coord_1.y, coord_2.x, coord_2.y });
                    const x_diff = coord_1.x - coord_2.x;
                    const y_diff = coord_1.y - coord_2.y;
                    if (include_resonance) {
                        // a.k.a. part 2
                        // always include ourselves,
                        // this is a set so it doesn't matter if we double add
                        try antinode_list.put(coord_1, {});
                        try antinode_list.put(coord_2, {});
                        var last_coord = Coordinate{ .x = coord_2.x - x_diff, .y = coord_2.y - y_diff };
                        while (self.checkInBounds(last_coord)) {
                            try antinode_list.put(last_coord, {});
                            last_coord = Coordinate{ .x = last_coord.x - x_diff, .y = last_coord.y - y_diff };
                        }
                        last_coord = Coordinate{ .x = coord_1.x + x_diff, .y = coord_1.y + y_diff };
                        while (self.checkInBounds(last_coord)) {
                            try antinode_list.put(last_coord, {});
                            last_coord = Coordinate{ .x = last_coord.x + x_diff, .y = last_coord.y + y_diff };
                        }
                    } else {
                        const antinode_1 = Coordinate{ .x = coord_2.x - x_diff, .y = coord_2.y - y_diff };
                        const antinode_2 = Coordinate{ .x = coord_1.x + x_diff, .y = coord_1.y + y_diff };
                        if (self.checkInBounds(antinode_1)) {
                            try antinode_list.put(antinode_1, {});
                        }
                        if (self.checkInBounds(antinode_2)) {
                            try antinode_list.put(antinode_2, {});
                        }
                    }
                }
            }
        }

        var iter = antinode_list.iterator();
        while (iter.next()) |entry| {
            std.debug.print("{any}\n", .{entry.key_ptr.*});
        }
        std.debug.print("{d},{d}\n", .{ self.x_max, self.y_max });
        return antinode_list.count();
    }

    fn checkInBounds(self: *Board, coord: Coordinate) bool {
        return coord.x >= 0 and coord.x <= self.x_max and coord.y >= 0 and coord.y <= self.y_max;
    }

    fn deinit(self: *Board) void {
        var signals_iter = self.signals.iterator();
        while (signals_iter.next()) |signal| {
            signal.value_ptr.*.deinit();
        }
        self.signals.deinit();
    }
};

fn parseBoard(allocator: std.mem.Allocator, input: []const u8) !Board {
    var signals = std.AutoHashMap(u8, CoordinateList).init(allocator);
    var lines = std.mem.splitScalar(u8, input, '\n');
    var row: usize = 0;
    var x_max: usize = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        for (line, 0..) |char, col| {
            x_max = line.len;
            if (char != '.') {
                const res = try signals.getOrPut(char);
                const new_coord = Coordinate{ .x = @as(i64, @intCast(col)), .y = @as(i64, @intCast(row)) };
                if (res.found_existing) {
                    try res.value_ptr.*.append(new_coord);
                } else {
                    var new_list = CoordinateList.init(allocator);
                    try new_list.append(new_coord);
                    res.value_ptr.* = new_list;
                }
            }
        }
        row += 1;
    }

    return Board{
        .signals = signals,
        .x_max = @as(i64, @intCast(x_max - 1)),
        .y_max = @as(i64, @intCast(row - 1)),
    };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var board = try parseBoard(allocator, input);
    defer board.deinit();
    return board.findAntinodes(allocator, false);
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var board = try parseBoard(allocator, input);
    defer board.deinit();
    return board.findAntinodes(allocator, true);
}

// 393 too high
