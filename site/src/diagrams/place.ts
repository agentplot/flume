// The start page's second diagram: one launch whole, in one 26 s loop.
// Three callers stand at the head of the flume: your session's /launch, an
// inbox's send and a release tool's fix key. The inbox sends item 7 as a
// change on main, and the launch runs down the flume into its place, a
// worktree under flume's data directory on change/csv-export cut from
// main, which flume makes ready.
// Each stage lights in turn with a fresh agent, its prompt confirmed by the
// agent's own hooks, and ends when its deliverable exists: the change
// written, every task ticked, the change archived. flume then submits the
// place; sluice tests the work as merged and lands it, and flume tidies the
// place.
//
// Timeline, percent of the loop:
//   2-16 launch · 16-22 ready · 22-62 stages · 62-80 landing · 84-96 tidy
import { Diagram, esc, type Layout } from "./kit";

const T = 26;
const PATH = "…/flume/places/3c1e0b9a72d4/change--csv-export";
const BRANCH = "change/csv-export";

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

// The three stages and the landing: each with its window, what it waits on, its deliverable
// and the agent's lines, each line with when it appears.
const STAGES: {
  name: string;
  a: number;
  b: number;
  waits: string;
  done: [string, string];
  log: [string, number, string][];
}[] = [
  {
    name: "construct",
    a: 22,
    b: 34,
    waits: "the change",
    done: ["the change", "written"],
    log: [
      ["wrote proposal, design, tasks", 26, "t"],
      ["committed openspec/changes/csv-export", 29.5, "t"],
      ["ended: the change exists", 33, "ok"],
    ],
  },
  {
    name: "code",
    a: 34,
    b: 54,
    waits: "every task",
    done: ["5 of 5 tasks", "ticked"],
    log: [
      ["ticking tasks.md as it goes", 37, "t"],
      ["committed the export and its test", 47, "t"],
      ["ended: every task ticked", 53, "ok"],
    ],
  },
  {
    name: "verify",
    a: 54,
    b: 62,
    waits: "the archive",
    done: ["the change", "archived"],
    log: [
      ["checked the work against the change", 56.5, "t"],
      ["archived the change with its report", 59, "t"],
      ["ended: the change archived", 61, "ok"],
    ],
  },
  {
    name: "landing",
    a: 62,
    b: 80,
    waits: "the merge queue",
    done: ["landed", "on main"],
    log: [
      ["flume: sluice submit main --no-wait", 64, "ac"],
      ["sluice tests main + the change", 70, "t"],
      ["landed on main", 80, "ok"],
    ],
  },
];
const TICKS = [38, 42, 45.5, 49.5, 52.5];

function geometry(layout: Layout) {
  if (layout === "wide") {
    return {
      W: 900,
      H: 500,
      n: false,
      door: (i: number): Box => ({ x: 20, y: 72 + i * 68, w: 170, h: 58 }),
      // The feeders meet at the flume's head; the flume runs down to the place.
      head: { x: 226, y: 150 } as Pt,
      foot: { x: 318, y: 186 } as Pt,
      legs: [252, 296],
      ground: 272,
      card: { x: 20, y: 290, w: 170, h: 114 } as Box,
      place: { x: 334, y: 72, w: 370, h: 332 } as Box,
      pathAt: { x: 346, y: 96 } as Pt,
      readyY: 136,
      node: (i: number): Box => ({ x: 346 + i * 88, y: 152, w: 82, h: 32 }),
      below: (i: number): Pt => ({ x: 346 + i * 88, y: 206 }),
      pane: { x: 346, y: 256, w: 346, h: 136 } as Box,
      sluice: { x: 720, y: 72, w: 160, h: 164 } as Box,
      tidy: { x: 720, y: 268, w: 160, h: 136 } as Box,
      caps: (i: number) => ({ x: 20 + i * 176, y: 446 }),
    };
  }
  return {
    W: 360,
    H: 1066,
    n: true,
    door: (i: number): Box => ({ x: 8 + i * 116, y: 86, w: 112, h: 54 }),
    head: { x: 64, y: 248 } as Pt,
    foot: { x: 292, y: 298 } as Pt,
    legs: [136, 224],
    ground: 336,
    card: { x: 8, y: 154, w: 344, h: 70 } as Box,
    place: { x: 8, y: 372, w: 344, h: 398 } as Box,
    pathAt: { x: 20, y: 396 } as Pt,
    readyY: 436,
    node: (i: number): Box => ({ x: 20, y: 450 + i * 44, w: 112, h: 36 }),
    below: (i: number): Pt => ({ x: 146, y: 464 + i * 44 }),
    pane: { x: 20, y: 628, w: 320, h: 130 } as Box,
    sluice: { x: 8, y: 806, w: 344, h: 62 } as Box,
    tidy: { x: 8, y: 900, w: 344, h: 58 } as Box,
    caps: (i: number) => ({ x: 8, y: 990 + i * 17 }),
  };
}

