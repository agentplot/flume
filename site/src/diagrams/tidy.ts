// How it works: tidying, in one 20 s loop. flume looks at three worktrees in
// turn. fix/bump-mise landed: its commits are on main, so flume removes the
// place and the branch. change/parquet was dropped holding two commits main
// lacks: flume removes the place and keeps the branch, named. A worktree you
// made by hand is not flume's, and flume leaves it alone.
//
// Timeline, percent of the loop:
//   6-28 the landed fix · 34-60 the dropped change · 66-84 the hand-made one
import { Diagram, esc, type Layout } from "./kit";

const T = 20;

interface Place {
  path: string;
  branch: string;
  outcome: [string, string];
  graph: "landed" | "dropped" | "hand";
  note: string;
  rows: [string, string][];
  a: number; // when flume reaches it
}

const PLACES: Place[] = [
  {
    path: "places/fix--bump-mise",
    branch: "fix/bump-mise",
    outcome: ["landed", "ok"],
    graph: "landed",
    note: "its commits are on main",
    rows: [
      ["place removed", "ok"],
      ["branch removed", "ok"],
    ],
    a: 6,
  },
  {
    path: "places/change--parquet",
    branch: "change/parquet",
    outcome: ["dropped", "warn"],
    graph: "dropped",
    note: "2 commits main lacks",
    rows: [
      ["place removed", "ok"],
      ["branch kept, and named", "warn"],
    ],
    a: 34,
  },
  {
    path: "spike",
    branch: "spike",
    outcome: ["made by hand", "m"],
    graph: "hand",
    note: "flume did not make it",
    rows: [["left alone", "m"]],
    a: 66,
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
      H: 318,
      n: false,
      card: (i: number): Box => ({ x: 16 + i * 236, y: 16, w: 216, h: 286 }),
    };
  }
  return {
    W: 360,
    H: 608,
    n: true,
    card: (i: number): Box => ({ x: 8, y: 8 + i * 200, w: 344, h: 192 }),
  };
}

