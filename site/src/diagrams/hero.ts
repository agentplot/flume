// The start page's diagram: one host's work, in one 36 s loop. Five
// launches come in, each naming its kind, its points and its stream. The
// host runs three agents at once across two streams: work, priority 10,
// with a bound of three agents and of 13 points in flight, on a pool of two
// accounts filled first to last; and personal, priority 0, with two agents
// and a pace of 8 points a day, on a pool of one. csv-export, parser-tabs
// and bump-deps start at once, each in a place of its own, a fresh agent
// for each stage. sso-login waits in work's queue, short of its points
// bound, until csv-export's last stage delivers and its points leave
// flight; readme-tour waits in personal's, short of its pace, even once
// personal has room. Each place's last delivery submits it; sluice lands
// it on main, and flume tidies the place.
//
// Timeline, percent of the loop:
//   2-22 launches · 5.5-80 stages · 28-88 landings · 37-92 tidies
import { Diagram, esc, type Layout } from "./kit";

const T = 36;
// The loop's end: everything fades back to an empty host after it.
const END = 96;

const HOST = 3;
interface Stream {
  name: string;
  priority: number;
  agents: number;
  pool: string;
  points?: number;
  pace?: number;
}
const STREAMS: Stream[] = [
  { name: "work", priority: 10, agents: 3, points: 13, pool: "work, fill-first" },
  { name: "personal", priority: 0, agents: 2, pace: 8, pool: "personal, least-busy" },
];

interface Stage {
  name: string;
  a: number;
  b: number;
}
// A launch's place, once it starts: its row in its stream, the account
// its agents run on, its stages, and when it lands and is tidied.
interface Run {
  start: number;
  slot: number;
  account: string;
  stages: Stage[];
  landed: number;
  tidied: number;
  gone: number;
}
// One launch: what it names, when it comes in, the room it is short of
// while it waits, and its place once it starts.
interface Work {
  kind: string;
  name: string;
  points: number;
  stream: number;
  launched: number;
  waits?: string;
  run?: Run;
}
const WORK: Work[] = [
  {
    kind: "change",
    name: "csv-export",
    points: 8,
    stream: 0,
    launched: 2,
    run: {
      start: 5.5,
      slot: 0,
      account: "work-1",
      stages: [
        { name: "construct", a: 5.5, b: 18 },
        { name: "code", a: 18, b: 42 },
        { name: "verify", a: 42, b: 52 },
      ],
      landed: 60,
      tidied: 63,
      gone: 64,
    },
  },
  {
    kind: "fix",
    name: "parser-tabs",
    points: 2,
    stream: 0,
    launched: 6,
    run: {
      start: 9.5,
      slot: 1,
      account: "work-2",
      stages: [{ name: "code", a: 9.5, b: 26 }],
      landed: 34,
      tidied: 37,
      gone: 38,
    },
  },
  {
    kind: "chore",
    name: "bump-deps",
    points: 5,
    stream: 1,
    launched: 10,
    run: {
      start: 13.5,
      slot: 0,
      account: "personal",
      stages: [{ name: "code", a: 13.5, b: 36 }],
      landed: 44,
      tidied: 47,
      gone: 48,
    },
  },
  {
    kind: "change",
    name: "sso-login",
    points: 8,
    stream: 0,
    launched: 14,
    waits: "short of its points bound",
    run: {
      start: 54.2,
      slot: 1,
      account: "work-1",
      stages: [
        { name: "construct", a: 54.2, b: 62 },
        { name: "code", a: 62, b: 74 },
        { name: "verify", a: 74, b: 80 },
      ],
      landed: 88,
      tidied: 91,
      gone: 92,
    },
  },
  {
    kind: "chore",
    name: "readme-tour",
    points: 5,
    stream: 1,
    launched: 18,
    waits: "short of its pace",
  },
];

