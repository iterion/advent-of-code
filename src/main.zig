const std = @import("std");
const clap = @import("clap");
const aoc2024 = @import("aoc2024/root.zig");

const SubCommands = enum {
    run,
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

    const command = res.positionals[0] orelse return error.MissingCommand;
    switch (command) {
        .run => {
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
            switch (day) {
                1 => {
                    try stdout.print("day 1 - part 1: {d}\n", .{try aoc2024.day01.solvePartOne(contents)});
                    try stdout.print("day 1 - part 2: {d}\n", .{try aoc2024.day01.solvePartTwo(contents)});
                },
                2 => {
                    try stdout.print("day 2 - part 1: {d}\n", .{try aoc2024.day02.solvePartOne(contents)});
                    try stdout.print("day 2 - part 2: {d}\n", .{try aoc2024.day02.solvePartTwo(contents)});
                },
                3 => {
                    var timer = try std.time.Timer.start();
                    const p1 = try aoc2024.day03.solvePartOne(contents);
                    const t1 = timer.lap();
                    const p2 = try aoc2024.day03.solvePartTwo(contents);
                    const t2 = timer.lap();
                    std.debug.print("part 1: {d}, part 2: {d}\n", .{ t1 / 100000, t2 / 100000 });
                    try stdout.print("day 3 - part 1: {d}\n", .{p1});
                    try stdout.print("day 3 - part 2: {d}\n", .{p2});
                },
                4 => {
                    var timer = try std.time.Timer.start();
                    const p1 = try aoc2024.day04.solvePartOne(contents);
                    const t1 = timer.lap();
                    const p2 = try aoc2024.day04.solvePartTwo(contents);
                    const t2 = timer.lap();
                    std.debug.print("part 1: {d}, part 2: {d}\n", .{ t1 / 100000, t2 / 100000 });
                    try stdout.print("day 4 - part 1: {d}\n", .{p1});
                    try stdout.print("day 4 - part 2: {d}\n", .{p2});
                },
                else => {
                    try stdout.print("unknown day\n", .{});
                },
            }
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