export function tidy(layout: Layout) {
  const g = geometry(layout);
  const n = g.n;
  const d = new Diagram(T);
  const add = (s: string) => d.add(s);

  add(`<rect class="bg" width="${g.W}" height="${g.H}" rx="10"/>`);

  PLACES.forEach((p, i) => {
    const C = g.card(i);
    const id = `p${i}`;
    const a = p.a;
    add(`<rect class="box2" x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="8"/>`);
    // flume's look at this place.
    add(
      `<rect class="${d.show(`${id}r`, a, a + 20)}" x="${C.x}" y="${C.y}" width="${C.w}" height="${C.h}" rx="8" style="fill:none;stroke:var(--dg-accent);stroke-width:1.5"/>`,
    );
    const x0 = C.x + 14;
    add(`<text class="${p.outcome[1]} b sm" x="${x0}" y="${C.y + 24}">${p.outcome[0]}</text>`);
    add(`<text class="m mono xxs" x="${x0}" y="${C.y + 44}">ledger/${esc(p.path)}</text>`);

    // Its branch against main.
    const gx = x0;
    const gw = n ? 150 : C.w - 28;
    const my = C.y + (n ? 96 : 106);
    add(
      `<line class="edge" x1="${gx}" y1="${my}" x2="${gx + gw}" y2="${my}" style="stroke-width:2"/>`,
    );
    add(`<text class="m mono xxs" x="${gx + gw}" y="${my + 18}" text-anchor="end">main</text>`);
    [0.08, 0.3].forEach((f) => {
      add(`<circle class="dot" cx="${gx + gw * f}" cy="${my}" r="4"/>`);
    });
    const bx = gx + gw * 0.3;
    if (p.graph === "landed") {
      add(
        `<path d="M${bx},${my} C${bx + 14},${my - 24} ${bx + 20},${my - 24} ${bx + 34},${my - 24} L${bx + 66},${my - 24} C${bx + 80},${my - 24} ${bx + 86},${my} ${bx + 100},${my}" style="fill:none;stroke:var(--dg-accent);stroke-width:2"/>`,
      );
      [34, 64].forEach((dx) => {
        add(`<circle class="ac" cx="${bx + dx}" cy="${my - 24}" r="4"/>`);
      });
      add(
        `<circle class="merge" cx="${bx + 100}" cy="${my}" r="6"/><circle class="mergein" cx="${bx + 100}" cy="${my}" r="2.5"/>`,
      );
    } else if (p.graph === "dropped") {
      add(
        `<path d="M${bx},${my} C${bx + 14},${my - 24} ${bx + 20},${my - 24} ${bx + 34},${my - 24} L${bx + 70},${my - 24}" style="fill:none;stroke:var(--dg-accent);stroke-width:2"/>`,
      );
      [34, 64].forEach((dx) => {
        add(`<circle class="ac" cx="${bx + dx}" cy="${my - 24}" r="4.5"/>`);
      });
      add(`<circle class="dot" cx="${gx + gw * 0.62}" cy="${my}" r="4"/>`);
    } else {
      add(
        `<path d="M${bx},${my} C${bx + 14},${my - 24} ${bx + 20},${my - 24} ${bx + 34},${my - 24} L${bx + 50},${my - 24}" style="fill:none;stroke:var(--dg-muted);stroke-width:2;stroke-dasharray:4 3"/>`,
      );
      add(`<circle class="dot" cx="${bx + 40}" cy="${my - 24}" r="4"/>`);
    }
    add(
      `<text class="mono xxs ${p.graph === "hand" ? "m" : "ac"}" x="${bx + 2}" y="${my - 36}">${esc(p.branch)}</text>`,
    );

    // What flume reads, then what it does.
    const rx = n ? x0 + 172 : x0;
    const ry = n ? C.y + 66 : C.y + 154;
    add(
      `<text class="m xxs ${d.show(`${id}q`, 0, a + 4)}" x="${rx}" y="${ry}">${p.graph === "hand" ? "did flume make it?" : "what does it hold"}</text>`,
    );
    if (p.graph !== "hand")
      add(
        `<text class="m xxs ${d.show(`${id}q2`, 0, a + 4)}" x="${rx}" y="${ry + 15}">that main lacks?</text>`,
      );
    add(
      `<text class="t xs b ${d.show(`${id}n`, a + 4, 96)}" x="${rx}" y="${ry}">${esc(p.note)}</text>`,
    );
    if (p.graph !== "hand")
      add(
        `<text class="m mono xxs ${d.show(`${id}c`, a + 4, 96)}" x="${rx}" y="${ry + 16}">git cherry main</text>`,
      );

    // The worktree itself: its files, gone once removed.
    const wy = n ? C.y + 128 : C.y + 182;
    const ww = n ? 150 : C.w - 28;
    const removed = p.graph !== "hand";
    const tree =
      `<rect class="file" x="${gx}" y="${wy}" width="${ww}" height="${n ? 48 : 44}" rx="5"/>` +
      `<text class="m xxs" x="${gx + 10}" y="${wy + 17}">the worktree</text>` +
      [0, 1, 2]
        .map(
          (k) =>
            `<rect x="${gx + 10 + k * 34}" y="${wy + 26}" width="28" height="8" rx="2" style="fill:var(--dg-rule)"/>`,
        )
        .join("");
    if (removed) {
      add(
        `<g class="${d.levels(`${id}w`, [
          [0, 1],
          [a + 10, 0],
          [96, 1],
        ])}">${tree}</g>`,
      );
      add(
        `<g class="${d.show(`${id}x`, a + 10, 96)}"><rect x="${gx}" y="${wy}" width="${ww}" height="${n ? 48 : 44}" rx="5" style="fill:none;stroke:var(--dg-muted);stroke-width:1;stroke-dasharray:4 3"/><text class="m xxs" x="${gx + ww / 2}" y="${wy + 26}" text-anchor="middle">removed</text></g>`,
      );
    } else {
      add(tree);
    }

    // The outcome, row by row.
    p.rows.forEach(([t, cls], k) => {
      const at = a + 10 + k * 5;
      const y = n ? ry + 46 + k * 20 : wy + 66 + k * 20;
      add(`<g class="${d.show(`${id}o${k}`, at, 96)}">`);
      if (cls === "m")
        add(
          `<circle cx="${rx + 4}" cy="${y - 4}" r="4" style="fill:none;stroke:var(--dg-muted);stroke-width:1.5"/>`,
        );
      else add(`<circle class="${cls}" cx="${rx + 4}" cy="${y - 4}" r="4"/>`);
      add(`<text class="${cls} xs b" x="${rx + 14}" y="${y}">${esc(t)}</text></g>`);
    });
  });

  return d.render({
    id: n ? "tn" : "tw",
    width: g.W,
    height: g.H,
    title:
      "Tidying: flume removes only places it made, and keeps a branch that holds commits main lacks",
    desc:
      "flume looks at three worktrees in turn. fix/bump-mise landed, and its commits are on main: flume removes the place and the branch. " +
      "change/parquet was dropped holding two commits main lacks: flume removes the place and keeps the branch, and names it. " +
      "A worktree called spike was made by hand: flume did not make it, and leaves it alone.",
    still: 92,
  });
}