// When a launch reaches its stream's queue, and when a waiting one leaves it.
const queued = (w: Work) => w.launched + 2.3;
const leaves = (w: Work) => (w.run ? w.run.start - 1.4 : END);
const delivered = (r: Run) => r.stages[r.stages.length - 1].b;
// An agent runs and its points are in flight from the first start until
// the last stage delivers (165).
const running = (w: Work, t: number) => !!w.run && w.run.start <= t && t < delivered(w.run);
const waiting = (w: Work, t: number) => !!w.waits && queued(w) <= t && t < leaves(w);

// Every moment something of the host's changes.
const EVENTS = [
  ...new Set(
    WORK.flatMap((w) => [
      queued(w),
      leaves(w),
      ...(w.run
        ? [
            w.run.start,
            delivered(w.run),
            delivered(w.run) + 2,
            w.run.landed,
            w.run.tidied,
            w.run.gone,
          ]
        : []),
    ]).filter((t) => t < END),
  ),
].sort((x, y) => x - y);

// A value of the host's over the loop: [at, value] each time it changes.
function over<V>(f: (t: number) => V): [number, V][] {
  const out: [number, V][] = [];
  for (const t of [0, ...EVENTS]) {
    const v = f(t);
    if (!out.length || out[out.length - 1][1] !== v) out.push([t, v]);
  }
  return out;
}
// Each value's window, from its time until the next's, the last until END.
const windows = <V>(steps: [number, V][]) =>
  steps.map(([at, v], i) => [at, i + 1 < steps.length ? steps[i + 1][0] : END, v] as const);

const agents = (k: number) => (t: number) =>
  WORK.filter((w) => w.stream === k && running(w, t)).length;
const inFlight = (k: number) => (t: number) =>
  WORK.filter((w) => w.stream === k && running(w, t)).reduce((n, w) => n + w.points, 0);
// A pace charges a place's points at its first start, and the charge
// stands (167).
const charged = (k: number) => (t: number) =>
  WORK.filter((w) => w.stream === k && !!w.run && w.run.start <= t).reduce(
    (n, w) => n + w.points,
    0,
  );
const onHost = (t: number) => STREAMS.reduce((n, _, k) => n + agents(k)(t), 0);
// The places waiting for room in the order they would start: streams of
// higher priority first, then in the order they came (70).
const queue = (t: number) =>
  WORK.filter((w) => waiting(w, t))
    .sort((x, y) => STREAMS[y.stream].priority - STREAMS[x.stream].priority)
    .map((w) => w.name)
    .join(", then ");

interface Box {
  x: number;
  y: number;
  w: number;
  h: number;
}
interface Pt {
  x: number;
  y: number;
}

function geometry(layout: Layout) {
  if (layout === "wide") {
    return {
      W: 900,
      H: 556,
      n: false,
      card: (i: number): Box => ({ x: 20, y: 76 + i * 58, w: 160, h: 50 }),
      host: { x: 196, y: 72, w: 528, h: 28 } as Box,
      lane: (k: number): Box => ({ x: 196, y: 110 + k * 168, w: 528, h: 160 }),
      // Where the facts of a lane end and its places begin.
      split: 256,
      row: (lane: Box, slot: number): Box => ({
        x: lane.x + 264,
        y: lane.y + 12 + slot * 56,
        w: 252,
        h: 48,
      }),
      sluice: { x: 740, y: 72, w: 140, h: 356 } as Box,
      caps: [20, 196, 460, 740],
      capY: 462,
      footY: 540,
    };
  }
  return {
    W: 360,
    H: 1086,
    n: true,
    card: (i: number): Box => ({
      x: 8 + (i % 2) * 176,
      y: 86 + Math.floor(i / 2) * 52,
      w: 168,
      h: 44,
    }),
    host: { x: 8, y: 248, w: 344, h: 44 } as Box,
    lane: (k: number): Box => ({ x: 8, y: [304, 590][k], w: 344, h: [274, 220][k] }),
    split: 0,
    row: (lane: Box, slot: number): Box => ({
      x: lane.x + 12,
      y: lane.y + 160 + slot * 56,
      w: 320,
      h: 48,
    }),
    sluice: { x: 8, y: 838, w: 344, h: 108 } as Box,
    caps: [0, 0, 0, 0],
    capY: 978,
    footY: 1056,
  };
}

