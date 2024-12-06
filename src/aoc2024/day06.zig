const std = @import("std");

const example: []const u8 =
    \\....#.....
    \\.........#
    \\..........
    \\..#.......
    \\.......#..
    \\..........
    \\.#..^.....
    \\........#.
    \\#.........
    \\......#...
;
const first_example_answer: i64 = 41;
const second_example_answer: i64 = 123;

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

const Coordinate = struct {
    x: usize,
    y: usize,
};

const Orientation = enum {
    up,
    right,
    down,
    left,
};

const ObstacleCoords = std.AutoHashMap(Coordinate, void);

const Map = struct {
    obstacle_coordinates: ObstacleCoords,
    guard_location: Coordinate,
    guard_orientation: Orientation,
    length: usize,
    width: usize,

    fn deinit(self: *Map) void {
        self.obstacle_coordinates.deinit();
    }

    fn moveGuard(self: *Map) bool {
        const new_coord = self.getNextGuardCoordinate() catch {
            // We're off the map, so we're done moving
            return false;
        };
        // hit an obstacle, so reorient
        if (self.obstacle_coordinates.contains(new_coord)) {
            // reorient but don't move
            self.guard_orientation = switch (self.guard_orientation) {
                .up => Orientation.right,
                .right => Orientation.down,
                .down => Orientation.left,
                .left => Orientation.up,
            };
        } else {
            self.guard_location = new_coord;
        }

        return true;
    }

    fn getNextGuardCoordinate(self: *Map) !Coordinate {
        var coord: Coordinate = undefined;
        const loc = self.guard_location;
        switch (self.guard_orientation) {
            .up => {
                if (loc.y == 0) {
                    return error.OffMap;
                }
                coord = Coordinate{ .x = loc.x, .y = loc.y - 1 };
            },
            .right => {
                if (loc.x == (self.width - 1)) {
                    return error.OffMap;
                }
                coord = Coordinate{ .x = loc.x + 1, .y = loc.y };
            },
            .down => {
                if (loc.y == (self.length - 1)) {
                    return error.OffMap;
                }
                coord = Coordinate{ .x = loc.x, .y = loc.y + 1 };
            },
            .left => {
                if (loc.x == 0) {
                    return error.OffMap;
                }
                coord = Coordinate{ .x = loc.x - 1, .y = loc.y };
            },
        }

        return coord;
    }
};

fn parseMap(allocator: std.mem.Allocator, input: []const u8) !Map {
    var obstacles = std.AutoHashMap(Coordinate, void).init(allocator);
    var guard_location: Coordinate = undefined;
    var guard_orientation: Orientation = undefined;
    var lines = std.mem.splitScalar(u8, input, '\n');
    var row: usize = 0;
    var width: usize = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        width = line.len;
        for (line, 0..) |char, col| {
            if (char == '#') {
                try obstacles.put(Coordinate{ .x = col, .y = row }, {});
            }
            if (char == '^') {
                guard_orientation = Orientation.up;
                guard_location = Coordinate{ .x = col, .y = row };
            }
        }
        row += 1;
    }

    return Map{
        .obstacle_coordinates = obstacles,
        .guard_location = guard_location,
        .guard_orientation = guard_orientation,
        .length = row,
        .width = width,
    };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var visited = std.AutoHashMap(Coordinate, void).init(allocator);
    defer visited.deinit();
    var map = try parseMap(allocator, input);
    defer map.deinit();

    while (map.moveGuard()) {
        try visited.put(map.guard_location, {});
    }

    return visited.count();
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var visited = std.AutoHashMap(Coordinate, void).init(allocator);
    defer visited.deinit();
    var map = try parseMap(allocator, input);
    defer map.deinit();

    while (map.moveGuard()) {
        try visited.put(map.guard_location, {});
    }

    return visited.count();
}
