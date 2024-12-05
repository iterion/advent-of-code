const std = @import("std");
const clap = @import("clap");
const zbench = @import("zbench");
const aoc2024 = @import("aoc2024/root.zig");

const SubCommands = enum {
    run,
    bench,
};

const main_parsers = .{
    .command = clap.parsers.enumeration(SubCommands),
};

// The parameters for `main`. Parameters for the subcommands are specified further down.
const main_params = clap.parseParamsComptime(
    \\-h, --help  Display this help and exit.
    \\<command>
    \\
);

const BenchDay = struct {
    content: []const u8,
    day: usize,
    part: usize,

    fn init(content: []const u8, day: usize, part: usize) BenchDay {
        return .{ .content = content, .day = day, .part = part };
    }

    pub fn run(self: BenchDay, allocator: std.mem.Allocator) void {
        _ = runDayAndPart(allocator, self.content, self.day, self.part) catch @panic("oops");
    }
};

// To pass around arguments returned by clap, `clap.Result` and `clap.ResultEx` can be used to
// get the return type of `clap.parse` and `clap.parseEx`.
const MainArgs = clap.ResultEx(clap.Help, &main_params, main_parsers);

pub fn main() !void {
    var gpa_state = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = gpa_state.allocator();
    defer _ = gpa_state.deinit();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var iter = try std.process.ArgIterator.initWithAllocator(gpa);
    defer iter.deinit();

    _ = iter.next();

    var diag = clap.Diagnostic{};
    var res = clap.parseEx(clap.Help, &main_params, main_parsers, &iter, .{
        .diagnostic = &diag,
        .allocator = gpa,

        // Terminate the parsing of arguments after parsing the first positional (0 is passed
        // here because parsed positionals are, like slices and arrays, indexed starting at 0).
        //
        // This will terminate the parsing after parsing the subcommand enum and leave `iter`
        // not fully consumed. It can then be reused to parse the arguments for subcommands.
        .terminating_positional = 0,
    }) catch |err| {
        diag.report(std.io.getStdErr().writer(), err) catch {};
        return err;
    };
    defer res.deinit();

    if (res.args.help != 0)
        std.debug.print("--help\n", .{});

    // The parameters for the subcommand.
    const params = comptime clap.parseParamsComptime(
        \\-h, --help  Display this help and exit.
        \\<usize>
        \\
    );

    var res2 = clap.parseEx(clap.Help, &params, clap.parsers.default, &iter, .{
        .diagnostic = &diag,
        .allocator = gpa,
    }) catch |err| {
        diag.report(std.io.getStdErr().writer(), err) catch {};
        return err;
    };
    defer res2.deinit();

    const day = res2.positionals[0] orelse return error.MissingArg1;
    const contents = try getFileForDay(gpa, day);
    defer gpa.free(contents);
    const command = res.positionals[0] orelse return error.MissingCommand;
    switch (command) {
        .run => {
            var timer = try std.time.Timer.start();
            const answer_1 = try runDayAndPart(gpa, contents, day, 1);
            const t1 = timer.lap();
            const answer_2 = try runDayAndPart(gpa, contents, day, 2);
            const t2 = timer.lap();
            try stdout.print("day {d} - part 1: {d} - {d}μs\n", .{ day, answer_1, t1 / 1000 });
            try stdout.print("day {d} - part 2: {d} - {d}μs\n", .{ day, answer_2, t2 / 1000 });
        },
        .bench => {
            var bench = zbench.Benchmark.init(std.heap.page_allocator, .{});
            defer bench.deinit();

            try bench.addParam("part 1", &BenchDay.init(contents, day, 1), .{});
            try bench.addParam("part 2", &BenchDay.init(contents, day, 2), .{});

            try stdout.writeAll("\n");
            try bench.run(stdout);
        },
    }

    try bw.flush(); // Don't forget to flush!
}

fn getFileForDay(allocator: std.mem.Allocator, day: usize) ![]const u8 {
    const file_name = try std.fmt.allocPrint(
        allocator,
        "day{d:02}.txt",
        .{day},
    );
    const path = try std.fs.path.join(allocator, &[_][]const u8{ "inputs", "2024", file_name });
    const file = try std.fs.cwd().openFile(path, .{});
    allocator.free(file_name);
    allocator.free(path);
    defer file.close();
    const stat = try file.stat();

    const contents = try file.reader().readAllAlloc(
        allocator,
        stat.size,
    );
    return contents;
}

pub fn runDayAndPart(allocator: std.mem.Allocator, contents: []const u8, day: usize, part: usize) !i64 {
    const PartFunction = fn (allocator: std.mem.Allocator, contents: []const u8) anyerror!i64;

    var solvePartOne: ?*const PartFunction = null;
    var solvePartTwo: ?*const PartFunction = null;

    switch (day) {
        1 => {
            solvePartOne = aoc2024.day01.solvePartOne;
            solvePartTwo = aoc2024.day01.solvePartTwo;
        },
        2 => {
            solvePartOne = aoc2024.day02.solvePartOne;
            solvePartTwo = aoc2024.day02.solvePartTwo;
        },
        3 => {
            solvePartOne = aoc2024.day03.solvePartOne;
            solvePartTwo = aoc2024.day03.solvePartTwo;
        },
        4 => {
            solvePartOne = aoc2024.day04.solvePartOne;
            solvePartTwo = aoc2024.day04.solvePartTwo;
        },
        5 => {
            solvePartOne = aoc2024.day05.solvePartOne;
            solvePartTwo = aoc2024.day05.solvePartTwo;
        },
        else => return error.InvalidDay,
    }

    if (solvePartOne == null or solvePartTwo == null) {
        return error.InvalidDay;
    }

    return switch (part) {
        1 => solvePartOne.?(allocator, contents),
        2 => solvePartTwo.?(allocator, contents),
        else => return error.InvalidPart,
    };
}
