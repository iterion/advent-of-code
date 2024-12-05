const std = @import("std");
const ArrayList = std.ArrayList;

const example: []const u8 =
    \\47|53
    \\97|13
    \\97|61
    \\97|47
    \\75|29
    \\61|13
    \\75|53
    \\29|13
    \\97|29
    \\53|29
    \\61|53
    \\97|53
    \\61|29
    \\47|13
    \\75|47
    \\97|75
    \\47|61
    \\75|61
    \\47|29
    \\75|13
    \\53|13
    \\
    \\75,47,61,53,29
    \\97,61,53,29,13
    \\75,29,13
    \\75,97,47,61,53
    \\61,13,29
    \\97,13,75,29,47
;
const first_example_answer: i64 = 143;
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

const RulesAndUpdates = struct {
    rules: std.AutoHashMap(i64, ArrayList(i64)),
    reverse_rules: std.AutoHashMap(i64, ArrayList(i64)),
    updates: ArrayList(ArrayList(i64)),

    fn deinit(self: *RulesAndUpdates) void {
        // now deinit hash map contents as they are not needed
        var rules_iter = self.rules.iterator();
        while (rules_iter.next()) |rule| {
            rule.value_ptr.*.deinit();
        }
        var reverse_rules_iter = self.reverse_rules.iterator();
        while (reverse_rules_iter.next()) |rule| {
            rule.value_ptr.*.deinit();
        }
        self.rules.deinit();
        self.reverse_rules.deinit();
        self.updates.deinit();
    }
};

fn parseRulesAndUpdates(allocator: std.mem.Allocator, input: []const u8) !RulesAndUpdates {
    var rules = std.AutoHashMap(i64, ArrayList(i64)).init(
        allocator,
    );
    var reverse_rules = std.AutoHashMap(i64, ArrayList(i64)).init(
        allocator,
    );
    var updates = ArrayList(ArrayList(i64)).init(allocator);
    var parsing_rules: bool = true;
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |line| {
        if (line.len == 0) {
            // We've hit the break between rules and updates
            parsing_rules = false;
            continue;
        }
        if (parsing_rules) {
            var items = std.mem.tokenizeScalar(u8, line, '|');
            const key = try std.fmt.parseInt(i64, items.next().?, 10);
            const value = try std.fmt.parseInt(i64, items.next().?, 10);
            const res = try rules.getOrPut(key);
            if (res.found_existing) {
                try res.value_ptr.*.append(value);
            } else {
                var rule_targets = ArrayList(i64).init(allocator);
                try rule_targets.append(value);
                res.value_ptr.* = rule_targets;
            }
            const reverse_res = try reverse_rules.getOrPut(value);
            if (reverse_res.found_existing) {
                try reverse_res.value_ptr.*.append(key);
            } else {
                var rule_targets = ArrayList(i64).init(allocator);
                try rule_targets.append(key);
                reverse_res.value_ptr.* = rule_targets;
            }
        } else {
            var items = std.mem.tokenizeScalar(u8, line, ',');
            var update = ArrayList(i64).init(allocator);
            while (items.next()) |item| {
                const update_val = try std.fmt.parseInt(i64, item, 10);
                try update.append(update_val);
            }
            try updates.append(update);
        }
    }

    return RulesAndUpdates{
        .rules = rules,
        .reverse_rules = reverse_rules,
        .updates = updates,
    };
}

pub fn solvePartOne(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var total: i64 = 0;
    var rules_and_updates = try parseRulesAndUpdates(allocator, input);

    for (rules_and_updates.updates.items) |update| {
        if (try checkUpdateAgainstRules(allocator, update, rules_and_updates.rules, rules_and_updates.reverse_rules)) {
            const item_to_add = update.items[update.items.len / 2];
            total += item_to_add;
        }
        // deinit these here, and not before!
        update.deinit();
    }

    rules_and_updates.deinit();

    return total;
}

fn checkRule(rules: std.AutoHashMap(i64, ArrayList(i64)), key: i64, value: i64) bool {
    const rule = rules.get(key);

    // if no rules then we're safe, for now
    if (rule) |r| {
        for (r.items) |rule_item| {
            // std.debug.print("checking {d}: {d} == {d}\n", .{ key, value, rule_item });
            if (rule_item == value) {
                return false;
            }
        }
    }
    return true;
}

// fn checkAllKeys() {
// }

fn checkUpdateAgainstRules(_: std.mem.Allocator, update: ArrayList(i64), rules: std.AutoHashMap(i64, ArrayList(i64)), reverse_rules: std.AutoHashMap(i64, ArrayList(i64))) !bool {
    const items_len = update.items.len;
    for (update.items, 0..) |update_item, i| {
        if (i == 0) {
            for (update.items[i + 1 ..]) |item| {
                if (!checkRule(rules, item, update_item)) {
                    return false;
                }
            }
        } else if (i == items_len - 1) {
            for (update.items[0 .. i - 1]) |item| {
                if (!checkRule(reverse_rules, item, update_item)) {
                    return false;
                }
            }
        } else {
            for (update.items[0 .. i - 1]) |item| {
                if (!checkRule(reverse_rules, item, update_item)) {
                    return false;
                }
            }
            for (update.items[i + 1 ..]) |item| {
                if (!checkRule(rules, item, update_item)) {
                    return false;
                }
            }
        }
    }
    return true;
}

pub fn fix_order(rules_and_updates: RulesAndUpdates, update: ArrayList(i64)) void {
    for (update.items, 0..) |*update_item, i| {
        if (i == 0) {
            for (update.items[i + 1 ..]) |*item| {
                if (!checkRule(rules_and_updates.rules, item.*, update_item.*)) {
                    std.mem.swap(i64, item, update_item);
                }
            }
        } else if (i == update.items.len - 1) {
            for (update.items[0 .. i - 1]) |*item| {
                if (!checkRule(rules_and_updates.reverse_rules, item.*, update_item.*)) {
                    std.mem.swap(i64, item, update_item);
                }
            }
        } else {
            for (update.items[0 .. i - 1]) |*item| {
                if (!checkRule(rules_and_updates.reverse_rules, item.*, update_item.*)) {
                    std.mem.swap(i64, item, update_item);
                }
            }
            for (update.items[i + 1 ..]) |*item| {
                if (!checkRule(rules_and_updates.rules, item.*, update_item.*)) {
                    std.mem.swap(i64, item, update_item);
                }
            }
        }
    }
}

pub fn solvePartTwo(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var total: i64 = 0;
    var rules_and_updates = try parseRulesAndUpdates(allocator, input);

    for (rules_and_updates.updates.items) |update| {
        if (!try checkUpdateAgainstRules(allocator, update, rules_and_updates.rules, rules_and_updates.reverse_rules)) {
            // sort the list with the rules and context of the array
            fix_order(rules_and_updates, update);
            if (!try checkUpdateAgainstRules(allocator, update, rules_and_updates.rules, rules_and_updates.reverse_rules)) {
                std.debug.print("failed update!", .{});
            }
            const item_to_add = update.items[update.items.len / 2];
            total += item_to_add;
        }
        // deinit these here, and not before!
        update.deinit();
    }

    rules_and_updates.deinit();

    return total;
}
