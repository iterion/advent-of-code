const std = @import("std");
const ArrayList = std.ArrayList;

const example: []const u8 =
    \\7 6 4 2 1
    \\1 2 7 8 9
    \\9 7 6 2 1
    \\1 3 2 4 5
    \\8 6 4 4 1
    \\1 3 6 7 9
;
const first_example_answer: i64 = 2;
const second_example_answer: i64 = 4;

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

const Direction = enum { up, down };

fn toDirection(diff: i64) Direction {
    if (diff < 0) {
        return Direction.down;
    } else {
        return Direction.up;
    }
}

fn parseLineToArrayList(allocator: std.mem.Allocator, line: []const u8) !std.ArrayList(i64) {
    var levels = std.ArrayList(i64).init(allocator);
    var level_strs = std.mem.tokenizeScalar(u8, line, ' ');
    while (level_strs.next()) |level_str| {
        try levels.append(try std.fmt.parseInt(i64, level_str, 10));
    }
    return levels;
}

const LevelVerify = enum { success, failed };
const LevelResult = union(LevelVerify) {
    success: bool,
    failed: usize,
};

fn verifyLevels(levels: []i64) LevelResult {
    // pop first level off so we can start comparing below
    const first_level = levels[0];
    const second_level = levels[1];
    const initial_diff = first_level - second_level;
    if (@abs(initial_diff) > 3 or initial_diff == 0) {
        return LevelResult{ .failed = 1 };
    }
    var last_direction = toDirection(initial_diff);
    // put it in last_level for comparison in loop
    var last_level = second_level;
    // first time in loop
    for (levels[2..], 2..) |level, i| {
        const diff = last_level - level;
        const new_direction = toDirection(diff);
        if (@abs(diff) > 3 or diff == 0 or last_direction != new_direction) {
            return LevelResult{ .failed = i };
        }
        last_level = level;
        last_direction = new_direction;
    }
    return LevelResult{ .success = true };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var safe_rows: i64 = 0;
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) {
            // ignore empty
            continue;
        }
        const levels = try parseLineToArrayList(allocator, line);
        defer levels.deinit();
        switch (verifyLevels(levels.items)) {
            .success => safe_rows += 1,
            .failed => {},
        }
    }
    return safe_rows;
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var safe_rows: i64 = 0;
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) {
            // ignore empty
            continue;
        }
        var levels = try parseLineToArrayList(allocator, line);
        defer levels.deinit();
        switch (verifyLevels(levels.items)) {
            .success => safe_rows += 1,
            .failed => {
                var i: usize = 0;
                while (i < levels.items.len) : (i += 1) {
                    var origLevels = try levels.clone();
                    defer origLevels.deinit();
                    _ = origLevels.orderedRemove(i);
                    switch (verifyLevels(origLevels.items)) {
                        .success => {
                            safe_rows += 1;
                            break;
                        },
                        .failed => {},
                    }
                }
            },
        }
    }
    return safe_rows;
}
