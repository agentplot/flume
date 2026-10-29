// How it works: the three kinds as tracks of stages, side by side, in one
// 24 s loop. Each stage waits on its deliverable and lights the next only
// once it exists, each with a fresh agent: a fix's code waits on a new
// commit, a change's construct on the OpenSpec change written, its code on
// every task ticked (they tick one by one), its verify on the report, and
// each kind's submit stage on the landing; a question's one stage is
// sounding's session, which waits on you once and ends on its conclusion.
// Each track runs at its own pace, and a landed place is tidied.
//
// Timeline, percent of the loop:
//   fix 3-40, tidied 46 · change 3-80, tidied 86 · question 3-76
import { Diagram, esc, type Layout } from "./kit";

const T = 24;

interface Stage {
  name: string;
  a: number;
  b: number;
  waits: string;
  done: string;
  ticks?: number[];
  // A stretch inside [a, b] where the agent waits on you, by sounding ask.
  ask?: [number, number];
}
interface Kind {
  kind: string;
  branch: string;
  from: string;
  cls: string;
  stages: Stage[];
  end: [string, number];
}

const KINDS: Kind[] = [
  {
    kind: "fix",
    branch: "fix/bump-mise",
    from: "headwater’s fix key",
    cls: "hw",
    stages: [
      { name: "code", a: 3, b: 30, waits: "a new commit", done: "committed: bump mise" },
      { name: "submit", a: 30, b: 40, waits: "the landing", done: "landed on main" },
    ],
    end: ["tidied: place and branch", 46],
  },
  {
    kind: "change",
    branch: "change/csv-export",
    from: "millpond’s send",
    cls: "pd",
    stages: [
      { name: "construct", a: 3, b: 20, waits: "the OpenSpec change", done: "the change, written" },
      {
        name: "code",
        a: 20,
        b: 56,
        waits: "every task ticked",
        done: "6 of 6 tasks ticked",
        ticks: [25, 31, 37, 43, 49, 55],
      },
      { name: "verify", a: 56, b: 70, waits: "its report", done: "report: 0 to fix" },
      { name: "submit", a: 70, b: 80, waits: "the landing", done: "landed, the change archived" },
    ],
    end: ["tidied: place and branch", 86],
  },
  {
    kind: "question",
    branch: "sounding/parquet",
    from: "sounding",
    cls: "sd",
    stages: [
      {
        name: "sounding’s session",
        a: 3,
        b: 76,
        waits: "its conclusion",
        done: "conclusion.md written",
        ask: [28, 40],
      },
    ],
    end: ["ended on sounding conclude", 76],
  },
];

interface Box {
  x: number;
  y: number;
  w: number;
  h: number;
}

function geometry(layout: Layout) {
  if (layout === "wide") {
    return {
      W: 720,
      H: 468,
      n: false,
      col: (k: number) => ({ x: 16 + k * 236, y: 20, w: 216 }),
      head: 58, // the track's header, above its first stage
      nodeH: 62,
      gap: 14,
      foot: 448,
    };
  }
  return {
    W: 360,
    H: 870,
    n: true,
    col: (k: number) => ({ x: 8, y: [16, 244, 618][k], w: 344 }),
    head: 50,
    nodeH: 54,
    gap: 10,
    foot: 834,
  };
}

