// The deliver part (215, C.4): in a session flume started whose
// environment names FLUME_PLACE, FLUME_AGENT and FLUME_DELIVER, which the
// profile sets only where it offers deliver, flume's plugin runs
// `flume hook deliver` at the session's start and at each turn's end, each
// after the engine's own, and every ten seconds while no turn runs, one run
// at a time, and submits what a run that exits 0 prints as the person's own
// words, a turn of its own once the session is idle, leaving the prompt box
// as it stands. The words are the core's take: every delivery pending for
// the agent, or a message waiting on its start, recorded handed when taken;
// the module decides nothing and writes nothing of flume's. Its clock runs
// in the agent program's own process and ends with its session, each tick
// one command of flume's that reads, records and ends (core.13). Outside
// such a session it registers no clock and runs nothing.

import type { EngineInterface, Register } from "claude-code";

type $ = EngineInterface;

/** flume's binary, which the profile writes by its absolute path, as it writes the hooks'. */
const FLUME = "flume";

/** How often the take is asked again while no turn runs. */
export const EVERY_MS = 10_000;

/** How long one run of the hook may take before it is let go. */
const RUN_MS = 10_000;

// Whether this session takes its deliveries itself, read once from its
// environment.
let ours: Promise<boolean> | null = null;

// Whether a turn runs, read from `turn.start` and `turn.complete`.
let turning = false;

// Whether a run of the hook is under way: one at a time.
let running = false;

function delivering($: $): Promise<boolean> {
  ours ??= Promise.all([
    $.env.get("FLUME_PLACE"),
    $.env.get("FLUME_AGENT"),
    $.env.get("FLUME_DELIVER"),
  ])
    .then(([place, agent, deliver]) => Boolean(place) && Boolean(agent) && Boolean(deliver))
    .catch(() => false);
  return ours;
}

/**
 * Runs `flume hook deliver` and submits what it prints as the person's own
 * words; a run that fails, or prints nothing, submits nothing.
 */
async function take($: $): Promise<void> {
  if (running) return;
  running = true;
  try {
    const ran = await $.process
      .run([FLUME, "hook", "deliver"], { timeoutMs: RUN_MS })
      .catch(() => null);
    if (ran === null || ran.exitCode !== 0) return;
    const text = ran.stdout.replace(/\n+$/, "");
    if (text.trim() === "") return;
    await $.prompt.submit({ text, asUser: true }).catch(() => undefined);
  } finally {
    running = false;
  }
}

export const register: Register = (on) => {
  on("session.start", async ($, e, next) => {
    const result = await next(e);
    if (await delivering($)) {
      void take($);
      $.clock.every(EVERY_MS, async () => {
        if (!turning) await take($);
      });
    }
    return result;
  });

  on("turn.start", async ($, e, next) => {
    turning = true;
    return next(e);
  });

  on("turn.complete", async ($, e, next) => {
    const result = await next(e);
    turning = false;
    if (await delivering($)) void take($);
    return result;
  });
};
