// The place's line under the prompt (design 4): in a session flume started,
// its environment naming FLUME_PLACE and FLUME_AGENT, flume's plugin pins
// the line `flume hook status-line` prints as its own status line, at the
// session's start, after each tool call, at each turn's end and every ten
// seconds, so a hold, a stall or a stop-short shows while the session is
// idle. The words are the core's; the module computes none, decides
// nothing and writes nothing of flume's. Its clock runs in the agent
// program's own process and ends with its session, each tick one command
// of flume's that reads and ends (core.13, core.14). Outside a session
// flume started it pins nothing, runs nothing and registers no clock.

import type { EngineInterface, Register } from "claude-code";

type $ = EngineInterface;

/** flume's binary, which the profile writes by its absolute path, as it writes the hooks'. */
const FLUME = "flume";

/** How often the line is asked again while the session lives. */
export const EVERY_MS = 10_000;

/** How long one run of the hook may take before it is let go. */
const RUN_MS = 10_000;

// Whether flume started this session, read once from its environment.
let ours: Promise<boolean> | null = null;

function started($: $): Promise<boolean> {
  ours ??= Promise.all([$.env.get("FLUME_PLACE"), $.env.get("FLUME_AGENT")])
    .then(([place, agent]) => Boolean(place) && Boolean(agent))
    .catch(() => false);
  return ours;
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

export const register: Register = (on) => {
  on("session.start", async ($, e, next) => {
    const result = await next(e);
    if (await started($)) {
      void draw($);
      $.clock.every(EVERY_MS, () => draw($));
    }
    return result;
  });

  on("tool.call", async ($, e, next) => {
    const result = await next(e);
    if (await started($)) void draw($);
    return result;
  }).catch(($, e, next) => next(e)); // never in the way of the tool itself

  on("turn.complete", async ($, e, next) => {
    const result = await next(e);
    if (await started($)) void draw($);
    return result;
  });
};
