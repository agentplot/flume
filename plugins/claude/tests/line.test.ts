// The module that pins the place's line (hooks/line.ts), run by the pinned
// engine's `plugin test` over a host whose `flume hook status-line` prints
// what each test says: the claude-code-plugin spec's module scenarios (56,
// 60, 72, C.4; core.13).

import type { On } from "claude-code";
import { expect, mock, test } from "claude-code/testing";

const WHOLE = { isStdoutTruncated: false, isStderrTruncated: false };
const FOLDER = "/places/fix--one";
const AGENT = { FLUME_PLACE: FOLDER, FLUME_AGENT: "flume:b3e4c15d5ebe:fix--one:code:1" };
const WORKING = "fix--one · fix@1 · code · run 1 · working";
const HELD = "fix--one · fix@1 · code · run 1 · held";

/** What the host's `flume hook status-line` does at its next run. */
type Hook = { prints: string } | { exits: number } | { fails: true };

/**
 * A host whose session runs with the variables `env`, whose `flume hook
 * status-line` does as `seen.hook` says, and which keeps every command
 * run, where it ran, and every line pinned.
 */
function host(on: On, env: Record<string, string>) {
  const seen = {
    hook: { prints: "" } as Hook,
    ran: [] as string[],
    cwd: [] as (string | undefined)[],
    pinned: [] as (string | undefined)[],
  };
  mock.env(on, env);
  on("session.start", ($, e) => ({ cwd: e.cwd }));
  on("process.run", ($, e) => {
    seen.ran.push(e.argv.join(" "));
    seen.cwd.push(e.init?.cwd);
    const hook = seen.hook;
    if ("fails" in hook) throw new Error("flume: no such file or directory");
    if ("exits" in hook)
      return { value: { exitCode: hook.exits, stdout: "", stderr: "flume: no", ...WHOLE } };
    return { value: { exitCode: 0, stdout: hook.prints, stderr: "", ...WHOLE } };
  });
  on("ui.status", ($, e) => {
    seen.pinned.push(e.text);
    return { value: undefined };
  });
  on("tool.call", () => ({ result: { stdout: "" }, text: "" }));
  on("turn.complete", ($, e) => ({ text: e.answer }));
  return seen;
}

/** The host, its clock in memory, moved only by the test. */
function clocked(on: On, env: Record<string, string>) {
  const seen = host(on, env);
  return { seen, clock: mock.clock(on) };
}

/**
 * The host, with no clock but one that counts each interval asked of it
 * and refuses it, which ends the interval.
 */
function counted(on: On, env: Record<string, string>) {
  const seen = host(on, env);
  const clocks = { every: 0 };
  on("clock.every", () => {
    clocks.every += 1;
    throw new Error("no clock here");
  });
  return { seen, clocks };
}

const START = { cwd: FOLDER, surface: "terminal", isInteractive: true } as const;

test("a code agent's session pins the place's line at its start", async ($, on) => {
  const { seen, clock } = clocked(on, AGENT);
  seen.hook = { prints: `${WORKING}\n` };
  await $.session.start(START);
  await clock.settle();
  expect(seen.pinned).toEqual([WORKING]);
  expect(seen.ran).toEqual(["flume hook status-line"]);
  // In the session's own folder.
  expect(seen.cwd).toEqual([undefined]);
});

test("the line is redrawn every ten seconds while the session is idle", async ($, on) => {
  const { seen, clock } = clocked(on, AGENT);
  seen.hook = { prints: `${WORKING}\n` };
  await $.session.start(START);
  await clock.settle();
  seen.hook = { prints: `${HELD}\n` };
  await clock.advance(9_999);
  expect(seen.pinned).toEqual([WORKING]);
  await clock.advance(1);
  expect(seen.pinned.at(-1)).toBe(HELD);
  expect(seen.ran.length).toBe(2);
});

test("a place the core no longer reads removes the line", async ($, on) => {
  const { seen, clock } = clocked(on, AGENT);
  seen.hook = { prints: `${WORKING}\n` };
  await $.session.start(START);
  await clock.settle();
  seen.hook = { prints: "" };
  await clock.advance(10_000);
  expect(seen.pinned).toEqual([WORKING, undefined]);
});

test("a run that fails changes nothing pinned", async ($, on) => {
  const { seen, clock } = clocked(on, AGENT);
  seen.hook = { prints: `${WORKING}\n` };
  await $.session.start(START);
  await clock.settle();
  seen.hook = { exits: 1 };
  await clock.advance(10_000);
  seen.hook = { fails: true };
  await clock.advance(10_000);
  expect(seen.ran.length).toBe(3);
  expect(seen.pinned).toEqual([WORKING]);
});

test("only the first line printed is pinned", async ($, on) => {
  const { seen, clock } = clocked(on, AGENT);
  seen.hook = { prints: `${WORKING}\nmore\n` };
  await $.session.start(START);
  await clock.settle();
  expect(seen.pinned).toEqual([WORKING]);
});

test("after each tool call and at each turn's end", async ($, on) => {
  const { seen, clock } = clocked(on, AGENT);
  seen.hook = { prints: `${WORKING}\n` };
  await $.session.start(START);
  await clock.settle();
  seen.hook = { prints: `${HELD}\n` };
  await $.tool.call({ tool: "Bash", command: "true" });
  await clock.settle();
  expect(seen.pinned).toEqual([WORKING, HELD]);
  seen.hook = { prints: `${WORKING}\n` };
  await $.turn.complete({
    answer: "done",
    durationMs: 1,
    isAborted: false,
    turnId: "turn-1",
    reason: "answer",
  });
  await clock.settle();
  expect(seen.pinned).toEqual([WORKING, HELD, WORKING]);
  expect(seen.ran.length).toBe(3);
});

test("a code agent's session asks one clock", async ($, on) => {
  const { seen, clocks } = counted(on, AGENT);
  seen.hook = { prints: `${WORKING}\n` };
  await $.session.start(START);
  await $.tool.call({ tool: "Bash", command: "true" });
  expect(clocks.every).toBe(1);
});

const OUTSIDE: [string, Record<string, string>][] = [
  ["no FLUME_PLACE or FLUME_AGENT", {}],
  ["no FLUME_PLACE", { FLUME_AGENT: AGENT.FLUME_AGENT }],
  ["no FLUME_AGENT", { FLUME_PLACE: FOLDER }],
];

for (const [names, env] of OUTSIDE) {
  test(`a session whose environment names ${names} pins nothing, runs nothing and asks no clock`, async ($, on) => {
    const { seen, clocks } = counted(on, env);
    seen.hook = { prints: `${WORKING}\n` };
    await $.session.start(START);
    await $.tool.call({ tool: "Bash", command: "true" });
    await $.turn.complete({
      answer: "done",
      durationMs: 1,
      isAborted: false,
      turnId: "turn-1",
      reason: "answer",
    });
    expect(clocks.every).toBe(0);
    expect(seen.ran).toEqual([]);
    expect(seen.pinned).toEqual([]);
  });
}