export function kinds(layout: Layout) {
  const g = geometry(layout);
  const n = g.n;
  const d = new Diagram(T);
  const add = (s: string) => d.add(s);

  add(`<rect class="bg" width="${g.W}" height="${g.H}" rx="10"/>`);

  KINDS.forEach((K, k) => {
    const c = g.col(k);
    const id = `k${k}`;
    // The track's header: the kind, its branch and who launches it here.
    add(`<text class="t h" x="${c.x}" y="${c.y + 18}">${K.kind}</text>`);
    add(
      `<text class="ac mono xxs b" x="${c.x + c.w}" y="${c.y + 18}" text-anchor="end">${esc(K.branch)}</text>`,
    );
    add(
      `<text class="m xxs" x="${c.x}" y="${c.y + 38}">launched here by <tspan class="${K.cls} b">${esc(K.from)}</tspan></text>`,
    );
    // A stage with tasks is taller, for the row of them.
    const nodeH = (s: Stage) => g.nodeH + (s.ticks ? 20 : 0);
    const nodeY = (i: number) =>
      c.y + g.head + K.stages.slice(0, i).reduce((y, s) => y + nodeH(s) + g.gap, 0);

    K.stages.forEach((s, i) => {
      const N: Box = { x: c.x, y: nodeY(i), w: c.w, h: nodeH(s) };
      const sid = `${id}s${i}`;
      // waiting, then working with a fresh agent, then done.
      add(`<rect class="box2" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="7"/>`);
      add(
        `<rect class="accbox ${d.show(`${sid}w`, s.a, s.b)}" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="7"/>`,
      );
      if (s.ask)
        add(
          `<rect class="warnbox ${d.show(`${sid}q`, s.ask[0], s.ask[1])}" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="7"/>`,
        );
      add(
        `<rect class="okbox ${d.show(`${sid}d`, s.b, 96)}" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="7"/>`,
      );
      add(`<text class="t b sm" x="${N.x + 12}" y="${N.y + 22}">${esc(s.name)}</text>`);
      // Its state, at the right.
      const sx = N.x + N.w - 12;
      add(
        `<text class="m xxs ${d.show(`${sid}s0`, 0, s.a)}" x="${sx}" y="${N.y + 21}" text-anchor="end">not started</text>`,
      );
      if (s.ask) {
        add(
          `<text class="ac xxs b ${d.show(`${sid}s1`, s.a, s.ask[0])}" x="${sx}" y="${N.y + 21}" text-anchor="end">working</text>`,
        );
        add(
          `<text class="warn xxs b ${d.show(`${sid}s2`, s.ask[0], s.ask[1])}" x="${sx}" y="${N.y + 21}" text-anchor="end">waiting on you</text>`,
        );
        add(
          `<text class="ac xxs b ${d.show(`${sid}s3`, s.ask[1], s.b)}" x="${sx}" y="${N.y + 21}" text-anchor="end">working</text>`,
        );
      } else {
        add(
          `<text class="ac xxs b ${d.show(`${sid}s1`, s.a, s.b)}" x="${sx}" y="${N.y + 21}" text-anchor="end">agent ${i + 1} · working</text>`,
        );
      }
      {
        const cx = sx - 5;
        const cy = N.y + 17;
        add(
          `<g class="${d.show(`${sid}k`, s.b, 96)}"><circle class="ok" cx="${cx}" cy="${cy}" r="7.5"/><path d="M${cx - 3.5},${cy} l2.5,2.5 l4.5,-5" style="fill:none;stroke:var(--dg-ink);stroke-width:1.8;stroke-linecap:round;stroke-linejoin:round"/></g>`,
        );
      }
      // What it waits on, then the deliverable itself.
      const ly = N.y + g.nodeH - 14;
      add(
        `<text class="m xs ${d.show(`${sid}v`, 0, s.b)}" x="${N.x + 12}" y="${ly}">waits on <tspan class="t">${esc(s.waits)}</tspan></text>`,
      );
      add(
        `<text class="ok xs b ${d.show(`${sid}e`, s.b, 96)}" x="${N.x + 12}" y="${ly}">${esc(s.done)}</text>`,
      );
      if (s.ticks) {
        // The change's tasks, ticked as its agent works through them.
        const bx = N.x + 12;
        const by = ly + 8;
        s.ticks.forEach((at, t) => {
          const x = bx + t * 15;
          add(
            `<rect x="${x}" y="${by}" width="11" height="11" rx="2" style="fill:none;stroke:var(--dg-muted);stroke-width:1.2"/>`,
          );
          add(
            `<g class="${d.show(`${sid}t${t}`, at, 96)}"><rect class="ac" x="${x}" y="${by}" width="11" height="11" rx="2"/><path d="M${x + 2.5},${by + 5.5} l2.5,2.5 l4,-5" style="fill:none;stroke:var(--dg-ink);stroke-width:1.6;stroke-linecap:round;stroke-linejoin:round"/></g>`,
          );
        });
      }
      // The hand-on to the next stage, lit as the deliverable lands.
      const ax = N.x + 26;
      const ay = N.y + N.h;
      add(
        `<path class="edge" d="M${ax},${ay + 2} L${ax},${ay + g.gap - 2}" style="stroke-width:1.5;stroke:var(--dg-rule)"/>`,
      );
      add(
        `<path class="${d.show(`${sid}h`, s.b, 96)}" d="M${ax},${ay + 2} L${ax},${ay + g.gap - 2}" style="fill:none;stroke:var(--dg-ok);stroke-width:2"/>`,
      );
    });

    // Where the track ends.
    const ey = nodeY(K.stages.length);
    const [label, at] = K.end;
    add(
      `<rect x="${c.x}" y="${ey}" width="${c.w}" height="28" rx="14" style="fill:none;stroke:var(--dg-rule);stroke-width:1;stroke-dasharray:4 3"/>`,
    );
    add(
      `<text class="m xs ${d.show(`${id}pre`, 0, at)}" x="${c.x + c.w / 2}" y="${ey + 18}" text-anchor="middle">${K.kind === "question" ? "ends on sounding conclude" : "then flume tidies it"}</text>`,
    );
    if (K.kind === "question") {
      // Who owns what in a question.
      const ny = ey + 52;
      const lines = n
        ? [
            "flume makes the place and runs the session; sounding",
            "keeps the question and the conclusion you decide.",
          ]
        : [
            "flume makes the place and",
            "runs the session. sounding",
            "keeps the question, its",
            "rounds and the conclusion",
            "you decide.",
          ];
      lines.forEach((l, j) => {
        add(`<text class="m xs" x="${c.x + 4}" y="${ny + j * 18}">${l}</text>`);
      });
    }
    add(
      `<g class="${d.show(`${id}end`, at, 96)}"><rect class="okbox" x="${c.x}" y="${ey}" width="${c.w}" height="28" rx="14" style="stroke-dasharray:none"/><text class="ok xs b" x="${c.x + c.w / 2}" y="${ey + 18}" text-anchor="middle">${esc(label)}</text></g>`,
    );
  });

  // A repository's or a launch's own kinds, in the same form.
  {
    const y = g.foot;
    if (n) {
      add(
        `<text class="m xs" x="12" y="${y}">A repository or a launch may declare its own kinds,</text>`,
      );
      add(
        `<text class="m xs" x="12" y="${y + 18}">each stage with its prompt and its deliverable.</text>`,
      );
    } else {
      add(
        `<text class="m xs" x="16" y="${y}">A repository or a launch may declare kinds of its own: each stage names its prompt and its deliverable.</text>`,
      );
    }
  }

  return d.render({
    id: n ? "kn" : "kw",
    width: g.W,
    height: g.H,
    title: "Three kinds, each a track of stages that end on their deliverables",
    desc:
      "Three tracks side by side. A fix, on fix/bump-mise from headwater’s fix key: code waits on a new commit, then submit waits on the landing, and the place is tidied. " +
      "A change, on change/csv-export from millpond’s send: construct waits on the OpenSpec change written; code waits on every task ticked, and its six tasks tick one by one; " +
      "verify waits on its report; submit waits on the landing, with the change archived; the place is tidied. A question, on sounding/parquet: sounding’s session works, waits on you once, works again, " +
      "and ends when sounding conclude writes conclusion.md. Each stage starts a fresh agent and lights the next only once its deliverable exists. " +
      "A repository or a launch may declare kinds of its own in the same form.",
    still: 92,
  });
}
