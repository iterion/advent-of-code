const std = @import("std");

const example: []const u8 =
    \\190: 10 19
    \\3267: 81 40 27
    \\83: 17 5
    \\156: 15 6
    \\7290: 6 8 6 15
    \\161011: 16 10 13
    \\192: 17 8 14
    \\21037: 9 7 18 13
    \\292: 11 6 16 20
;
const first_example_answer: i64 = 3749;
const second_example_answer: i64 = 6;

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

const Equation = struct {
    answer: i64,
    constants: ConstantsList,

    fn hasPossibleSolution(self: *Equation, allocator: std.mem.Allocator) !bool {
        const constant_count = self.constants.items.len;
        const operand_count = constant_count - 1;
        std.debug.print("{d}: {any} -- {d}\n", .{ self.answer, self.constants.items, operand_count });
        const allowed_operands: [2]Operand = .{ .add, .multiply };
        var possible_operand_lists = std.ArrayList(std.ArrayList(Operand)).init(allocator);
        for (allowed_operands) |operand| {
            var sub_list = std.ArrayList(Operand).init(allocator);
            try sub_list.append(operand);
            const res = self.validateOperands(sub_list);
            if (res == .low) {
                try possible_operand_lists.append(sub_list);
            } else if (operand_count == 1 and res == .correct) {
                return true;
            }
        }

        if (operand_count == 1) {
            return false;
        }

        // For each position, let's try successive values
        var operand_position: usize = 1;
        while (operand_position < operand_count) {
            const last_operand = operand_position == (operand_count - 1);
            var new_items = std.ArrayList(std.ArrayList(Operand)).init(allocator);
            defer new_items.deinit();
            var indexes_to_remove = std.ArrayList(usize).init(allocator);
            defer indexes_to_remove.deinit();
            // We need to check all of our possible paths so far
            for (possible_operand_lists.items, 0..) |*possible_list, j| {
                // Dynamic programming!
                var new_list = try possible_list.clone();
                try new_list.append(.add);
                // std.debug.print("{any} - iter count {d}\n", .{ new_list.items, operand_position });
                var result = self.validateOperands(new_list);
                if (last_operand and result == EquationResult.correct) {
                    for (possible_operand_lists.items) |*l| {
                        l.deinit();
                    }
                    possible_operand_lists.deinit();
                    return true;
                } else if (result == EquationResult.low) {
                    // temp storage to not modify iterated list
                    try new_items.append(new_list);
                } else {
                    // otherwise do nothing since we have exceeded the answer and can't possibly make this work
                    // but do free up memory
                    new_list.deinit();
                }
                try possible_list.append(.multiply);
                // std.debug.print("{any} - iter count {d}\n", .{ possible_list.items, operand_position });
                result = self.validateOperands(possible_list.*);
                if (last_operand and result == EquationResult.correct) {
                    for (possible_operand_lists.items) |*l| {
                        l.deinit();
                    }
                    possible_operand_lists.deinit();
                    return true;
                } else if (result == EquationResult.low) {
                    // try new_items.append(new_list);
                } else {
                    // std.debug.print("should remove! {d}\n", .{j});
                    try indexes_to_remove.append(j);
                }
            }

            var total_removed: usize = 0;
            for (indexes_to_remove.items) |j| {
                // remove offsetting for items removed so far
                // no need to sort as it's already in order
                const possible_list = possible_operand_lists.items[j - total_removed];
                // And, remove allocated mem
                possible_list.deinit();
                // in this case we need to remove this item as it's not worth calculating this branch anymore
                _ = possible_operand_lists.swapRemove(j - total_removed);
                total_removed += 1;
            }

            // Now add new items
            try possible_operand_lists.appendSlice(try new_items.toOwnedSlice());
            operand_position += 1;
        }
        // Dealloc everything used for computation
        for (possible_operand_lists.items) |*possible_list| {
            possible_list.deinit();
        }
        possible_operand_lists.deinit();
        return false;
    }

    fn validateOperands(self: *Equation, operands: std.ArrayList(Operand)) EquationResult {
        var total: i64 = 0;
        for (operands.items, 0..) |op, i| {
            const lhs = if (i == 0) self.constants.items[i] else total;
            if (op == .multiply) {
                // std.debug.print("{d} * {d} - {any}\n", .{ lhs, self.constants.items[i + 1], op });
                total = lhs * self.constants.items[i + 1];
            } else {
                // std.debug.print("{d} + {d} - {any}\n", .{ lhs, self.constants.items[i + 1], op });
                total = lhs + self.constants.items[i + 1];
            }
        }
        // std.debug.print("{d}\n", .{total});
        return if (total > self.answer)
            EquationResult.high
        else if (total < self.answer)
            EquationResult.low
        else
            EquationResult.correct;
    }

    fn deinit(self: *Equation) void {
        self.constants.deinit();
    }
};

const ConstantsList = std.ArrayList(i64);

const EquationList = std.ArrayList(Equation);

fn deinit_equations(equations: *EquationList) void {
    for (equations.items) |*equation| {
        equation.deinit();
    }
    equations.deinit();
}

const EquationResult = enum {
    low,
    high,
    correct,
};

const Operand = enum {
    add,
    multiply,
};

fn parseAllEquations(allocator: std.mem.Allocator, input: []const u8) !EquationList {
    var equations = EquationList.init(allocator);
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        try equations.append(try parseEquation(allocator, line));
    }

    return equations;
}

fn parseEquation(allocator: std.mem.Allocator, input: []const u8) !Equation {
    var answer_and_constants = std.mem.splitSequence(u8, input, ": ");
    const answer = try std.fmt.parseInt(i64, answer_and_constants.next().?, 10);
    const constants_str = answer_and_constants.next().?;
    var constants_str_iter = std.mem.tokenizeScalar(u8, constants_str, ' ');
    var constants = ConstantsList.init(allocator);
    while (constants_str_iter.next()) |constant_str| {
        const constant = try std.fmt.parseInt(i64, constant_str, 10);
        try constants.append(constant);
    }
    return Equation{
        .answer = answer,
        .constants = constants,
    };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var equations = try parseAllEquations(allocator, input);
    defer deinit_equations(&equations);

    var count: i64 = 0;
    for (equations.items) |*equation| {
        if (try equation.hasPossibleSolution(allocator)) {
            count += equation.answer;
        } else {
            std.debug.print("NOT GUD\n", .{});
        }
    }

    return count;
}

pub fn solvePartTwo(_: std.mem.Allocator, _: []const u8) !i64 {
    const count: usize = 0;

    return count;
}
