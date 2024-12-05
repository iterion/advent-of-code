const std = @import("std");
const example: []const u8 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const second_example: []const u8 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
const first_example_answer: i64 = 161;
const second_example_answer: i64 = 48;

test "part one" {
    const allocator = std.testing.allocator;
    const answer = try solvePartOne(allocator, example);
    try std.testing.expectEqual(answer, first_example_answer);
}

test "part two" {
    const allocator = std.testing.allocator;
    const answer = try solvePartTwo(allocator, second_example);
    try std.testing.expectEqual(answer, second_example_answer);
}

pub fn solvePartOne(_: std.mem.Allocator, input: []const u8) !i64 {
    var total: i64 = 0;
    var i: usize = 0;
    outer: while (i < input.len) {
        if (input[i] == 'm') {
            const next4 = input[i .. i + 4];
            if (std.mem.eql(u8, next4, "mul(")) {
                const mul_start = i + 4;
                var new_i = mul_start;
                var comma_index: usize = 0;
                var lhs: i64 = 0;
                while (new_i < input.len) {
                    const val = input[new_i];
                    if (val == ',') {
                        if (new_i == mul_start or comma_index != 0) {
                            i += 1;
                            continue :outer;
                        }
                        comma_index = new_i;
                        const lhs_str = input[mul_start..comma_index];
                        lhs = try std.fmt.parseInt(i64, lhs_str, 10);
                        new_i += 1;
                    } else if (val == ')') {
                        if (comma_index != 0) {
                            const rhs_str = input[comma_index + 1 .. new_i];
                            const rhs = try std.fmt.parseInt(i64, rhs_str, 10);
                            total += lhs * rhs;
                            i = new_i;
                            continue :outer;
                        } else {
                            i += 1;
                            continue :outer;
                        }
                    } else if (std.ascii.isDigit(val)) {
                        new_i += 1;
                    } else {
                        i += 1;
                        continue :outer;
                    }
                }
            }
        }
        i += 1;
    }
    return total;
}

pub fn solvePartTwo(_: std.mem.Allocator, input: []const u8) !i64 {
    var total: i64 = 0;
    var i: usize = 0;
    var enabled: bool = true;
    outer: while (i < input.len) {
        if (input[i] == 'm') {
            const next4 = input[i .. i + 4];
            if (enabled and std.mem.eql(u8, next4, "mul(")) {
                const mul_start = i + 4;
                var new_i = mul_start;
                var comma_index: usize = 0;
                var lhs: i64 = 0;
                while (new_i < input.len) {
                    const val = input[new_i];
                    if (val == ',') {
                        if (new_i == mul_start or comma_index != 0) {
                            i += 1;
                            continue :outer;
                        }
                        comma_index = new_i;
                        const lhs_str = input[mul_start..comma_index];
                        lhs = try std.fmt.parseInt(i64, lhs_str, 10);
                        new_i += 1;
                    } else if (val == ')') {
                        if (comma_index != 0) {
                            const rhs_str = input[comma_index + 1 .. new_i];
                            const rhs = try std.fmt.parseInt(i64, rhs_str, 10);
                            total += lhs * rhs;
                            i = new_i;
                            continue :outer;
                        } else {
                            i += 1;
                            continue :outer;
                        }
                    } else if (std.ascii.isDigit(val)) {
                        new_i += 1;
                    } else {
                        i += 1;
                        continue :outer;
                    }
                }
            }
            i += 1;
        } else if (input[i] == 'd') {
            if (std.mem.eql(u8, input[i .. i + 4], "do()")) {
                enabled = true;
                i += 4;
            } else if (std.mem.eql(u8, input[i .. i + 7], "don't()")) {
                enabled = false;
                i += 7;
            }
        } else {
            i += 1;
        }
    }
    return total;
}
