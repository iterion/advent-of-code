const std = @import("std");

const example: []const u8 = "2333133121414131402";
const first_example_answer: i64 = 1928;
const second_example_answer: i64 = 2858;

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

const Block = union(enum) { id: u16, free: bool };

const FileMeta = struct {
    id: u16,
    size: u8,
    initial_location: u32,
};

const Filesystem = struct {
    layout: std.ArrayList(Block),
    file_meta: std.ArrayList(FileMeta),

    fn compact(self: *Filesystem) !void {
        const size = self.layout.items.len;
        var location = size - 1;
        var first_free: usize = 0;

        while (location > 0) : (location -= 1) {
            first_free = try self.findFirstFree(1);
            if (location < first_free) {
                // We're now putting things on the end which we don't want
                break;
            }
            const value = &self.layout.items[location];
            switch (value.*) {
                .id => {
                    std.mem.swap(Block, value, &self.layout.items[first_free]);
                },
                .free => continue,
            }
        }
    }

    fn compactFiles(self: *Filesystem) !void {
        const size = self.file_meta.items.len;
        var location = size - 1;
        var first_free: usize = 0;

        while (location > 0) : (location -= 1) {
            const file = &self.file_meta.items[location];
            first_free = self.findFirstFree(file.size) catch continue;
            if (file.initial_location < first_free) {
                // We're now putting things on the end which we don't want
                continue;
            }

            for (0..file.size) |i| {
                const index_to_move = file.initial_location + i;
                const free_index = first_free + i;
                std.mem.swap(Block, &self.layout.items[index_to_move], &self.layout.items[free_index]);
            }
        }
    }

    fn checksum(self: *Filesystem) i64 {
        var total: i64 = 0;
        for (self.layout.items, 0..) |block, i| {
            switch (block) {
                .id => {
                    total += @as(i64, @intCast(i * block.id));
                },
                .free => continue,
            }
        }

        return total;
    }

    fn findFirstFree(self: *Filesystem, size: usize) !usize {
        var free_index: usize = 0;
        var free_count: usize = 0;
        for (self.layout.items, 0..) |block, i| {
            switch (block) {
                .id => free_count = 0,
                .free => {
                    if (free_count == 0) {
                        free_index = i;
                    }
                    free_count += 1;
                },
            }
            if (free_count == size) {
                return free_index;
            }
        }
        return error.NoFreeSpace;
    }

    fn print(self: *Filesystem) void {
        for (self.layout.items) |block| {
            switch (block) {
                .id => std.debug.print("{}", .{block.id}),
                .free => std.debug.print(".", .{}),
            }
        }
        std.debug.print("\n", .{});
    }

    fn deinit(self: *Filesystem) void {
        self.layout.deinit();
    }
};

fn parseFilesystem(allocator: std.mem.Allocator, input: []const u8) !Filesystem {
    var layout = std.ArrayList(Block).init(allocator);
    var file_metas = std.ArrayList(FileMeta).init(allocator);
    var is_file = true;
    var id: usize = 0;
    var cur_index: usize = 0;
    for (input) |char| {
        if (char == ' ' or char == '\n') {
            continue;
        }
        const count = try std.fmt.parseInt(u8, &[_]u8{char}, 10);
        if (is_file) {
            try file_metas.append(FileMeta{ .id = @as(u16, @intCast(id)), .size = count, .initial_location = @as(u32, @intCast(cur_index)) });
        }
        for (0..count) |_| {
            cur_index += 1;
            if (is_file) {
                try layout.append(Block{ .id = @as(u16, @intCast(id)) });
            } else {
                try layout.append(Block{ .free = true });
            }
        }

        if (is_file) {
            id += 1;
        }

        is_file = !is_file;
    }

    return Filesystem{
        .layout = layout,
        .file_meta = file_metas,
    };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var fs = try parseFilesystem(allocator, input);
    defer fs.deinit();
    try fs.compact();

    return fs.checksum();
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var fs = try parseFilesystem(allocator, input);
    defer fs.deinit();
    try fs.compactFiles();

    return fs.checksum();
}
