// The module's deliver part (hooks/flume.ts), run by the pinned engine's
// `plugin test` over a host whose `flume hook deliver` prints, exits
// non-zero or fails as each test says: the claude-code-plugin spec's
// deliver scenarios (215, C.4; core.13). The module's line part runs
// beside it, so each test reads the runs of `flume hook deliver` alone.

import type { On } from "claude-code";
import { expect, mock, test } from "claude-code/testing";

const WHOLE = { isStdoutTruncated: false, isStderrTruncated: false };
const FOLDER = "/places/turn--east-ops";
const AGENT = {
  FLUME_PLACE: FOLDER,
  FLUME_AGENT: "flume:b3e4c15d5ebe:turn--east-ops:plan:1",
};
const DELIVERING = { ...AGENT, FLUME_DELIVER: "1" };
const DELIVER = "flume hook deliver";
const BATCH =
  "ops messages east/ops:\nthe nightly job failed; confirm: flume confirm --recipient east/ops message/east/ops/0a1b2c3d\n\nflume: fix--one (mend it) code delivered [stage/code/1/delivered]; read: flume status fix--one; confirm: flume confirm --recipient east/ops fix--one/0a1b2c3d/stage/code/1/delivered";

/** What the host's `flume hook deliver` does at its next run. */
type Hook = { prints: string } | { exits: number } | { fails: true };

/**
 * A host whose session runs with the variables `env`, whose `flume hook
 * deliver` does as `seen.hook` says and whose `flume hook status-line`
 * prints nothing, and which keeps each run of `flume hook deliver`, each
 * prompt submitted, whether as the person's words, and each write of the
 * prompt box.
 */
function host(on: On, env: Record<string, string>) {
  const seen = {
    hook: { prints: "" } as Hook,
    ran: [] as string[],
    submitted: [] as { text: string; asUser: boolean }[],
    filled: 0,
  };
  mock.env(on, env);
  on("session.start", ($, e) => ({ cwd: e.cwd }));
  on("process.run", ($, e) => {
    const argv = e.argv.join(" ");
    if (argv !== DELIVER) {
      return { value: { exitCode: 0, stdout: "", stderr: "", ...WHOLE } };
    }
    seen.ran.push(argv);
    const hook = seen.hook;
    if ("fails" in hook) throw new Error("flume: no such file or directory");
    if ("exits" in hook)
      return { value: { exitCode: hook.exits, stdout: "", stderr: "flume: no", ...WHOLE } };
    return { value: { exitCode: 0, stdout: hook.prints, stderr: "", ...WHOLE } };
  });
  on("prompt.submit", ($, e) => {
    seen.submitted.push({
      text: e.text,
      asUser: e.origin.kind === "plugin" && e.origin.asUser === true,
    });
    return { text: e.text };
  });
  on("prompt.fill", () => {
    seen.filled += 1;
    return { isFilled: true };
  });
  on("ui.status", () => ({ value: undefined }));
  on("turn.start", ($, e) => ({ turnId: e.turnId }));
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

const COMPLETE = {
  answer: "done",
  durationMs: 1,
  isAborted: false,
  turnId: "turn-1",
  reason: "answer",
} as const;

test("words at a turn's end are submitted once as the person's own, the prompt box untouched", async ($, on) => {
  const { seen, clock } = clocked(on, DELIVERING);
  await $.session.start(START);
  await clock.settle();
  expect(seen.ran).toEqual([DELIVER]);
  expect(seen.submitted).toEqual([]);
  await $.turn.start({ text: "go on", turnId: "turn-1" });
  seen.hook = { prints: `${BATCH}\n` };
  await $.turn.complete(COMPLETE);
  await clock.settle();
  expect(seen.ran).toEqual([DELIVER, DELIVER]);
  expect(seen.submitted).toEqual([{ text: BATCH, asUser: true }]);
  expect(seen.filled).toBe(0);
});

test("nothing printed, or a run that exits 1 or fails, submits nothing", async ($, on) => {
  const { seen, clock } = clocked(on, DELIVERING);
  await $.session.start(START);
  await clock.settle();
  for (const hook of [{ prints: "" }, { exits: 1 }, { fails: true }] as Hook[]) {
    seen.hook = hook;
    await $.turn.start({ text: "go on", turnId: "turn-1" });
    await $.turn.complete(COMPLETE);
    await clock.settle();
  }
  expect(seen.ran.length).toBe(4);
  expect(seen.submitted).toEqual([]);
});

test("idle ticks run the hook every ten seconds, and none while a turn runs", async ($, on) => {
  const { seen, clock } = clocked(on, DELIVERING);
  await $.session.start(START);
  await clock.settle();
  const atStart = seen.ran.length;
  await clock.advance(30_000);
  expect(seen.ran.length - atStart).toBe(3);
  await $.turn.start({ text: "go on", turnId: "turn-2" });
  await clock.advance(30_000);
  expect(seen.ran.length - atStart).toBe(3);
  expect(seen.submitted).toEqual([]);
});

const WITHOUT: [string, Record<string, string>, number][] = [
  ["FLUME_PLACE and FLUME_AGENT but no FLUME_DELIVER", AGENT, 1],
  ["nothing of flume's, a session flume did not start", {}, 0],
];

for (const [names, env, lines] of WITHOUT) {
  test(`a session naming ${names} asks no clock for deliveries and runs nothing`, async ($, on) => {
    const { seen, clocks } = counted(on, env);
    seen.hook = { prints: `${BATCH}\n` };
    await $.session.start(START);
    await $.turn.start({ text: "go on", turnId: "turn-1" });
    await $.turn.complete(COMPLETE);
    // The line module's clock alone, where flume started the session.
    expect(clocks.every).toBe(lines);
    expect(seen.ran).toEqual([]);
    expect(seen.submitted).toEqual([]);
  });
}