const pillW = (s: string) => Math.round(s.length * 5.7 + 16);

export function hero(layout: Layout, opts: { id?: string; standalone?: boolean } = {}) {
  const g = geometry(layout);
  const n = g.n;
  const d = new Diagram(T);
  const add = (s: string) => d.add(s);

  add(`<rect class="bg" width="${g.W}" height="${g.H}" rx="10"/>`);
  if (n) {
    add(`<text class="h" x="12" y="30">flume</text>`);
    add(`<text class="m xs" x="12" y="50">every launch waits in its stream for room</text>`);
    add(`<text class="hd" x="12" y="76">LAUNCHES</text>`);
  } else {
    add(`<text class="h" x="20" y="32">flume</text>`);
    add(
      `<text class="m sm" x="80" y="32">every launch waits in its stream for room, runs in a place of its own, and lands</text>`,
    );
    add(`<text class="hd" x="20" y="62">LAUNCHES</text>`);
    add(`<text class="hd" x="196" y="62">THE HOST AND ITS STREAMS</text>`);
    add(`<text class="hd" x="${g.lane(0).x + 264}" y="62">PLACES</text>`);
    add(`<text class="hd" x="${g.sluice.x}" y="62">THE MERGE QUEUE</text>`);
  }

  // ---- the launches, each a card that fills as it comes in.
  WORK.forEach((w, i) => {
    const C = g.card(i);
    add(
      `<rect x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="6" style="fill:none;stroke:var(--dg-rule);stroke-width:1;stroke-dasharray:4 3"/>`,
    );
    add(`<g class="${d.show(`c${i}`, w.launched, END)}">`);
    add(`<rect class="box2" x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="6"/>`);
    add(
      `<rect class="accbox ${d.show(`ch${i}`, w.launched, w.launched + 3)}" x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="6"/>`,
    );
    const y1 = C.y + (n ? 18 : 20);
    const y2 = C.y + (n ? 34 : 38);
    add(`<text class="t mono xs b" x="${C.x + 10}" y="${y1}">${esc(w.name)}</text>`);
    add(
      `<text class="m xxs" x="${C.x + 10}" y="${y2}"><tspan class="t">${w.kind}</tspan> · ${w.points} points · <tspan class="t">${STREAMS[w.stream].name}</tspan></text>`,
    );
    add("</g>");
  });
  if (!n) {
    ["Every launch is taken and", "waits its turn: none is", "refused for want of room."].forEach(
      (t, k) => {
        add(`<text class="m xxs" x="20" y="${384 + k * 15}">${t}</text>`);
      },
    );
  }

  // ---- the host: its agents running under its bound, and its queue.
  {
    const H = g.host;
    add(`<rect class="panel" x="${H.x}" y="${H.y}" width="${H.w}" height="${H.h}" rx="6"/>`);
    const x0 = H.x + 12;
    const y1 = H.y + 18;
    add(`<text class="t b xs" x="${x0}" y="${y1}">host</text>`);
    windows(over(onHost)).forEach(([a, b, v], k) => {
      const words = `${v} agent${v === 1 ? "" : "s"} running of ${HOST}, room for ${HOST - v}`;
      add(
        `<text class="t xxs${v === HOST ? " b" : ""} ${d.show(`hr${k}`, a, b)}" x="${x0 + 34}" y="${y1}">${words}</text>`,
      );
    });
    const qx = n ? x0 : H.x + H.w - 12;
    const qy = n ? H.y + 36 : y1;
    const anchor = n ? "" : ` text-anchor="end"`;
    windows(over(queue)).forEach(([a, b, v], k) => {
      add(
        `<text class="m xxs ${d.show(`hq${k}`, a, b)}" x="${qx}" y="${qy}"${anchor}>${
          v ? `the queue: <tspan class="t mono">${v}</tspan>` : "the queue: no place waiting"
        }</text>`,
      );
    });
  }

  // ---- the streams, each a lane: its facts and queue, then its places.
  STREAMS.forEach((s, k) => {
    const L = g.lane(k);
    add(`<rect class="box2" x="${L.x}" y="${L.y}" width="${L.w}" height="${L.h}" rx="8"/>`);
    const x0 = L.x + 12;
    const right = n ? L.x + L.w - 12 : L.x + g.split - 12;
    add(
      `<text class="sm" x="${x0}" y="${L.y + 22}"><tspan class="m">stream </tspan><tspan class="t b">${s.name}</tspan></text>`,
    );
    add(
      `<text class="m xxs" x="${right}" y="${L.y + 22}" text-anchor="end">priority ${s.priority}</text>`,
    );
    // A meter: its label, a track and its value.
    const mx = x0 + 46;
    const mw = 72;
    const vx = x0 + 126;
    const meter = (row: number, label: string) => {
      const y = L.y + 42 + row * 18;
      add(`<text class="m xxs" x="${x0}" y="${y}">${label}</text>`);
      return y;
    };
    {
      // The agents running, a box for each the stream's bound allows.
      const y = meter(0, "agents");
      const bw = (mw - (s.agents - 1) * 4) / s.agents;
      const steps = over(agents(k));
      for (let j = 0; j < s.agents; j++) {
        const x = mx + j * (bw + 4);
        add(
          `<rect x="${x}" y="${y - 8}" width="${bw}" height="9" rx="2" style="fill:none;stroke:var(--dg-rule);stroke-width:1"/>`,
        );
        add(
          `<rect class="ac ${d.levels(
            `a${k}${j}`,
            steps
              .map(([at, v], i) => [i ? at + d.E : 0, v > j ? 1 : 0] as [number, number])
              .concat([[END + d.E, 0]]),
          )}" x="${x}" y="${y - 8}" width="${bw}" height="9" rx="2"/>`,
        );
      }
      windows(steps).forEach(([a, b, v], j) => {
        add(
          `<text class="t xxs ${d.show(`av${k}${j}`, a, b)}" x="${vx}" y="${y}">${v} of ${s.agents}</text>`,
        );
      });
    }
    {
      // Its points in flight under its bound, or what it started under its pace.
      const bound = s.points ?? s.pace ?? 1;
      const y = meter(1, s.points ? "points" : "pace");
      const steps = over(s.points ? inFlight(k) : charged(k));
      add(`<rect class="track" x="${mx}" y="${y - 7}" width="${mw}" height="7" rx="3.5"/>`);
      // Each change as its number's: the new value comes in over [at, at + E].
      const E = d.E;
      const fr: [number, string][] = [[0, `transform:scaleX(${steps[0][1] / bound})`]];
      steps.slice(1).forEach(([at, v], j) => {
        fr.push(
          [at, `transform:scaleX(${steps[j][1] / bound})`],
          [at + E, `transform:scaleX(${v / bound})`],
        );
      });
      fr.push(
        [END, `transform:scaleX(${steps[steps.length - 1][1] / bound})`],
        [END + E, "transform:scaleX(0)"],
        [100, "transform:scaleX(0)"],
      );
      d.kf(`pb${k}`, fr);
      add(
        `<rect class="bar ${d.anim(`pb${k}`)}" x="${mx}" y="${y - 7}" width="${mw}" height="7" rx="3.5"/>`,
      );
      windows(steps).forEach(([a, b, v], j) => {
        const words = s.points ? `${v} of ${bound} in flight` : `${v} of ${bound} a day`;
        add(`<text class="t xxs ${d.show(`pv${k}${j}`, a, b)}" x="${vx}" y="${y}">${words}</text>`);
      });
    }
    {
      const y = meter(2, "pool");
      add(`<text class="t xxs" x="${mx}" y="${y}">${s.pool}</text>`);
    }

    // The stream's queue: a place waiting for room, and the room it is short of.
    const qy = L.y + 104;
    add(
      `<line x1="${x0}" y1="${L.y + 90}" x2="${right}" y2="${L.y + 90}" style="stroke:var(--dg-rule);stroke-width:1"/>`,
    );
    add(`<text class="m xxs" x="${x0}" y="${qy}">waiting for room</text>`);
    const Q: Box = { x: x0, y: qy + 8, w: n ? 160 : 150, h: 20 };
    const mine = WORK.filter((w) => w.stream === k && w.waits);
    const none = windows(over((t) => mine.some((w) => waiting(w, t))));
    none.forEach(([a, b, busy], j) => {
      if (!busy)
        add(
          `<text class="m xxs ${d.show(`qn${k}${j}`, a, b)}" x="${Q.x + 8}" y="${Q.y + 14}">no place waiting</text>`,
        );
    });
    mine.forEach((w) => {
      const i = WORK.indexOf(w);
      add(`<g class="${d.show(`qw${i}`, queued(w), leaves(w))}">`);
      add(
        `<rect class="ghost" x="${Q.x}" y="${Q.y}" width="${Q.w}" height="${Q.h}" rx="10" style="stroke-dasharray:4 3"/>`,
      );
      add(`<text class="t mono xxs b" x="${Q.x + 10}" y="${Q.y + 14}">${esc(w.name)}</text>`);
      add(
        `<text class="m xxs" x="${Q.x + Q.w - 10}" y="${Q.y + 14}" text-anchor="end">${w.points} points</text>`,
      );
      add(`<text class="ac xxs b" x="${Q.x}" y="${Q.y + Q.h + 15}">${w.waits}</text>`);
      add("</g>");
    });

    // Where the stream runs no place.
    {
      const some = (t: number) =>
        WORK.some((w) => w.stream === k && !!w.run && w.run.start <= t && t < w.run.gone);
      const steps = over(some);
      const P = g.row(L, 0);
      const cx = P.x + P.w / 2;
      const cy = n ? P.y + 28 : L.y + L.h / 2 + 4;
      windows(steps).forEach(([a, b, v], j) => {
        if (!v)
          add(
            `<text class="m xxs ${d.show(`pe${k}${j}`, a, b)}" x="${cx}" y="${cy}" text-anchor="middle">no place running</text>`,
          );
      });
    }
  });

  // ---- each launch's way in: to its stream's queue, then, with room, to
  // its place.
  const qAt = (w: Work): Pt => {
    const L = g.lane(w.stream);
    return { x: L.x + 12 + (n ? 80 : 75), y: L.y + 122 };
  };
  const rowAt = (w: Work, r: Run): Pt => {
    const R = g.row(g.lane(w.stream), r.slot);
    return { x: R.x + 56, y: R.y + R.h / 2 };
  };
  const fromCard = (i: number): Pt => {
    const C = g.card(i);
    return n ? { x: C.x + C.w / 2, y: C.y + C.h } : { x: C.x + C.w, y: C.y + C.h / 2 };
  };
  const toQueue = (i: number, w: Work): Pt[] => {
    const s = fromCard(i);
    const q = qAt(w);
    return n ? [s, q] : [s, { x: 188, y: s.y }, { x: 188, y: q.y }, q];
  };
  const toRow = (w: Work, r: Run): Pt[] => {
    const q = qAt(w);
    const at = rowAt(w, r);
    const cx = g.lane(w.stream).x + g.split - 4;
    return n ? [q, at] : [q, { x: cx, y: q.y }, { x: cx, y: at.y }, at];
  };
  const chip = (cls: string, at: Pt, name: string) =>
    `<g class="${cls}"><rect class="chip" x="${at.x - 42}" y="${at.y - 9}" width="84" height="18" rx="9"/><text class="chipt mono xxs" x="${at.x}" y="${at.y + 4}" text-anchor="middle">${esc(name)}</text></g>`;
  WORK.forEach((w, i) => {
    const r = w.run;
    if (r && !w.waits) {
      const pts = [...toQueue(i, w), ...toRow(w, r).slice(1)];
      add(
        chip(d.route(`f${i}`, pts, w.launched + 0.3, r.start, r.start + 0.5), rowAt(w, r), w.name),
      );
      return;
    }
    add(
      chip(
        d.route(`f${i}`, toQueue(i, w), w.launched + 0.3, queued(w), queued(w) + 0.3),
        qAt(w),
        w.name,
      ),
    );
    if (r)
      add(
        chip(d.route(`g${i}`, toRow(w, r), leaves(w), r.start, r.start + 0.5), rowAt(w, r), w.name),
      );
  });

  // ---- the places: each a row in its stream, its stages lit in turn with
  // a fresh agent each, then submitted, landed and tidied.
  const S = g.sluice;
  const testY = n ? S.y + 42 : S.y + 74;
  WORK.forEach((w, i) => {
    const r = w.run;
    if (!r) return;
    const R = g.row(g.lane(w.stream), r.slot);
    const done = delivered(r);
    add(`<g class="${d.show(`r${i}`, r.start, r.tidied)}">`);
    add(`<rect class="box2" x="${R.x}" y="${R.y}" width="${R.w}" height="${R.h}" rx="6"/>`);
    add(
      `<text class="t mono xs b" x="${R.x + 10}" y="${R.y + 18}">${w.kind}--${esc(w.name)}</text>`,
    );
    add(
      `<text class="m xxs" x="${R.x + R.w - 10}" y="${R.y + 18}" text-anchor="end">${w.points} points</text>`,
    );
    let px = R.x + 10;
    const py = R.y + 26;
    r.stages.forEach((st, j) => {
      const pw = pillW(st.name);
      add(
        `<rect class="track" x="${px}" y="${py}" width="${pw}" height="15" rx="7.5" style="stroke:var(--dg-rule);stroke-width:1"/>`,
      );
      add(
        `<rect class="accbox ${d.show(`s${i}${j}w`, st.a, st.b)}" x="${px}" y="${py}" width="${pw}" height="15" rx="7.5"/>`,
      );
      add(
        `<rect class="okbox ${d.show(`s${i}${j}d`, st.b, r.tidied)}" x="${px}" y="${py}" width="${pw}" height="15" rx="7.5"/>`,
      );
      add(`<text class="t xxs b" x="${px + 8}" y="${py + 11}">${st.name}</text>`);
      px += pw + 4;
    });
    // At its right: the account its agent runs on, then the submission.
    const sx = R.x + R.w - 10;
    const sy = R.y + 37;
    add(
      `<text class="m xxs ${d.show(`ra${i}`, r.start, done)}" x="${sx}" y="${sy}" text-anchor="end">on ${r.account}</text>`,
    );
    add(
      `<text class="ac xxs b ${d.show(`rs${i}`, done, r.landed)}" x="${sx}" y="${sy}" text-anchor="end">submitted</text>`,
    );
    add(
      `<text class="ok xxs b ${d.show(`rl${i}`, r.landed, r.tidied)}" x="${sx}" y="${sy}" text-anchor="end">landed</text>`,
    );
    add("</g>");
    // Tidied: the place's outline, dashed, then nothing.
    add(`<g class="${d.show(`rt${i}`, r.tidied, r.gone)}">`);
    add(
      `<rect x="${R.x}" y="${R.y}" width="${R.w}" height="${R.h}" rx="6" style="fill:none;stroke:var(--dg-muted);stroke-width:1.2;stroke-dasharray:5 4"/>`,
    );
    add(
      `<text class="m xxs" x="${R.x + R.w / 2}" y="${R.y + R.h / 2 + 4}" text-anchor="middle">${w.kind}--${esc(w.name)} tidied</text>`,
    );
    add("</g>");
    // The submission, carried to the merge queue.
    const from = { x: R.x + R.w, y: R.y + R.h / 2 };
    const cx = n ? g.W - 5 : S.x - 8;
    const to = { x: n ? S.x + S.w - 14 : S.x + 6, y: testY - 4 };
    const pts = [from, { x: cx, y: from.y }, { x: cx, y: to.y }, to];
    add(
      `<circle class="chip ${d.route(`u${i}`, pts, done, done + 2, done + 2.3)}" cx="${to.x}" cy="${to.y}" r="5"/>`,
    );
  });

  // ---- the merge queue: each submission tested as merged and landed on main.
  {
    add(`<rect class="box2" x="${S.x}" y="${S.y}" width="${S.w}" height="${S.h}" rx="8"/>`);
    if (n) add(`<text class="hd" x="12" y="${S.y - 10}">THE MERGE QUEUE</text>`);
    const x0 = S.x + 12;
    add(`<text class="ok b sm" x="${x0}" y="${S.y + 22}">sluice</text>`);
    if (n)
      add(`<text class="m xxs" x="${x0 + 52}" y="${S.y + 22}">tests each change as merged</text>`);
    else {
      add(`<text class="m xxs" x="${x0}" y="${S.y + 40}">tests each change</text>`);
      add(`<text class="m xxs" x="${x0}" y="${S.y + 54}">as merged</text>`);
    }
    const landings = WORK.flatMap((w) => (w.run ? [{ name: w.name, ...w.run }] : [])).sort(
      (x, y) => x.landed - y.landed,
    );
    const idle = windows(
      over((t) => landings.find((w) => delivered(w) + 2 <= t && t < w.landed)?.name ?? ""),
    );
    idle.forEach(([a, b, v], j) => {
      add(
        v
          ? `<text class="ac xxs b ${d.show(`sq${j}`, a, b)}" x="${x0}" y="${testY}">testing ${esc(v)}</text>`
          : `<text class="m xxs ${d.show(`sq${j}`, a, b)}" x="${x0}" y="${testY}">waits on a submission</text>`,
      );
    });
    // The test's progress, under its words.
    const bx1 = S.x + S.w - 12;
    add(`<rect class="track" x="${x0}" y="${testY + 8}" width="${bx1 - x0}" height="5" rx="2.5"/>`);
    const fr: [number, string][] = [[0, "transform:scaleX(0)"]];
    landings.forEach((w) => {
      const a = delivered(w) + 2;
      fr.push(
        [a, "transform:scaleX(0)"],
        [w.landed, "transform:scaleX(1)"],
        [w.landed + 0.8, "transform:scaleX(0)"],
      );
    });
    fr.push([100, "transform:scaleX(0)"]);
    d.kf("st", fr);
    add(
      `<rect class="bar ${d.anim("st")}" x="${x0}" y="${testY + 8}" width="${bx1 - x0}" height="5" rx="2.5" style="fill:var(--dg-ok)"/>`,
    );
    // main, each landing merged into it in turn.
    if (n) {
      const my = S.y + 84;
      add(`<text class="m mono xxs" x="${x0}" y="${my - 14}">main</text>`);
      add(`<line class="edge" x1="${x0 + 30}" y1="${my}" x2="${S.x + S.w - 12}" y2="${my}"/>`);
      add(`<circle class="dot" cx="${x0 + 40}" cy="${my}" r="4"/>`);
      landings.forEach((w, j) => {
        const x = x0 + 76 + j * 70;
        add(`<g class="${d.show(`ml${j}`, w.landed, END)}">`);
        add(
          `<path d="M${x - 24},${my - 12} C${x - 12},${my - 12} ${x - 8},${my} ${x},${my}" style="fill:none;stroke:var(--dg-accent);stroke-width:2"/>`,
        );
        add(
          `<circle class="merge" cx="${x}" cy="${my}" r="6"/><circle class="mergein" cx="${x}" cy="${my}" r="2.5"/>`,
        );
        add(
          `<text class="t mono xxs" x="${x}" y="${my + 18}" text-anchor="middle">${esc(w.name)}</text>`,
        );
        add("</g>");
      });
    } else {
      const mx = x0 + 10;
      add(`<text class="m mono xxs" x="${x0}" y="${S.y + 120}">main</text>`);
      add(`<line class="edge" x1="${mx}" y1="${S.y + 130}" x2="${mx}" y2="${S.y + S.h - 14}"/>`);
      add(`<circle class="dot" cx="${mx}" cy="${S.y + 144}" r="4"/>`);
      landings.forEach((w, j) => {
        const y = S.y + 184 + j * 40;
        add(`<g class="${d.show(`ml${j}`, w.landed, END)}">`);
        add(
          `<path d="M${mx + 26},${y - 20} C${mx + 26},${y - 8} ${mx + 12},${y} ${mx},${y}" style="fill:none;stroke:var(--dg-accent);stroke-width:2"/>`,
        );
        add(
          `<circle class="merge" cx="${mx}" cy="${y}" r="6"/><circle class="mergein" cx="${mx}" cy="${y}" r="2.5"/>`,
        );
        add(`<text class="t mono xxs" x="${mx + 16}" y="${y + 4}">${esc(w.name)}</text>`);
        add("</g>");
      });
    }
  }

  // ---- captions: what each column shows.
  const caps: string[][] = n
    ? [
        ["A launch names its kind, its points and its stream."],
        ["It waits by priority for its stream’s room."],
        ["Its place runs each stage with a fresh agent."],
        ["sluice lands it, and flume tidies the place."],
      ]
    : [
        ["A launch names its", "kind, its points and", "its stream."],
        [
          "It waits its turn by priority until",
          "the host, its stream’s bound, points",
          "or pace, and its pool have room.",
        ],
        ["Its place runs each stage", "with a fresh agent; its last", "delivery submits it."],
        ["sluice lands it;", "flume tidies", "the place."],
      ];
  caps.forEach((lines, i) => {
    const x = n ? 12 : g.caps[i];
    const y = n ? g.capY + i * 18 : g.capY;
    const r = n ? 7 : 10;
    add(`<circle class="badge" cx="${x + r}" cy="${y - 4}" r="${r}"/>`);
    add(
      `<text class="badget" x="${x + r}" y="${y}" text-anchor="middle"${n ? ` style="font-size:10px"` : ""}>${i + 1}</text>`,
    );
    lines.forEach((l, k) => {
      add(`<text class="t ${n ? "xs" : "sm"}" x="${x + 2 * r + 8}" y="${y + k * 17}">${l}</text>`);
    });
  });
  {
    const foot = n
      ? [
          "A place’s points, and a stream’s points bound and",
          "pace, are decided and wait on their build.",
        ]
      : [
          "A place’s points, and a stream’s points bound and pace, are decided and wait on their build.",
        ];
    foot.forEach((t, k) => {
      add(`<text class="m xxs" x="${n ? 12 : 20}" y="${g.footY + k * 15}">${t}</text>`);
    });
  }

  return d.render({
    id: opts.id ?? (n ? "hn" : "hw"),
    width: g.W,
    height: g.H,
    title:
      "flume: every launch waits in its stream for room, runs in a place of its own, and lands",
    desc:
      "Five launches come in, each naming its kind, its points and its stream: the change csv-export, 8 points, and the fix parser-tabs, 2 points, in work; " +
      "the chore bump-deps, 5 points, in personal; the change sso-login, 8 points, in work; and the chore readme-tour, 5 points, in personal. " +
      "The host runs three agents at once. The stream work, priority 10, runs up to three agents and 13 points in flight, on the pool work, filled first to last; " +
      "the stream personal, priority 0, runs up to two agents at a pace of 8 points a day, on the pool personal. " +
      "csv-export, parser-tabs and bump-deps start at once, each in a place of its own, whose stages light in turn with a fresh agent each, on the account work-1, work-2 or personal. " +
      "sso-login waits for room in work’s queue, short of its points bound, until csv-export’s last stage delivers and its points leave flight; then it starts on work-1. " +
      "readme-tour waits in personal’s queue, short of its pace, even once personal runs no agent. " +
      "Each place’s last delivery submits it to the merge queue; sluice tests it as merged and lands it on main, and flume tidies the place. " +
      "A place’s points, and a stream’s points bound and pace, are decided and wait on their build.",
    still: 45,
    standalone: opts.standalone,
  });
}
