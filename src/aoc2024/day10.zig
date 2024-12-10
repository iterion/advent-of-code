const std = @import("std");

const example: []const u8 =
    \\89010123
    \\78121874
    \\87430965
    \\96549874
    \\45678903
    \\32019012
    \\01329801
    \\10456732
;
const first_example_answer: i64 = 36;
const second_example_answer: i64 = 81;

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
    x: i16,
    y: i16,
};

const CoordinateHeight = struct {
    x: i16,
    y: i16,
    z: u8,

    pub fn format(value: @This(), comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
        return writer.print("<{d},{d},{d}>", .{ value.x, value.y, value.z });
    }
};

const Map = struct {
    height: i16,
    width: i16,
    location_heights: std.ArrayList(u8),
    zero_locations: std.ArrayList(Coordinate),

    fn getCoordHeight(self: *Map, coord: Coordinate) CoordinateHeight {
        const i = coord.x + coord.y * self.width;
        // std.debug.print("({d},{d}) - {d}x{d}\n", .{ coord.x, coord.y, self.width, self.height });
        const height = self.location_heights.items[@as(usize, @intCast(i))];
        return CoordinateHeight{ .x = coord.x, .y = coord.y, .z = height };
    }

    fn isValidCoord(self: *Map, coord: Coordinate) bool {
        return coord.x < self.width and coord.y < self.height and coord.x >= 0 and coord.y >= 0;
    }

    fn getSurrounding(self: *Map, coord: CoordinateHeight) [4]?CoordinateHeight {
        //                   top, right, bottom, left
        const x_add: [4]i16 = .{ 0, 1, 0, -1 };
        const y_add: [4]i16 = .{ -1, 0, 1, 0 };
        var coords: [4]?CoordinateHeight = .{null} ** 4;
        for (x_add, 0..) |x_diff, i| {
            const new_coord = Coordinate{ .x = coord.x + x_diff, .y = coord.y + y_add[i] };
            if (self.isValidCoord(new_coord)) {
                coords[i] = self.getCoordHeight(new_coord);
            }
        }
        return coords;
    }

    fn sumPossiblePaths(self: *Map, allocator: std.mem.Allocator, return_endpoints: bool) !i64 {
        var current_paths = std.ArrayList([10]?CoordinateHeight).init(allocator);
        defer current_paths.deinit();
        var next_paths = std.ArrayList([10]?CoordinateHeight).init(allocator);
        defer next_paths.deinit();
        var endpoints = std.AutoHashMap(CoordinateHeight, void).init(allocator);
        defer endpoints.deinit();
        var total_paths: i64 = 0;
        for (self.zero_locations.items) |coord| {
            var first_path: [10]?CoordinateHeight = .{null} ** 10;
            first_path[0] = CoordinateHeight{ .x = coord.x, .y = coord.y, .z = 0 };
            try current_paths.append(first_path);
            for (1..10) |next_height| {
                for (current_paths.items) |path| {
                    const last_coord = path[next_height - 1];
                    // std.debug.print("{any} - {d} - {any}\n", .{ path, next_height, last_coord });
                    const surrounding = self.getSurrounding(last_coord.?);
                    // std.debug.print("{any}\n", .{surrounding});
                    for (surrounding) |loc| {
                        if (loc != null and loc.?.z == next_height) {
                            // deep copy this path
                            var new_path: [10]?CoordinateHeight = .{null} ** 10;
                            for (path, 0..) |ch, i| {
                                new_path[i] = ch;
                            }
                            // add new loc to path
                            new_path[next_height] = loc;
                            // add to next_paths
                            try next_paths.append(new_path);
                        }
                    }
                }
                // optimization: if at any point all paths point to the same location we should only keep one list
                // as from that point all routes that lead to this route don't matter as we only care about end points
                current_paths.clearRetainingCapacity();
                const temp = current_paths;
                current_paths = next_paths;
                next_paths = temp;
            }

            if (return_endpoints) {
                while (current_paths.items.len > 0) {
                    const path = current_paths.pop();
                    // only need the last point
                    try endpoints.put(path[9].?, {});
                }
                total_paths += endpoints.count();
                endpoints.clearRetainingCapacity();
            } else {
                total_paths += @as(i64, @intCast(current_paths.items.len));
                current_paths.clearRetainingCapacity();
            }
            next_paths.clearRetainingCapacity();
        }
        return total_paths;
    }

    fn deinit(self: *Map) void {
        self.location_heights.deinit();
        self.zero_locations.deinit();
    }
};

fn parseMap(allocator: std.mem.Allocator, input: []const u8) !Map {
    var location_heights = std.ArrayList(u8).init(allocator);
    var zero_locations = std.ArrayList(Coordinate).init(allocator);
    var lines = std.mem.splitScalar(u8, input, '\n');
    var row: i16 = 0;
    var cols: i16 = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        cols = @as(i16, @intCast(line.len));
        for (line, 0..) |c, col| {
            const height = try std.fmt.parseInt(u8, &[_]u8{c}, 10);
            if (height == 0) {
                try zero_locations.append(Coordinate{ .x = @as(i16, @intCast(col)), .y = row });
            }

            try location_heights.append(height);
        }
        row += 1;
    }

    return Map{
        .location_heights = location_heights,
        .zero_locations = zero_locations,
        .height = row,
        .width = cols,
    };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var map = try parseMap(allocator, input);
    defer map.deinit();

    return map.sumPossiblePaths(allocator, true);
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var map = try parseMap(allocator, input);
    defer map.deinit();

    return map.sumPossiblePaths(allocator, false);
}
