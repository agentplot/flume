// How it works: flume's herdr plugin, as a small mock, in one 26 s loop.
// The sidebar lists every place in flight, each with its kind, its stage and
// its agent's state, and a change's tasks ticked. A fix lands and leaves the
// list once tidied. The open key shows a place's pane beside its status; the
// tell key answers an agent that stopped short and said what it needs; stop
// ends an agent going the wrong way, and retry starts its stage again with a
// fresh one. A sounding waiting on you shows as a row too.
//
// Timeline, percent of the loop:
//   0-28 a change's tasks tick · 8-22 a fix lands, leaves · 30-50 open, tell
//   52-76 open, stop, retry
import { Diagram, esc, type Layout } from './kit';

const T = 26;
// A state that holds to the loop's end fades out over its last frames, as
// the loop's first state fades back in.
const END = 99.2;

const KEYS: [string, number[]][] = [
  ['open', [30, 52]],
  ['tell', [36]],
  ['stop', [60]],
  ['retry', [68]],
];

interface Box {
  x: number;
  y: number;
  w: number;
  h: number;
}

function geometry(layout: Layout) {
  if (layout === 'wide') {
    return {
      W: 720,
      H: 420,
      n: false,
      win: { x: 16, y: 16, w: 688, h: 388 } as Box,
      side: { x: 16, y: 44, w: 220, h: 304 } as Box,
      slot: (i: number) => ({ x: 24, y: 78 + i * 52, w: 204, h: 46 }),
      ws: { x: 236, y: 44, w: 468, h: 304 } as Box,
      wsTitle: { x: 250, y: 70 },
      paneA: { x: 248, y: 84, w: 246, h: 252 } as Box,
      paneB: { x: 504, y: 84, w: 188, h: 252 } as Box,
      keysY: 362,
      keysX: 84,
    };
  }
  return {
    W: 360,
    H: 820,
    n: true,
    win: { x: 8, y: 16, w: 344, h: 792 } as Box,
    side: { x: 8, y: 44, w: 344, h: 302 } as Box,
    slot: (i: number) => ({ x: 16, y: 76 + i * 52, w: 328, h: 46 }),
    ws: { x: 8, y: 346, w: 344, h: 400 } as Box,
    wsTitle: { x: 20, y: 370 },
    paneA: { x: 16, y: 382, w: 328, h: 200 } as Box,
    paneB: { x: 16, y: 590, w: 328, h: 148 } as Box,
    keysY: 762,
    keysX: 16,
  };
}

