// The account's usage windows (184, 211, C.4): in a session flume started,
// its environment naming FLUME_PLACE and FLUME_AGENT, flume's plugin passes
// each report of the rate-limit windows the engine measures to `flume hook
// usage`, which keeps them beside the start's record and has the core take
// them as the reading of the account the start runs under. A report goes
// only where the measurement's `changed` names `rateLimits` and its
// `rateLimits` holds a window: the engine's first measurement names every
// unit it has a figure for, and an empty list is the engine having no
// reading, off a subscription or before the first response, which would
// replace a reading kept with nothing. The module waits on no run, so the
// engine's measuring is never held, and a run that fails changes nothing
// and throws nothing into the engine. It computes nothing, decides nothing
// and writes nothing of flume's; each run is one command of flume's that
// ends (core.13). Outside a session flume started it runs nothing.

import type { EngineInterface, Register, SessionMeasureInput } from "claude-code";

type $ = EngineInterface;

/** flume's binary, which the profile writes by its absolute path, as it writes the hooks'. */
const FLUME = "flume";

/** How long one run of the hook may take before it is let go. */
export const RUN_MS = 10_000;

// Whether flume started this session, read once from its environment.
let ours: Promise<boolean> | null = null;

function started($: $): Promise<boolean> {
  ours ??= Promise.all([$.env.get("FLUME_PLACE"), $.env.get("FLUME_AGENT")])
    .then(([place, agent]) => Boolean(place) && Boolean(agent))
    .catch(() => false);
  return ours;
}

/** Whether the measurement reports windows that moved: its `changed` names `rateLimits` and it holds one. */
function reports(e: SessionMeasureInput): boolean {
  return e.changed.includes("rateLimits") && e.rateLimits.length > 0;
}

/**
 * Runs `flume hook usage` with the windows on its standard input, as the
 * engine gives them, and goes on: a run that cannot start, fails or outlasts
 * its bound changes nothing.
 */
function pass($: $, e: SessionMeasureInput): void {
  try {
    void $.process
      .run([FLUME, "hook", "usage"], {
        stdin: JSON.stringify({ rateLimits: e.rateLimits }),
        timeoutMs: RUN_MS,
      })
      .catch(() => undefined);
  } catch {
    // A run that cannot be asked changes nothing.
  }
}

export const register: Register = (on) => {
  on("session.measure", async ($, e, next) => {
    const result = await next(e);
    if (reports(e) && (await started($))) pass($, e);
    return result;
  });
};