export function place(layout: Layout) {
  const g = geometry(layout);
  const n = g.n;
  const d = new Diagram(T);
  const add = (s: string) => d.add(s);

  add(`<rect class="bg" width="${g.W}" height="${g.H}" rx="10"/>`);
  if (n) {
    add(`<text class="h" x="12" y="30">flume</text>`);
    add(`<text class="m xs" x="12" y="50">one launch, one place, a fresh agent per stage</text>`);
    add(`<text class="hd" x="12" y="76">LAUNCHED FROM</text>`);
  } else {
    add(`<text class="h" x="20" y="32">flume</text>`);
    add(
      `<text class="m sm" x="80" y="32">one launch, one place, a fresh agent for each stage</text>`,
    );
    add(`<text class="hd" x="20" y="62">LAUNCHED FROM</text>`);
    add(`<text class="hd" x="${g.place.x}" y="62">THE PLACE</text>`);
    add(`<text class="hd" x="${g.sluice.x}" y="62">SLUICE</text>`);
  }
  if (!n) add(`<text class="hd" x="${g.tidy.x}" y="${g.tidy.y - 10}">TIDY</text>`);

  // ---- the callers: your session, an inbox, a release tool; the inbox sends.
  const doors: [string, string, string, string][] = n
    ? [
        ["your session", "t", "/launch", "mono"],
        ["an inbox", "pd", "send", ""],
        ["a release tool", "hw", "fix key", ""],
      ]
    : [
        ["your session", "t", "/launch <brief>", "mono"],
        ["an inbox", "pd", "sends item 7 as a change", ""],
        ["a release tool", "hw", "its fix key on a release", ""],
      ];
  doors.forEach(([who, cls, what, mono], i) => {
    const D = g.door(i);
    add(`<rect class="box2" x="${D.x}" y="${D.y}" width="${D.w}" height="${D.h}" rx="6"/>`);
    add(`<text class="${cls} b ${n ? "xs" : "sm"}" x="${D.x + 10}" y="${D.y + 23}">${who}</text>`);
    add(
      `<text class="m ${mono} ${n ? "xxs" : "xs"}" x="${D.x + 10}" y="${D.y + 43}">${esc(what)}</text>`,
    );
  });
  {
    const D = g.door(1);
    d.kf("d1", [
      [0, "opacity:0"],
      [1.5, "opacity:0"],
      [2.3, "opacity:1"],
      [15, "opacity:1"],
      [16, "opacity:0"],
      [100, "opacity:0"],
    ]);
    add(
      `<rect class="${d.anim("d1")}" x="${D.x}" y="${D.y}" width="${D.w}" height="${D.h}" rx="6" style="fill:none;stroke:var(--dg-pond);stroke-width:2"/>`,
    );
  }

  // ---- the flume: three feeders meet at its head; its water runs down
  // to the place on a trestle.
  const H = g.head;
  const F = g.foot;
  // On a phone each caller writes the launch above the flume; on a wide
  // screen the three meet at its head.
  const feeders = [0, 1, 2].map((i) => {
    const D = g.door(i);
    if (n) return `M${D.x + D.w / 2},${D.y + D.h} L${D.x + D.w / 2},${g.card.y}`;
    const s = { x: D.x + D.w, y: D.y + D.h / 2 };
    return `M${s.x},${s.y} C${s.x + 22},${s.y} ${H.x - 22},${H.y} ${H.x},${H.y}`;
  });
  feeders.forEach((p) => {
    add(`<path class="edge" d="${p}" style="stroke-width:1.5;stroke:var(--dg-rule)"/>`);
  });
  add(
    `<path class="${d.show("feed", 2, 15)}" d="${feeders[1]}" style="fill:none;stroke:var(--dg-pond);stroke-width:2"/>`,
  );
  if (n)
    add(
      `<path class="edge" d="M${H.x},${g.card.y + g.card.h} L${H.x},${H.y}" style="stroke-width:1.5;stroke:var(--dg-rule)"/>`,
    );
  {
    const slope = (F.y - H.y) / (F.x - H.x);
    g.legs.forEach((x) => {
      const y = H.y + (x - H.x) * slope + 9;
      add(
        `<line x1="${x}" y1="${y}" x2="${x}" y2="${g.ground}" style="stroke:var(--dg-rule);stroke-width:5;stroke-linecap:round"/>`,
      );
    });
    const [l0, l1] = g.legs;
    const y0 = H.y + (l0 - H.x) * slope + 34;
    add(
      `<line x1="${l0}" y1="${y0}" x2="${l1}" y2="${g.ground - 14}" style="stroke:var(--dg-rule);stroke-width:3;stroke-linecap:round"/>`,
    );
    // The trough, then its water, moving while the flume carries a launch.
    add(
      `<path d="M${H.x},${H.y} L${F.x},${F.y}" style="stroke:var(--dg-rule);stroke-width:22;stroke-linecap:round;fill:none"/>`,
    );
    add(
      `<path d="M${H.x},${H.y} L${F.x},${F.y}" style="stroke:var(--dg-accent-soft);stroke-width:16;stroke-linecap:round;fill:none"/>`,
    );
    d.rule(
      ".water{stroke:var(--dg-accent);stroke-width:4;stroke-linecap:round;fill:none;stroke-dasharray:7 9}",
    );
    add(`<path class="water ${d.flow("water", 16, 34)}" d="M${H.x},${H.y} L${F.x},${F.y}"/>`);
    // The fall from the flume's foot into the place.
    if (n) add(`<path class="dash" d="M${F.x},${F.y + 12} L${F.x},${g.place.y - 4}"/>`);
    else
      add(
        `<path class="dash" d="M${F.x + 6},${F.y + 4} C${F.x + 14},${F.y + 8} ${g.place.x - 6},${F.y + 10} ${g.place.x},${F.y + 10}"/>`,
      );
  }

  // The launch, written out as the inbox hands it to flume.
  {
    const C = g.card;
    add(
      `<rect class="box2" x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="6" stroke-dasharray="4 3"/>`,
    );
    add(
      `<text class="m xs ${d.show("cw", 0, 3)}" x="${C.x + C.w / 2}" y="${C.y + C.h / 2 + 4}" text-anchor="middle">no launch yet</text>`,
    );
    add(`<g class="${d.show("cd", 3, 96)}">`);
    add(`<rect class="accbox" x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="6"/>`);
    add(`<text class="ac mono xs b" x="${C.x + 10}" y="${C.y + 20}">flume launch</text>`);
    add("</g>");
    const fields: [string, string, number][] = [
      ["repository", "ledger", 3.5],
      ["kind", "change", 5],
      ["name", "csv-export", 6.5],
      ["base", "main", 7.5],
      ["brief", n ? "item 7, export as CSV" : "export as CSV", 8.5],
    ];
    // On a phone: the first three on one row, the base and brief under them.
    const row = [
      [10, 72],
      [128, 156],
      [208, 238],
      [10, 40],
      [128, 160],
    ];
    fields.forEach(([k, v, at], i) => {
      const x = n ? C.x + row[i][0] : C.x + 10;
      const y = n ? C.y + 40 + (i >= 3 ? 18 : 0) : C.y + 38 + i * 17;
      const vx = n ? C.x + row[i][1] : x + 64;
      add(
        `<g class="${d.show(`fd${i}`, at, 96)}"><text class="m xxs" x="${x}" y="${y}">${k}</text>`,
      );
      add(`<text class="t mono xxs b" x="${vx}" y="${y}">${esc(v)}</text></g>`);
    });
  }

  // The launch runs down the flume: from the door to the head, down the
  // water, and into the place.
  {
    const D = g.door(1);
    const start = n ? { x: H.x, y: g.card.y + g.card.h } : { x: D.x + D.w, y: D.y + D.h / 2 };
    const pts: Pt[] = [
      start,
      H,
      F,
      n ? { x: F.x, y: g.place.y + 10 } : { x: g.place.x + 14, y: F.y + 10 },
    ];
    const end = pts[pts.length - 1];
    d.route("ln", pts, 9, 14.5, 15.5);
    add(
      `<g class="ln"><rect class="chip" x="${end.x - 42}" y="${end.y - 10}" width="84" height="20" rx="10"/><text class="chipt mono xxs" x="${end.x}" y="${end.y + 4}" text-anchor="middle">csv-export</text></g>`,
    );
  }

  // ---- the place
  const P = g.place;
  add(`<rect class="panel" x="${P.x}" y="${P.y}" width="${P.w}" height="${P.h}" rx="8"/>`);
  d.kf("pring", [
    [0, "opacity:0"],
    [14.5, "opacity:0"],
    [15.3, "opacity:1"],
    [84, "opacity:1"],
    [85, "opacity:0"],
    [100, "opacity:0"],
  ]);
  add(
    `<rect class="pring ${d.anim("pring")}" x="${P.x}" y="${P.y}" width="${P.w}" height="${P.h}" rx="8"/>`,
  );
  d.rule(".pring{fill:none;stroke:var(--dg-accent);stroke-width:1.5}");
  if (n) add(`<text class="hd" x="${P.x + 12}" y="${P.y - 8}">THE PLACE</text>`);
  add(
    `<text class="m xs ${d.show("pw", 0, 14.4)}" x="${P.x + P.w / 2}" y="${P.y + P.h / 2}" text-anchor="middle">no place yet</text>`,
  );
  // Everything inside fades when the place is tidied.
  add(
    `<g class="${d.levels("inside", [
      [0, 0],
      [15.8, 1],
      [85.5, 0],
    ])}">`,
  );
  add(`<text class="t mono xs b" x="${g.pathAt.x}" y="${g.pathAt.y}">${PATH}</text>`);
  add(`<text class="m xxs" x="${g.pathAt.x}" y="${g.pathAt.y + 18}">on</text>`);
  add(`<text class="ac mono xxs b" x="${g.pathAt.x + 17}" y="${g.pathAt.y + 18}">${BRANCH}</text>`);
  add(`<text class="m xxs" x="${g.pathAt.x + 132}" y="${g.pathAt.y + 18}">cut from main</text>`);
  {
    const y = g.readyY;
    const x = g.pathAt.x;
    add(`<text class="m xxs" x="${x}" y="${y}">made ready by the merge queue’s policy</text>`);
    const rx = n ? x : x + 234;
    const ry = n ? y + 0 : y;
    if (!n) {
      add(
        `<g class="${d.show("rd0", 16.5, 96)}"><circle class="ok" cx="${rx + 4}" cy="${ry - 4}" r="4"/><text class="ok xxs b" x="${rx + 12}" y="${ry}">toolchain</text></g>`,
      );
      add(
        `<g class="${d.show("rd1", 19, 96)}"><circle class="ok" cx="${rx + 72}" cy="${ry - 4}" r="4"/><text class="ok xxs b" x="${rx + 80}" y="${ry}">setup</text></g>`,
      );
    } else {
      add(
        `<g class="${d.show("rd0", 16.5, 96)}"><circle class="ok" cx="${x + 230}" cy="${y - 4}" r="4"/><text class="ok xxs b" x="${x + 238}" y="${y}">ready</text></g>`,
      );
    }
  }

  // The stages, each lit while its agent works and ticked when its
  // deliverable exists.
  STAGES.forEach((s, i) => {
    const N = g.node(i);
    add(`<rect class="box2" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="6"/>`);
    add(
      `<g class="${d.show(`nw${i}`, s.a, s.b)}"><rect class="accbox" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="6"/></g>`,
    );
    add(
      `<g class="${d.show(`nd${i}`, s.b, 96)}"><rect class="okbox" x="${N.x}" y="${N.y}" width="${N.w}" height="${N.h}" rx="6"/></g>`,
    );
    add(`<text class="t b xs" x="${N.x + 10}" y="${N.y + N.h / 2 + 4}">${s.name}</text>`);
    if (n)
      add(
        `<text class="ac xxs ${d.show(`na${i}`, s.a, s.b)}" x="${N.x + N.w - 8}" y="${N.y + N.h / 2 + 4}" text-anchor="end">working</text>`,
      );
    {
      const cx = N.x + N.w - 3;
      const cy = N.y + 3;
      add(
        `<g class="${d.show(`nk${i}`, s.b, 96)}"><circle class="ok" cx="${cx}" cy="${cy}" r="7.5"/><path d="M${cx - 3.5},${cy} l2.5,2.5 l4.5,-5" style="fill:none;stroke:var(--dg-ink);stroke-width:1.8;stroke-linecap:round;stroke-linejoin:round"/></g>`,
      );
    }
    if (!n && i < STAGES.length - 1)
      add(
        `<path class="edge" d="M${N.x + N.w + 1},${N.y + N.h / 2} l6,0" style="stroke-width:1.5"/>`,
      );
    if (n && i < STAGES.length - 1)
      add(`<path class="edge" d="M${N.x + 20},${N.y + N.h + 1} l0,6" style="stroke-width:1.5"/>`);
    // Under the node: what it waits on, then its deliverable.
    const B = g.below(i);
    const l2 = n ? 0 : 15;
    add(
      n
        ? `<text class="m xxs ${d.show(`nwt${i}`, s.a, s.b)}" x="${B.x}" y="${B.y}">waits on <tspan class="ac b">${s.waits}</tspan></text>`
        : `<g class="${d.show(`nwt${i}`, s.a, s.b)}"><text class="m xxs" x="${B.x}" y="${B.y}">waits on</text><text class="ac xxs b" x="${B.x}" y="${B.y + l2}">${s.waits}</text></g>`,
    );
    if (s.name === "code") {
      // The tasks, ticked one by one as the agent works through them.
      const bx = n ? B.x + 124 : B.x;
      const by = n ? B.y - 9 : B.y + 22;
      TICKS.forEach((at, k) => {
        const x = bx + k * (n ? 14 : 15);
        add(
          `<rect x="${x}" y="${by}" width="11" height="11" rx="2" class="${d.show(`tb${k}`, s.a, 96)}" style="fill:none;stroke:var(--dg-muted);stroke-width:1.2"/>`,
        );
        add(
          `<g class="${d.show(`tk${k}`, at, 96)}"><rect class="ac" x="${x}" y="${by}" width="11" height="11" rx="2"/><path d="M${x + 2.5},${by + 5.5} l2.5,2.5 l4,-5" style="fill:none;stroke:var(--dg-ink);stroke-width:1.6;stroke-linecap:round;stroke-linejoin:round"/></g>`,
        );
      });
    }
    add(
      n
        ? `<text class="ok xxs b ${d.show(`ndl${i}`, s.b, 96)}" x="${B.x}" y="${B.y}">${s.done[0]} ${s.done[1]}</text>`
        : `<g class="${d.show(`ndl${i}`, s.b, 96)}"><text class="ok xxs b" x="${B.x}" y="${B.y}">${s.done[0]}</text><text class="ok xxs" x="${B.x}" y="${B.y + l2}">${s.done[1]}</text></g>`,
    );
  });

  // The agent's pane: a fresh agent for each stage, its prompt confirmed in
  // its transcript before flume counts the stage as started.
  const S = g.pane;
  add(`<rect class="box2" x="${S.x}" y="${S.y}" width="${S.w}" height="${S.h}" rx="6"/>`);
  add(
    `<text class="m xs ${d.show("pn0", 0, 22)}" x="${S.x + S.w / 2}" y="${S.y + S.h / 2 + 4}" text-anchor="middle">a herdr pane, for each stage’s agent</text>`,
  );
  const logY = (k: number) => S.y + 74 + k * 20;
  STAGES.forEach((s, i) => {
    const until = i === STAGES.length - 1 ? 84 : s.b;
    // The pane flashes as the old agent ends and a fresh one starts.
    add(
      `<rect class="${d.show(`pf${i}`, s.a, s.a + 1.5)}" x="${S.x}" y="${S.y}" width="${S.w}" height="${S.h}" rx="6" style="fill:none;stroke:var(--dg-accent);stroke-width:1.5"/>`,
    );
    add(`<g class="${d.show(`pt${i}`, s.a, until)}">`);
    const landing = i === STAGES.length - 1;
    add(
      `<text class="t b xs" x="${S.x + 10}" y="${S.y + 20}">${landing ? "flume · the landing" : `agent ${i + 1} · ${s.name}`}</text>`,
    );
    add(
      `<text class="m xxs" x="${S.x + S.w - 10}" y="${S.y + 20}" text-anchor="end">${landing ? "no agent" : "a fresh agent"}</text>`,
    );
    add(
      `<line x1="${S.x}" y1="${S.y + 30}" x2="${S.x + S.w}" y2="${S.y + 30}" style="stroke:var(--dg-rule)"/>`,
    );
    add("</g>");
    // The prompt: sent once the agent says it started, then acknowledged
    // by its own hook. The landing has no agent and no prompt.
    if (!landing) {
      add(
        `<text class="m mono xxs ${d.show(`pp${i}`, s.a + 0.4, s.a + 1.6)}" x="${S.x + 10}" y="${S.y + 48}">prompt sent: ${s.name}…</text>`,
      );
      add(
        `<g class="${d.show(`pc${i}`, s.a + 1.6, until)}"><circle class="ok" cx="${S.x + 14}" cy="${S.y + 44}" r="4"/><text class="ok xxs b" x="${S.x + 24}" y="${S.y + 48}">prompt received</text></g>`,
      );
    }
    s.log.forEach(([line, at, cls], k) => {
      add(
        `<text class="mono xxs ${cls} ${d.show(`pl${i}${k}`, at, until)}" x="${S.x + 10}" y="${logY(k)}">${esc(line)}</text>`,
      );
    });
  });

  add("</g>"); // the place's insides

  // In the place, once tidied: only its outline, dashed.
  add(`<g class="${d.show("gone", 85, 96)}">`);
  add(
    `<rect x="${P.x}" y="${P.y}" width="${P.w}" height="${P.h}" rx="8" style="fill:none;stroke:var(--dg-muted);stroke-width:1.2;stroke-dasharray:5 4"/>`,
  );
  add(
    `<text class="m mono xs" x="${P.x + P.w / 2}" y="${P.y + P.h / 2 - 8}" text-anchor="middle">${PATH}</text>`,
  );
  add(
    `<text class="m sm" x="${P.x + P.w / 2}" y="${P.y + P.h / 2 + 14}" text-anchor="middle">removed</text>`,
  );
  add("</g>");

  // ---- sluice: the submission is tested as merged and lands on main.
  const L = g.sluice;
  {
    // The submission travels from the landing to sluice.
    const N = g.node(3);
    const from = n ? { x: N.x + N.w, y: N.y + N.h / 2 } : { x: N.x + N.w / 2, y: N.y };
    const to = n ? { x: L.x + 30, y: L.y + 14 } : { x: L.x + 30, y: L.y + 40 };
    const pts: Pt[] = n
      ? [
          from,
          { x: P.x + P.w - 6, y: from.y },
          { x: P.x + P.w - 6, y: P.y + P.h + 16 },
          { x: to.x, y: P.y + P.h + 16 },
          to,
        ]
      : [
          from,
          { x: from.x, y: P.y - 4 },
          { x: P.x + P.w + 8, y: P.y - 4 },
          { x: P.x + P.w + 8, y: to.y },
          to,
        ];
    add(
      `<polyline class="dash ${d.show("sway", 67.5, 71.5)}" points="${pts.map((p) => `${p.x},${p.y}`).join(" ")}"/>`,
    );
    d.route("sub", pts, 67.5, 71, 72);
    add(`<circle class="chip sub" cx="${to.x}" cy="${to.y}" r="5"/>`);
  }
  add(`<rect class="box2" x="${L.x}" y="${L.y}" width="${L.w}" height="${L.h}" rx="8"/>`);
  if (n) add(`<text class="hd" x="${L.x + 4}" y="${L.y - 10}">SLUICE</text>`);
  {
    const x0 = L.x + 12;
    const x1 = L.x + L.w - 12;
    const sx = n ? x0 + 56 : x0;
    const sy = n ? L.y + 24 : L.y + 46;
    const my = n ? L.y + 44 : L.y + 124;
    add(`<text class="ok b sm" x="${x0}" y="${n ? L.y + 24 : L.y + 26}">sluice</text>`);
    add(
      `<text class="m xxs ${d.show("sl0", 0, 72)}" x="${sx}" y="${sy}">tests a change as merged</text>`,
    );
    add(
      `<text class="ac xxs b ${d.show("sl1", 72, 80)}" x="${sx}" y="${sy}">testing main + csv-export</text>`,
    );
    add(
      `<text class="ok xxs b ${d.show("sl2", 80, 96)}" x="${sx}" y="${sy}">landed on main</text>`,
    );
    if (!n) {
      // The test's progress.
      add(`<rect class="track" x="${x0}" y="${L.y + 58}" width="${x1 - x0}" height="6" rx="3"/>`);
      d.kf("stest", [
        [0, "transform:scaleX(0)"],
        [72, "transform:scaleX(0)"],
        [80, "transform:scaleX(1)"],
        [96, "transform:scaleX(1)"],
        [97, "transform:scaleX(0)"],
        [100, "transform:scaleX(0)"],
      ]);
      add(
        `<rect class="bar ${d.anim("stest")}" x="${x0}" y="${L.y + 58}" width="${x1 - x0}" height="6" rx="3" style="fill:var(--dg-ok)"/>`,
      );
      add(`<text class="m mono xxs" x="${x0}" y="${my - 26}">main</text>`);
    }
    // main, with the branch merging into it when it lands.
    const mx0 = n ? sx : x0;
    add(`<line x1="${mx0}" y1="${my}" x2="${x1}" y2="${my}" class="edge" style="stroke-width:2"/>`);
    [0.12, 0.38].forEach((f) => {
      add(`<circle class="dot" cx="${mx0 + (x1 - mx0) * f}" cy="${my}" r="4"/>`);
    });
    const mxm = mx0 + (x1 - mx0) * 0.72;
    add(
      `<path class="${d.show("mbr", 72, 96)}" d="M${mxm - 40},${my - (n ? 12 : 20)} C${mxm - 18},${my - (n ? 12 : 20)} ${mxm - 10},${my} ${mxm},${my}" style="fill:none;stroke:var(--dg-accent);stroke-width:2"/>`,
    );
    add(
      `<g class="${d.show("mrg", 80, 96)}"><circle class="merge" cx="${mxm}" cy="${my}" r="7"/><circle class="mergein" cx="${mxm}" cy="${my}" r="3"/></g>`,
    );
  }

  // ---- tidy: once the work has landed, flume removes the place it made,
  // and the branch, since it holds no commit main lacks.
  {
    const K = g.tidy;
    add(`<rect class="box2" x="${K.x}" y="${K.y}" width="${K.w}" height="${K.h}" rx="8"/>`);
    if (n) add(`<text class="hd" x="${K.x + 4}" y="${K.y - 10}">TIDY</text>`);
    const x0 = K.x + 12;
    const wait = n
      ? ["when the work lands or is dropped,", "flume removes the place it made"]
      : ["when the work lands", "or is dropped, flume", "removes the place it made"];
    wait.forEach((t, k) => {
      add(
        `<text class="m xxs ${d.show(`ty${k}`, 0, 85)}" x="${x0}" y="${K.y + 24 + k * 16}">${t}</text>`,
      );
    });
    const rows: [string, string, number][] = n
      ? [
          ["change/csv-export holds nothing main lacks", "m", 85],
          ["place removed · branch removed", "ok", 87.5],
        ]
      : [
          ["change/csv-export", "mono t", 85],
          ["holds nothing main lacks", "m", 85.5],
          ["place removed", "ok", 87.5],
          ["branch removed", "ok", 89.5],
        ];
    rows.forEach(([t, cls, at], k) => {
      const y = K.y + 24 + k * (n ? 18 : 20) + (!n && k >= 2 ? 12 : 0);
      const tick = cls === "ok";
      add(`<g class="${d.show(`tr${k}`, at, 96)}">`);
      if (tick) add(`<circle class="ok" cx="${x0 + 4}" cy="${y - 4}" r="4"/>`);
      add(
        `<text class="${cls} xxs${tick ? " b" : ""}" x="${tick ? x0 + 13 : x0}" y="${y}">${esc(t)}</text></g>`,
      );
    });
  }

  // ---- captions
  const caps: [string, number, number][] = n
    ? [
        ["A launch: repository, kind, brief, name, base.", 2, 16],
        ["flume makes the place, and makes it ready.", 16, 22],
        ["A fresh agent per stage, ended by its deliverable.", 22, 68],
        ["flume submits; sluice lands the work.", 68, 84],
        ["flume tidies the place it made.", 84, 96],
      ]
    : [
        ["A launch: a repository,|a kind, brief, name, base.", 2, 16],
        ["flume makes the place|and makes it ready.", 16, 22],
        ["A fresh agent per stage,|ended by its deliverable.", 22, 68],
        ["flume submits;|sluice lands the work.", 68, 84],
        ["flume tidies the place|it made.", 84, 96],
      ];
  caps.forEach(([txt, a, b], i) => {
    const c = g.caps(i);
    const cls = d.keep(d.show(`cap${i}`, a, b, 0.35));
    const lines = txt.split("|");
    if (n) {
      add(`<g class="${cls}"><circle class="badge" cx="${c.x + 8}" cy="${c.y - 4}" r="7"/>`);
      add(
        `<text class="badget" x="${c.x + 8}" y="${c.y}" text-anchor="middle" style="font-size:10px">${i + 1}</text>`,
      );
      add(`<text class="t xs" x="${c.x + 22}" y="${c.y}">${lines[0]}</text></g>`);
      return;
    }
    add(`<g class="${cls}"><circle class="badge" cx="${c.x + 11}" cy="${c.y - 4}" r="10"/>`);
    add(`<text class="badget" x="${c.x + 11}" y="${c.y}" text-anchor="middle">${i + 1}</text>`);
    lines.forEach((l, k) => {
      add(`<text class="t sm" x="${c.x + 28}" y="${c.y + k * 17}">${l}</text>`);
    });
    add("</g>");
  });

  return d.render({
    id: n ? "pn" : "pw",
    width: g.W,
    height: g.H,
    title: "flume: one launch, one place, a fresh agent for each stage",
    desc:
      "Three callers stand at the head of the flume: your session’s /launch, an inbox’s send and a release tool’s fix key. The inbox sends item 7 as a change: " +
      "the launch names the repository ledger, the kind change, the name csv-export, the base main and the brief, export the ledger as CSV. It runs down the flume into its place, " +
      "a worktree under flume’s data directory, change--csv-export, on the branch change/csv-export, cut from main, which flume makes ready by the merge queue’s policy. " +
      "Three stages light in turn, each with a fresh agent in a herdr pane whose prompt the agent’s own hook acknowledges: construct ends when the OpenSpec change is written, " +
      "code when all five of its tasks are ticked, and verify when the change is archived with its report. flume then submits the branch to main, with no agent; sluice tests main with the change as merged and lands it. " +
      "flume then tidies the place: the worktree is removed, and the branch with it, since it holds nothing main lacks.",
    still: 82,
  });
}