export function herdr(layout: Layout) {
  const g = geometry(layout);
  const n = g.n;
  const d = new Diagram(T);
  const add = (s: string) => d.add(s);

  add(`<rect class="bg" width="${g.W}" height="${g.H}" rx="10"/>`);
  const W = g.win;
  add(`<rect class="box" x="${W.x}" y="${W.y}" width="${W.w}" height="${W.h}" rx="10"/>`);
  [0, 1, 2].forEach((i) => add(`<circle class="dot" cx="${W.x + 16 + i * 14}" cy="${W.y + 14}" r="4"/>`));
  add(`<text class="m xs" x="${W.x + W.w / 2}" y="${W.y + 18}" text-anchor="middle">herdr · flume’s plugin</text>`);
  add(`<line x1="${W.x}" y1="${W.y + 28}" x2="${W.x + W.w}" y2="${W.y + 28}" style="stroke:var(--dg-rule)"/>`);

  // ---- the sidebar: every place in flight
  const SB = g.side;
  add(`<rect class="panel" x="${SB.x + 1}" y="${SB.y}" width="${SB.w - 1}" height="${SB.h}"/>`);
  add(`<text class="hd" x="${SB.x + 14}" y="${SB.y + 22}">IN FLIGHT</text>`);
  if (!n) add(`<line x1="${SB.x + SB.w}" y1="${SB.y}" x2="${SB.x + SB.w}" y2="${SB.y + SB.h}" style="stroke:var(--dg-rule)"/>`);
  else add(`<line x1="${SB.x}" y1="${SB.y + SB.h}" x2="${SB.x + SB.w}" y2="${SB.y + SB.h}" style="stroke:var(--dg-rule)"/>`);

  // A row moves between slots: [percent, slot] steps, each move taking 2%.
  const slotY = (i: number) => g.slot(i).y;
  const moves = (cls: string, home: number, steps: [number, number][]) => {
    const at = (s: number) => `transform:translate(0px,${slotY(s) - slotY(home)}px)`;
    const fr: [number, string][] = [[0, at(steps[0][1])]];
    for (let k = 1; k < steps.length; k++) {
      const [p, s] = steps[k];
      fr.push([p - 2, at(steps[k - 1][1])], [p, at(s)]);
    }
    fr.push([100, at(steps[steps.length - 1][1])]);
    d.kf(cls, fr);
    return d.anim(cls);
  };

  // One row: its branch, its kind, its stage and state over time, its
  // selection, and a change's tasks.
  const row = (
    id: string,
    branch: string,
    home: number,
    steps: [number, number][],
    shown: [number, number],
    states: [string, string, number, number][],
    selected: [number, number][],
    ticks?: number[],
  ) => {
    const s = g.slot(home);
    add(`<g class="${moves(`mv${id}`, home, steps)}">`);
    add(`<g class="${shown[1] >= 100 ? '' : d.show(`vis${id}`, shown[0], shown[1])}">`);
    selected.forEach(([a, b], k) => {
      add(`<g class="${d.show(`sel${id}${k}`, a, b)}"><rect class="accbox" x="${s.x}" y="${s.y}" width="${s.w}" height="${s.h}" rx="6" style="stroke-width:0"/>`);
      add(`<rect class="chip" x="${s.x}" y="${s.y + 6}" width="3" height="${s.h - 12}" rx="1.5"/></g>`);
    });
    add(`<text class="t b mono xs" x="${s.x + 12}" y="${s.y + 18}">${esc(branch)}</text>`);
    states.forEach(([txt, cls, a, b], k) => {
      const dot = cls === 'm' ? `<circle cx="${s.x + 16}" cy="${s.y + 33}" r="4" style="fill:none;stroke:var(--dg-muted);stroke-width:1.5"/>` : `<circle class="${cls}" cx="${s.x + 16}" cy="${s.y + 33}" r="4"/>`;
      add(`<g class="${b >= END && a <= 0 ? '' : d.show(`rs${id}${k}`, a, b)}">${dot}<text class="xxs ${cls}" x="${s.x + 26}" y="${s.y + 37}">${esc(txt)}</text></g>`);
    });
    if (ticks) {
      // The change's tasks, as a count that climbs.
      const all = ticks.length + 3;
      const counts = [3, ...ticks.map((_, k) => 4 + k)];
      counts.forEach((c, k) => {
        const a = k === 0 ? -1 : ticks[k - 1];
        const b = k === counts.length - 1 ? END : ticks[k];
        add(`<text class="ac mono xxs b ${d.show(`ct${id}${k}`, a, b)}" x="${s.x + s.w - 10}" y="${s.y + 18}" text-anchor="end">${c}/${all}</text>`);
      });
    }
    add('</g></g>');
  };

  // change/csv-export: its tasks tick, and it moves on to verify.
  row(
    'a',
    'change/csv-export',
    0,
    [[0, 0]],
    [-1, 100],
    [
      ['change · code · working', 'ac', -1, 24],
      ['change · verify · working', 'ac', 24, END],
    ],
    [[-1, 30]],
    [6, 12, 18],
  );
  // fix/bump-mise: lands, is tidied and leaves the list.
  row(
    'b',
    'fix/bump-mise',
    1,
    [[0, 1]],
    [-1, 20],
    [
      ['fix · submit · in sluice', 'ac', -1, 10],
      ['landed', 'ok', 10, 15],
      ['tidied: place and branch gone', 'm', 15, 20],
    ],
    [],
  );
  // fix/flaky-test: stopped short, says what it needs; told, works again.
  row(
    'c',
    'fix/flaky-test',
    2,
    [
      [0, 2],
      [22, 1],
      [100, 1],
    ],
    [-1, 100],
    [
      ['fix · code · stopped: needs input', 'warn', -1, 43],
      ['fix · code · working', 'ac', 43, END],
    ],
    [[30, 52]],
  );
  // sounding/parquet: a sounding waiting on you.
  row(
    'd',
    'sounding/parquet',
    3,
    [
      [0, 3],
      [22, 2],
      [100, 2],
    ],
    [-1, 100],
    [['question · waiting on you', 'warn', -1, END]],
    [],
  );
  // fix/lint-config: going the wrong way; stopped, and retried.
  row(
    'e',
    'fix/lint-config',
    4,
    [
      [0, 4],
      [22, 3],
      [100, 3],
    ],
    [-1, 100],
    [
      ['fix · code · working', 'ac', -1, 60],
      ['fix · code · stopped', 'bad', 60, 68],
      ['fix · code · agent 2 · working', 'ac', 68, END],
    ],
    [[52, 96]],
  );

  // ---- the workspace: the place's pane beside its status
  const P1 = g.paneA;
  const P2 = g.paneB;
  const ttl = (txt: string, a: number, b: number, k: string) =>
    add(`<text class="t b mono xs ${d.show(`wt${k}`, a, b)}" x="${g.wsTitle.x}" y="${g.wsTitle.y}">${esc(txt)}</text>`);
  ttl('change/csv-export', -1, 30, 'a');
  ttl('fix/flaky-test', 30, 52, 'c');
  ttl('fix/lint-config', 52, 96, 'e');
  add(`<text class="m xs ${d.show('wt0', 96, 100)}" x="${g.wsTitle.x}" y="${g.wsTitle.y}">change/csv-export</text>`);
  add(`<rect class="box2" x="${P1.x}" y="${P1.y}" width="${P1.w}" height="${P1.h}" rx="6"/>`);
  add(`<rect class="box2" x="${P2.x}" y="${P2.y}" width="${P2.w}" height="${P2.h}" rx="6"/>`);

  // The agent's pane, by phase: a header line, then its lines.
  const lineH = 19;
  const pane = (head: string, lines: [string, string, number][], a: number, b: number, k: string) => {
    add(`<text class="m xxs ${d.show(`ph${k}`, a, b)}" x="${P1.x + 10}" y="${P1.y + 18}">${esc(head)}</text>`);
    lines.forEach(([t, c, at], i) =>
      add(`<text class="mono xxs ${c} ${d.show(`pl${k}${i}`, Math.max(at, a), b)}" x="${P1.x + 10}" y="${P1.y + 42 + i * lineH}">${esc(t)}</text>`),
    );
  };
  // tasks.md, ticking, in the change's code stage.
  {
    add(`<text class="m xxs ${d.show('ph-a', -1, 24)}" x="${P1.x + 10}" y="${P1.y + 18}">agent 2 · code · tasks.md</text>`);
    const tasks = ['read the ledger’s rows', 'write rows as CSV', 'quote fields with commas', 'stream large ledgers', 'add the export command', 'test a round trip'];
    const at = [-1, -1, -1, 6, 12, 18];
    tasks.forEach((t, i) => {
      const y = P1.y + 32 + i * 21;
      const x = P1.x + 10;
      add(`<g class="${d.show(`tk${i}`, -1, 24)}">`);
      add(`<rect x="${x}" y="${y}" width="11" height="11" rx="2" style="fill:none;stroke:var(--dg-muted);stroke-width:1.2"/>`);
      add(`<text class="t xxs" x="${x + 18}" y="${y + 10}">${esc(t)}</text>`);
      add('</g>');
      add(`<g class="${d.show(`tt${i}`, at[i], 24)}"><rect class="ac" x="${x}" y="${y}" width="11" height="11" rx="2"/><path d="M${x + 2.5},${y + 5.5} l2.5,2.5 l4,-5" style="fill:none;stroke:var(--dg-ink);stroke-width:1.6;stroke-linecap:round;stroke-linejoin:round"/></g>`);
    });
  }
  pane(
    'agent 3 · verify · a fresh agent',
    [
      ['prompt in its transcript', 'ok', 25],
      ['checking the code against', 't', 27],
      ['the change, task by task', 't', 27],
    ],
    24,
    30,
    'a2',
  );
  pane(
    'agent 1 · code',
    [
      ['ran the failing test', 't', 30],
      ['it reads a fixture that is', 't', 30],
      ['not in the repository.', 't', 30],
      ['Which CSV should it read?', 'warn', 30],
    ],
    30,
    43,
    'c1',
  );
  // The tell: typed, sent as a prompt, found in the transcript.
  {
    const y = P1.y + P1.h - 62;
    add(`<g class="${d.show('tell', 36, 43)}">`);
    add(`<rect class="warnbox" x="${P1.x + 8}" y="${y}" width="${P1.w - 16}" height="50" rx="6"/>`);
    add(`<text class="warn xxs b" x="${P1.x + 18}" y="${y + 17}">tell fix/flaky-test</text>`);
    const msg = 'use fixtures/small.csv';
    add(`<text class="t mono xxs" x="${P1.x + 18}" y="${y + 37}">${msg}</text>`);
    d.kf('ttype', [
      [0, 'transform:scaleX(1)'],
      [37, 'transform:scaleX(1)'],
      [41, 'transform:scaleX(0)'],
      [100, 'transform:scaleX(0)'],
    ]);
    add(`<rect class="cover ${d.anim('ttype')}" x="${P1.x + 16}" y="${y + 26}" width="${msg.length * 6.4 + 6}" height="15"/>`);
    d.rule(`.ttype{animation-timing-function:steps(${msg.length},end)}`);
    add('</g>');
  }
  pane(
    'agent 1 · code',
    [
      ['your tell, in its transcript', 'ok', 43],
      ['reading fixtures/small.csv', 't', 45],
      ['the test passes', 't', 48],
    ],
    43,
    52,
    'c2',
  );
  pane(
    'agent 1 · code',
    [
      ['rewriting .eslintrc in', 't', 52],
      ['every package, one by one', 't', 54],
    ],
    52,
    60,
    'e1',
  );
  pane(
    'agent 1 · code',
    [
      ['stopped: agent 1 ended', 'bad', 60],
      ['its place and branch kept', 'm', 61],
    ],
    60,
    68,
    'e2',
  );
  pane(
    'agent 2 · code · a fresh agent',
    [
      ['prompt in its transcript', 'ok', 69],
      ['the brief: one root config', 't', 71],
      ['editing eslint.config.js', 't', 74],
    ],
    68,
    96,
    'e3',
  );

  // The place's status, as flume status --json gives it.
  const bx = P2.x + 10;
  const by = P2.y + 18;
  add(`<text class="m xxs" x="${bx}" y="${by}">flume status</text>`);
  const status = (rows: [string, string, string][], a: number, b: number, k: string) => {
    add(`<g class="${d.show(`st${k}`, a, b)}">`);
    rows.forEach(([key, val, cls], i) => {
      const y = by + 26 + i * (n ? 20 : 34);
      add(`<text class="m xxs" x="${bx}" y="${y}">${key}</text>`);
      add(`<text class="${cls} xxs b" x="${n ? bx + 90 : bx}" y="${n ? y : y + 15}">${esc(val)}</text>`);
    });
    add('</g>');
  };
  status(
    [
      ['kind', 'change', 't'],
      ['stage', 'code', 't'],
      ['agent', 'working', 'ac'],
      ['deliverable', 'tasks ticked, not all', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    -1,
    24,
    'a1',
  );
  status(
    [
      ['kind', 'change', 't'],
      ['stage', 'verify', 't'],
      ['agent', 'working', 'ac'],
      ['deliverable', 'no report yet', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    24,
    30,
    'a2',
  );
  status(
    [
      ['kind', 'fix', 't'],
      ['stage', 'code', 't'],
      ['agent', 'stopped short', 'warn'],
      ['deliverable', 'no commit yet', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    30,
    43,
    'c1',
  );
  status(
    [
      ['kind', 'fix', 't'],
      ['stage', 'code', 't'],
      ['agent', 'working', 'ac'],
      ['deliverable', 'no commit yet', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    43,
    52,
    'c2',
  );
  status(
    [
      ['kind', 'fix', 't'],
      ['stage', 'code', 't'],
      ['agent', 'working', 'ac'],
      ['deliverable', 'no commit yet', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    52,
    60,
    'e1',
  );
  status(
    [
      ['kind', 'fix', 't'],
      ['stage', 'code', 't'],
      ['agent', 'stopped', 'bad'],
      ['deliverable', 'no commit yet', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    60,
    68,
    'e2',
  );
  status(
    [
      ['kind', 'fix', 't'],
      ['stage', 'code, retried', 't'],
      ['agent', 'working', 'ac'],
      ['deliverable', 'no commit yet', 'm'],
      ['submission', 'none yet', 'm'],
    ],
    68,
    96,
    'e3',
  );

  // ---- the keys
  add(`<text class="hd" x="${n ? g.keysX : W.x + 16}" y="${g.keysY + (n ? -8 : 16)}">KEYS</text>`);
  let kx = n ? g.keysX : g.keysX + 6;
  const ky = n ? g.keysY + 2 : g.keysY;
  KEYS.forEach(([label, at], i) => {
    const w = label.length * 7 + 26;
    add(`<rect class="box2" x="${kx}" y="${ky}" width="${w}" height="24" rx="5"/>`);
    add(`<text class="t xs" x="${kx + w / 2}" y="${ky + 16}" text-anchor="middle">${label}</text>`);
    const fr: [number, number][] = [[0, 0]];
    at.forEach((p) => fr.push([p, 1], [p + 3, 0]));
    add(
      `<g class="${d.levels(`k${i}`, fr)}"><rect class="chip" x="${kx}" y="${ky}" width="${w}" height="24" rx="5"/>` +
        `<text class="chipt xs" x="${kx + w / 2}" y="${ky + 16}" text-anchor="middle">${label}</text></g>`,
    );
    kx += w + 8;
  });
  if (!n) add(`<text class="m xxs" x="${W.x + W.w - 16}" y="${g.keysY + 16}" text-anchor="end">a place’s pane, beside its status</text>`);

  return d.render({
    id: n ? 'rn' : 'rw',
    width: g.W,
    height: g.H,
    title: 'flume’s herdr plugin: every place in flight, a place’s pane and status, and the keys',
    desc:
      'A herdr window. Its sidebar lists every place in flight with its kind, its stage and its agent’s state: change/csv-export in code, its tasks ticking from 3 of 6 to 6 of 6 before it moves on to verify; ' +
      'fix/bump-mise in sluice, which lands, is tidied and leaves the list; fix/flaky-test, whose agent stopped short and says what it needs; sounding/parquet, a question waiting on you; and fix/lint-config, working. ' +
      'Beside the selected place’s pane, its status gives the kind, the stage, the agent’s state, the deliverable and the submission. ' +
      'The open key shows fix/flaky-test, whose agent asks which CSV its test should read; the tell key sends use fixtures/small.csv as a prompt, it reaches the transcript, and the agent works again. ' +
      'The open key shows fix/lint-config, rewriting the wrong file; the stop key ends its agent, keeping its place and branch, and the retry key starts the stage again with a fresh agent.',
    still: 34,
  });
}
