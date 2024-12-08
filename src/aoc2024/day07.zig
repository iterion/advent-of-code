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

    fn hasPossibleSolution(self: *Equation, allocator: std.mem.Allocator, comptime use_concat: bool) !bool {
        const constant_count = self.constants.items.len;
        const operand_count = constant_count - 1;
        // std.debug.print("{d}: {any} -- {d}\n", .{ self.answer, self.constants.items, operand_count });
        const allowed_operands = if (use_concat) [3]Operand{ .add, .multiply, .concat } else [2]Operand{ .add, .multiply };
        var current_level = std.ArrayList(std.ArrayList(Operand)).init(allocator);
        var next_level = std.ArrayList(std.ArrayList(Operand)).init(allocator);

        // Initialize current_level based on the first allowed operators
        for (allowed_operands) |op| {
            var seq = std.ArrayList(Operand).init(allocator);
            try seq.append(op);
            const res = try self.validateOperands(allocator, seq);
            if (operand_count == 1 and res == .correct) {
                seq.deinit();
                current_level.deinit();
                next_level.deinit();
                return true;
            } else if (res == .low or res == .correct) {
                try current_level.append(seq);
            } else {
                seq.deinit();
            }
        }

        if (operand_count == 1) {
            // No correct solution found
            for (current_level.items) |*seq| seq.deinit();
            current_level.deinit();
            next_level.deinit();
            return false;
        }
        var operand_pos: usize = 1;
        while (operand_pos < operand_count) {
            const last_operand = (operand_pos == operand_count - 1);

            // Clear next_level for the upcoming expansions
            while (next_level.items.len > 0) {
                const item = next_level.items[next_level.items.len - 1];
                _ = next_level.pop();
                item.deinit();
            }

            // Expand all current sequences
            while (current_level.items.len > 0) {
                const seq = current_level.items[current_level.items.len - 1];
                _ = current_level.pop();

                // For each allowed operator, clone seq, append operator, and check
                for (allowed_operands) |op| {
                    var new_seq = std.ArrayList(Operand).init(allocator);
                    // copy current seq
                    for (seq.items) |existing_op| {
                        try new_seq.append(existing_op);
                    }
                    try new_seq.append(op);

                    const res = try self.validateOperands(allocator, new_seq);
                    if (last_operand and res == .correct) {
                        // Found a correct solution
                        new_seq.deinit();
                        seq.deinit();
                        // Cleanup and return true
                        for (current_level.items) |*item| item.deinit();
                        for (next_level.items) |*item| item.deinit();
                        current_level.deinit();
                        next_level.deinit();
                        return true;
                    } else if (res == .low or res == .correct) {
                        try next_level.append(new_seq);
                    }
                }

                seq.deinit();
            }

            // Now move all next_level items into current_level for the next iteration
            // Instead of popping each item individually, just swap references:
            const temp = current_level;
            current_level = next_level;
            next_level = temp;
            operand_pos += 1;
        }

        // After all expansions, if no correct solution found:
        for (current_level.items) |*seq| {
            seq.deinit();
        }
        current_level.deinit();
        next_level.deinit();
        return false;
    }

    fn validateOperands(self: *Equation, allocator: std.mem.Allocator, operands: std.ArrayList(Operand)) !EquationResult {
        var total: i64 = 0;
        for (operands.items, 0..) |op, i| {
            const lhs = if (i == 0) self.constants.items[i] else total;
            const rhs = self.constants.items[i + 1];
            if (op == .multiply) {
                // std.debug.print("{d} * {d} - {any}\n", .{ lhs, self.constants.items[i + 1], op });
                total = lhs * rhs;
            } else if (op == .add) {
                // std.debug.print("{d} + {d} - {any}\n", .{ lhs, self.constants.items[i + 1], op });
                total = lhs + rhs;
            } else {
                const concat_str = try std.fmt.allocPrint(
                    allocator,
                    "{d}{d}",
                    .{ lhs, rhs },
                );
                total = try std.fmt.parseInt(i64, concat_str, 10);
                allocator.free(concat_str);
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
    concat,
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
        if (try equation.hasPossibleSolution(allocator, false)) {
            count += equation.answer;
        } else {
            // std.debug.print("NOT GUD\n", .{});
        }
    }

    return count;
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var equations = try parseAllEquations(allocator, input);
    defer deinit_equations(&equations);

    var count: i64 = 0;
    for (equations.items) |*equation| {
        if (try equation.hasPossibleSolution(allocator, true)) {
            count += equation.answer;
        } else {
            // std.debug.print("NOT GUD\n", .{});
        }
    }

    return count;
}
