const std = @import("std");

const target: []const u8 = "XMAS";
const dummy_char: u8 = 'n';
const dummy: [4]u8 = .{ dummy_char, dummy_char, dummy_char, dummy_char };
const example: []const u8 =
    \\MMMSXXMASM
    \\MSAMXMSMSA
    \\AMXSXMAAMM
    \\MSAMASMSMX
    \\XMASAMXAMM
    \\XXAMMXXAMA
    \\SMSMSASXSS
    \\SAXAMASAAA
    \\MAMMMXMMMM
    \\MXMXAXMASX
;
const first_example_answer: i64 = 18;
const second_example_answer: i64 = 9;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();
const ArrayList = std.ArrayList;

test "part one" {
    const answer = try solvePartOne(example);
    try std.testing.expectEqual(answer, first_example_answer);
}

test "part two" {
    const answer = try solvePartTwo(example);
    try std.testing.expectEqual(answer, second_example_answer);
}

pub fn solvePartOne(input: []const u8) !i64 {
    var total: i64 = 0;
    var i: usize = 0;
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const size = std.mem.replacementSize(u8, input, "\n", "");
    const input_clean = try allocator.alloc(u8, size);
    _ = std.mem.replace(u8, input, "\n", "", input_clean);
    while (i < input_clean.len) {
        if (input_clean[i] == 'X') {
            // Now we check around the X in all orientations
            const down: [4]u8 = if (i + width * 3 < input_clean.len)
                .{ input_clean[i], input_clean[i + width], input_clean[i + width * 2], input_clean[i + width * 3] }
            else
                dummy;

            const up: [4]u8 = if (i >= width * 3)
                .{ input_clean[i], input_clean[i - width], input_clean[i - width * 2], input_clean[i - width * 3] }
            else
                dummy;

            const right: [4]u8 = if ((i % width) + 3 >= width or i + 3 >= input_clean.len)
                dummy
            else
                .{ input_clean[i], input_clean[i + 1], input_clean[i + 2], input_clean[i + 3] };

            const down_right: [4]u8 = if ((i % width) + 3 >= width or i + width * 3 + 3 >= input_clean.len)
                dummy
            else
                .{ input_clean[i], input_clean[i + (width + 1)], input_clean[i + (width * 2 + 2)], input_clean[i + (width * 3 + 3)] };

            const up_right: [4]u8 = if ((i % width) + 3 >= width or i < width * 3 + 3)
                dummy
            else
                .{ input_clean[i], input_clean[i - (width - 1)], input_clean[i - (width * 2 - 2)], input_clean[i - (width * 3 - 3)] };

            const left: [4]u8 = if (i < 3 or i % width < 3)
                dummy
            else
                .{ input_clean[i], input_clean[i - 1], input_clean[i - 2], input_clean[i - 3] };

            const down_left: [4]u8 = if (i < 3 or i + (width * 3 - 3) >= input_clean.len or i % width < 3)
                dummy
            else
                .{ input_clean[i], input_clean[i + (width - 1)], input_clean[i + (width * 2 - 2)], input_clean[i + (width * 3 - 3)] };

            const up_left: [4]u8 = if (i < width * 3 + 3 or i % width < 3)
                dummy
            else
                .{ input_clean[i], input_clean[i - (width + 1)], input_clean[i - (width * 2 + 2)], input_clean[i - (width * 3 + 3)] };

            // std.debug.print("{s}-{s}-{s}-{s}-{s}-{s}-{s}-{s} at [{d},{d}]\n", .{ left, right, up, down, down_left, down_right, up_left, up_right, i % width, i / width });
            if (std.mem.eql(u8, &right, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &left, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &up, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &down, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &down_right, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &down_left, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &up_left, target)) {
                total += 1;
            }
            if (std.mem.eql(u8, &up_right, target)) {
                total += 1;
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    return total;
}

pub fn solvePartTwo(input: []const u8) !i64 {
    var total: i64 = 0;
    var i: usize = 0;
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const size = std.mem.replacementSize(u8, input, "\n", "");
    const input_clean = try allocator.alloc(u8, size);
    _ = std.mem.replace(u8, input, "\n", "", input_clean);
    while (i < input_clean.len) {
        if (input_clean[i] == 'A') {
            // Now we find the chars on the X
            const down_right: u8 = if ((i % width) + 1 >= width or i + width + 1 >= input_clean.len)
                dummy_char
            else
                input_clean[i + (width + 1)];

            const up_right: u8 = if ((i % width) + 1 >= width or i < width + 1)
                dummy_char
            else
                input_clean[i - (width - 1)];

            const down_left: u8 = if (i < 1 or i + (width - 1) >= input_clean.len or i % width < 1)
                dummy_char
            else
                input_clean[i + (width - 1)];

            const up_left: u8 = if (i < width + 1 or i % width < 1)
                dummy_char
            else
                input_clean[i - (width + 1)];

            const x: [4]u8 = .{ up_left, down_left, down_right, up_right };
            // std.debug.print("{s} at [{d},{d}]\n", .{ x, i % width, i / width });
            if (std.mem.eql(u8, &x, "MMSS")) {
                total += 1;
            }
            if (std.mem.eql(u8, &x, "SSMM")) {
                total += 1;
            }
            if (std.mem.eql(u8, &x, "MSSM")) {
                total += 1;
            }
            if (std.mem.eql(u8, &x, "SMMS")) {
                total += 1;
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    return total;
}
