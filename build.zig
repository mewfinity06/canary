const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "canary",
        .root_module = b.addModule("canary", .{
            .target = target,
            .optimize = optimize,
        }),
    });

    exe.linkLibC();

    exe.addCSourceFiles(.{
        .files = &.{
            // source
            "./source/main.c",
            "./source/canary.c",
            // Lexer
            "./source/lexer/lexer.c",
            "./source/lexer/token.c",
            // Parser
            "./source/parser/parser.c",
            // vendor
            "./vendor/flag.c",
        },
    });

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
