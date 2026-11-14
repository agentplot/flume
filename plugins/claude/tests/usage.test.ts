// The module that passes the account's usage windows (hooks/usage.ts), run
// by the pinned engine's `plugin test` over a host whose `flume hook usage`
// does what each test says: the claude-code-plugin spec's scenarios of the
// module (184, 211, C.4; core.13).

import type { On, SessionMeasureInput, SessionRateLimit, UsageUnit } from "claude-code";
import { expect, mock, test } from "claude-code/testing";

const WHOLE = { isStdoutTruncated: false, isStderrTruncated: false };
const AGENT = {
  FLUME_PLACE: "/places/fix--one",
  FLUME_AGENT: "flume:b3e4c15d5ebe:fix--one:code:1",
};
const FIVE_HOUR: SessionRateLimit = {
  kind: "five_hour",
  percentUsed: 42,
  resetsAt: "2026-11-05T17:00:00.000Z",
};
const SEVEN_DAY: SessionRateLimit = { kind: "seven_day", percentUsed: 18 };

/** What the host's `flume hook usage` does at its next run. */
type Hook = { exits: number } | { fails: true };

/**
 * A host whose session runs with the variables `env`, whose `flume hook
 * usage` does as `seen.hook` says, and which keeps every command run, the
 * standard input it was given and its bound; its clock in memory.
 */
function host(on: On, env: Record<string, string>) {
  const seen = {
    hook: { exits: 0 } as Hook,
    ran: [] as string[],
    stdin: [] as (string | undefined)[],
    bound: [] as (number | undefined)[],
  };
  mock.env(on, env);
  on("session.measure", ($, e) => ({ changed: e.changed }));
  on("process.run", ($, e) => {
    seen.ran.push(e.argv.join(" "));
    seen.stdin.push(e.init?.stdin);
    seen.bound.push(e.init?.timeoutMs);
    const hook = seen.hook;
    if ("fails" in hook) throw new Error("flume: no such file or directory");
    return { value: { exitCode: hook.exits, stdout: "", stderr: "", ...WHOLE } };
  });
  return { seen, clock: mock.clock(on) };
}

/** A measurement whose `changed` names `changed` and whose windows are `rateLimits`. */
function measured(changed: UsageUnit[], rateLimits: SessionRateLimit[]): SessionMeasureInput {
  return { context: { tokens: 21_000, window: 200_000, percent: 10 }, rateLimits, changed };
}

test("a report passed", async ($, on) => {
  const { seen, clock } = host(on, AGENT);
  const result = await $.session.measure(
    measured(["context", "rateLimits"], [FIVE_HOUR, SEVEN_DAY]),
  );
  await clock.settle();
  expect(result).toEqual({ changed: ["context", "rateLimits"] });
  expect(seen.ran).toEqual(["flume hook usage"]);
  expect(seen.stdin.map((stdin) => JSON.parse(stdin ?? "null"))).toEqual([
    { rateLimits: [FIVE_HOUR, SEVEN_DAY] },
  ]);
  expect(seen.bound).toEqual([10_000]);
});

test("a measure of the context alone", async ($, on) => {
  const { seen, clock } = host(on, AGENT);
  await $.session.measure(measured(["context"], [FIVE_HOUR, SEVEN_DAY]));
  await clock.settle();
  expect(seen.ran).toEqual([]);
});

test("no window to pass", async ($, on) => {
  const { seen, clock } = host(on, AGENT);
  await $.session.measure(measured(["rateLimits"], []));
  await clock.settle();
  expect(seen.ran).toEqual([]);
});

test("a run that fails", async ($, on) => {
  const { seen, clock } = host(on, AGENT);
  seen.hook = { exits: 1 };
  const failed = await $.session.measure(measured(["rateLimits"], [FIVE_HOUR]));
  await clock.settle();
  seen.hook = { fails: true };
  const unfound = await $.session.measure(measured(["rateLimits"], [SEVEN_DAY]));
  await clock.settle();
  seen.hook = { exits: 0 };
  await $.session.measure(measured(["rateLimits"], [FIVE_HOUR, SEVEN_DAY]));
  await clock.settle();
  expect(failed).toEqual({ changed: ["rateLimits"] });
  expect(unfound).toEqual({ changed: ["rateLimits"] });
  expect(seen.ran).toEqual(["flume hook usage", "flume hook usage", "flume hook usage"]);
});

const OUTSIDE: [string, Record<string, string>][] = [
  ["no FLUME_PLACE or FLUME_AGENT", {}],
  ["no FLUME_PLACE", { FLUME_AGENT: AGENT.FLUME_AGENT }],
  ["no FLUME_AGENT", { FLUME_PLACE: AGENT.FLUME_PLACE }],
];

for (const [names, env] of OUTSIDE) {
  test(`outside a session flume started: a session whose environment names ${names} runs no process`, async ($, on) => {
    const { seen, clock } = host(on, env);
    await $.session.measure(measured(["context", "rateLimits"], [FIVE_HOUR, SEVEN_DAY]));
    await clock.settle();
    expect(seen.ran).toEqual([]);
  });
}
