// flume's hooks module (C.4), the one module hooks.json names, since the
// pinned Claude Code takes one hooks module per plugin, each event's hook
// once in it, and `$` into no function of another file. It does nothing
// outside a session flume started, its environment naming FLUME_PLACE and
// FLUME_AGENT, and registers no clock there. Its clocks run in the agent
// program's own process and end with its session, each tick one command of
// flume's that reads and ends (core.13). It has two parts.
//
// The place's line under the prompt (design 4): the plugin pins the line
// `flume hook status-line` prints as its own status line, at the session's
// start, after each tool call, at each turn's end and every ten seconds, so
// a hold, a stall or a stop-short shows while the session is idle. The
// words are the core's; the module computes none, decides nothing and
// writes nothing of flume's (core.14).
//
// The deliver part (215, C.4): where the environment also names
// FLUME_DELIVER, which the profile sets only where it offers deliver, the
// plugin runs `flume hook deliver` at the session's start and at each
// turn's end, each after the engine's own, and every ten seconds while no
// turn runs, one run at a time, and submits what a run that exits 0 prints
// as the person's own words, a turn of its own once the session is idle,
// leaving the prompt box as it stands. The words are the core's take: every
// delivery pending for the agent, or a message waiting on its start,
// recorded handed when taken.

import type { EngineInterface, Register } from "claude-code";

type $ = EngineInterface;

/** flume's binary, which the profile writes by its absolute path, as it writes the hooks'. */
const FLUME = "flume";

/** How often the line, and the take while no turn runs, is asked again. */
export const EVERY_MS = 10_000;

/** How long one run of a hook may take before it is let go. */
const RUN_MS = 10_000;

// Whether flume started this session, read once from its environment.
let ours: Promise<boolean> | null = null;

// Whether this session takes its deliveries itself, read once from its
// environment.
let delivers: Promise<boolean> | null = null;

// Whether a turn runs, read from `turn.start` and `turn.complete`.
let turning = false;

// Whether a run of `flume hook deliver` is under way: one at a time.
let taking = false;

function started($: $): Promise<boolean> {
  ours ??= Promise.all([$.env.get("FLUME_PLACE"), $.env.get("FLUME_AGENT")])
    .then(([place, agent]) => Boolean(place) && Boolean(agent))
    .catch(() => false);
  return ours;
}

function delivering($: $): Promise<boolean> {
  delivers ??= Promise.all([started($), $.env.get("FLUME_DELIVER")])
    .then(([ours, deliver]) => ours && Boolean(deliver))
    .catch(() => false);
  return delivers;
}

/**
 * Pins the first line `flume hook status-line` prints, run in the session's
 * folder, or removes the line where it prints nothing; a run that fails
 * changes nothing pinned.
 */
async function draw($: $): Promise<void> {
  const ran = await $.process
    .run([FLUME, "hook", "status-line"], { timeoutMs: RUN_MS })
    .catch(() => null);
  if (ran === null || ran.exitCode !== 0) return;
  const line = ran.stdout.split("\n")[0]?.trim() ?? "";
  $.ui.status(line === "" ? undefined : line);
}

/**
 * Runs `flume hook deliver` and submits what it prints as the person's own
 * words; a run that fails, or prints nothing, submits nothing.
 */
async function take($: $): Promise<void> {
  if (taking) return;
  taking = true;
  try {
    const ran = await $.process
      .run([FLUME, "hook", "deliver"], { timeoutMs: RUN_MS })
      .catch(() => null);
    if (ran === null || ran.exitCode !== 0) return;
    const text = ran.stdout.replace(/\n+$/, "");
    if (text.trim() === "") return;
    await $.prompt.submit({ text, asUser: true }).catch(() => undefined);
  } finally {
    taking = false;
  }
}

export const register: Register = (on) => {
  on("session.start", async ($, e, next) => {
    const result = await next(e);
    if (await started($)) {
      void draw($);
      $.clock.every(EVERY_MS, () => draw($));
    }
    if (await delivering($)) {
      void take($);
      $.clock.every(EVERY_MS, async () => {
        if (!turning) await take($);
      });
    }
    return result;
  });

  on("tool.call", async ($, e, next) => {
    const result = await next(e);
    if (await started($)) void draw($);
    return result;
  }).catch(($, e, next) => next(e)); // never in the way of the tool itself

  on("turn.start", async ($, e, next) => {
    turning = true;
    return next(e);
  });

  on("turn.complete", async ($, e, next) => {
    const result = await next(e);
    turning = false;
    if (await started($)) void draw($);
    if (await delivering($)) void take($);
    return result;
  });
};
