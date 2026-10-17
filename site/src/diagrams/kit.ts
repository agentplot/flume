// The diagrams' builder. A diagram is an SVG with CSS keyframes only, so it
// animates with no script, inline in a page or as an <img> on GitHub. Every
// diagram is drawn twice, a wide and a narrow layout (Diagram.astro shows the
// one its column fits), and each copy's classes, keyframes and ids are
// prefixed so no two copies on a page meet. Colours are the --dg-* variables
// of src/styles/brand.css, drawn from the family theme's tokens, so a diagram
// follows the site's theme toggle; a standalone copy (the README's) carries
// its own palette instead.
import { member } from '@agentplot/site-theme/family';

export type Frames = [number, string][];

export interface RenderOptions {
  id: string; // the prefix: unique per diagram and layout
  width: number;
  height: number;
  title: string;
  desc: string;
  still: number; // the percent of the loop a reader who asks for less motion sees
  standalone?: boolean;
}

// The accents a diagram reads from the family list: flume's own, and the
// callers' where a diagram draws them: millpond's inbox, headwater's
// releases and sounding's questions.
const own = member('flume').accent;
export const pond = member('millpond').accent;
export const head = member('headwater').accent;
export const sound = member('sounding').accent;

// The palette a standalone copy carries: brand.css's --dg-* as they
// resolve from @agentplot/site-theme's tokens and the family's accents.
const LIGHT =
  '--dg-bg:#e9e7e0;--dg-panel:#f2f1ec;--dg-soft:#dbd9d1;--dg-rule:#c9c7bf;--dg-fg:#0e1116;--dg-muted:#5d6370;' +
  `--dg-accent:${own.light};--dg-accent-soft:color-mix(in oklab,${own.light} 20%,#f2f1ec);--dg-pond:${pond.light};--dg-head:${head.light};--dg-sound:${sound.light};` +
  '--dg-ink:#f7f6f2;--dg-ok:#1f8a4c;--dg-warn:#8a5a00;--dg-bad:#c0392b';
const DARK =
  '--dg-bg:#161a21;--dg-panel:#1b2029;--dg-soft:#21232b;--dg-rule:#2a2f38;--dg-fg:#f2f1ec;--dg-muted:#8c92a0;' +
  `--dg-accent:${own.dark};--dg-accent-soft:color-mix(in oklab,${own.dark} 20%,#1b2029);--dg-pond:${pond.dark};--dg-head:${head.dark};--dg-sound:${sound.dark};` +
  '--dg-ink:#0e1116;--dg-ok:#45c27a;--dg-warn:#e9b44c;--dg-bad:#f08a7e';

// What every diagram draws with. ROOT_W is the copy's root class.
const BASE = `
ROOT_W text{font-family:var(--sl-font,-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif);fill:var(--dg-fg)}
ROOT_W .mono,ROOT_W .mono *{font-family:var(--sl-font-mono,ui-monospace,SFMono-Regular,Menlo,Consolas,monospace)}
.bg{fill:var(--dg-bg)} .t{fill:var(--dg-fg)} .m{fill:var(--dg-muted)} .ac{fill:var(--dg-accent)} .ok{fill:var(--dg-ok)} .warn{fill:var(--dg-warn)} .bad{fill:var(--dg-bad)}
.b{font-weight:600} .h{font-size:17px;font-weight:700} .hd{font-size:11px;letter-spacing:.07em;fill:var(--dg-muted)}
.sm{font-size:13px} .xs{font-size:11.5px} .xxs{font-size:10.5px}
.box{fill:var(--dg-panel);stroke:var(--dg-rule);stroke-width:1.5}
.box2{fill:var(--dg-panel);stroke:var(--dg-rule);stroke-width:1}
.panel{fill:var(--dg-soft)}
.chip{fill:var(--dg-accent)} .chipt{fill:var(--dg-ink);font-weight:600}
.ghost{fill:none;stroke:var(--dg-accent);stroke-width:1.5}
.tag{fill:none;stroke:var(--dg-muted);stroke-width:1}
.tagj{fill:var(--dg-accent)} .tagjt{fill:var(--dg-ink);font-weight:600}
.track{fill:var(--dg-soft)} .bar{fill:var(--dg-accent);transform-box:fill-box;transform-origin:left center}
.edge{stroke:var(--dg-muted);stroke-width:2;fill:none}
.dash{stroke:var(--dg-accent);stroke-width:1.5;fill:none;stroke-dasharray:4 4}
.okdash{stroke:var(--dg-ok);stroke-width:1.5;fill:none;stroke-dasharray:4 4}
.strike{stroke:var(--dg-muted);stroke-width:1.25}
.dot{fill:var(--dg-muted)} .merge{fill:var(--dg-ok)} .mergein{fill:var(--dg-panel)}
.pill{fill:var(--dg-fg)} .pillt{fill:var(--dg-bg);font-weight:600}
.badge{fill:var(--dg-accent)} .badget{fill:var(--dg-ink);font-size:11.5px;font-weight:700}
.file{fill:var(--dg-panel);stroke:var(--dg-muted);stroke-width:1}
.accbox{fill:var(--dg-accent-soft);stroke:var(--dg-accent);stroke-width:1.5}
.okbox{fill:var(--dg-panel);stroke:var(--dg-ok);stroke-width:1.5}
.warnbox{fill:var(--dg-panel);stroke:var(--dg-warn);stroke-width:1.5}
.badbox{fill:var(--dg-panel);stroke:var(--dg-bad);stroke-width:1.5}
.pd{fill:var(--dg-pond)} .pdchip{fill:var(--dg-pond)}
.pondbox{fill:var(--dg-panel);stroke:var(--dg-pond);stroke-width:1.5}
.hw{fill:var(--dg-head)} .sd{fill:var(--dg-sound)}
.hwbox{fill:var(--dg-panel);stroke:var(--dg-head);stroke-width:1.5}
.sdbox{fill:var(--dg-panel);stroke:var(--dg-sound);stroke-width:1.5}
.cover{fill:var(--dg-panel);transform-box:fill-box;transform-origin:right center}
.move{transform-box:view-box}
`;

export class Diagram {
  private parts: string[] = [];
  private css: string[] = [];
  private keepOnStill: string[] = [];

  constructor(
    readonly T: number, // seconds per loop
    readonly E = 0.8, // fade width, percent
  ) {}

  add(s: string) {
    this.parts.push(s);
  }

  kf(name: string, frames: Frames) {
    const body = frames.map(([p, v]) => `${+p.toFixed(3)}%{${v}}`).join(' ');
    this.css.push(`@keyframes k-${name}{${body}}`);
  }

  anim(cls: string, name = cls) {
    this.css.push(`.${cls}{animation:k-${name} ${this.T}s linear infinite}`);
    return cls;
  }

  rule(css: string) {
    this.css.push(css);
  }

  // Opacity hi inside [a, b], lo outside, fading in over [a, a + E] and out
  // over [b, b + E], so a window that starts where another ends crosses it
  // rather than sharing a frame with it. A window that starts at or before 0
  // is shown at the loop's start too.
  show(cls: string, a: number, b: number, lo = 0, hi = 1) {
    const E = this.E;
    let fr: Frames;
    if (a <= 0) {
      fr = [
        [0, `opacity:${hi}`],
        [b, `opacity:${hi}`],
        [b + E, `opacity:${lo}`],
        [100 - E, `opacity:${lo}`],
        [100, `opacity:${hi}`],
      ];
    } else {
      fr = [
        [0, `opacity:${lo}`],
        [a, `opacity:${lo}`],
        [Math.min(a + E, b), `opacity:${hi}`],
        [b, `opacity:${hi}`],
      ];
      // A window that runs to the loop's end holds there: two keyframes at
      // 100% would leave only the last, and fade the whole window out.
      if (b < 100) fr.push([Math.min(b + E, 100), `opacity:${lo}`]);
      if (b + E < 100) fr.push([100, `opacity:${lo}`]);
    }
    this.kf(cls, fr);
    return this.anim(cls);
  }

  // A sequence of opacity levels: [[at, level], ...], each held until the next.
  levels(cls: string, steps: [number, number][]) {
    const E = this.E;
    const fr: Frames = [];
    steps.forEach(([at, v], i) => {
      if (i === 0) fr.push([0, `opacity:${v}`]);
      else {
        const prev = steps[i - 1][1];
        fr.push([Math.max(at - E, 0), `opacity:${prev}`], [at, `opacity:${v}`]);
      }
    });
    fr.push([100, `opacity:${steps[steps.length - 1][1]}`]);
    this.kf(cls, fr);
    return this.anim(cls);
  }

  // A move from (dx, dy) to (0, 0) between a and b, hidden before `from` and
  // after `gone`.
  fly(cls: string, dx: number, dy: number, from: number, a: number, b: number, gone: number) {
    const at = (x: number, y: number, o: number) => `transform:translate(${x}px,${y}px);opacity:${o}`;
    const fr: Frames = [
      [0, at(dx, dy, 0)],
      [Math.max(from - 0.5, 0), at(dx, dy, 0)],
      [from, at(dx, dy, 1)],
      [a, at(dx, dy, 1)],
      [b, at(0, 0, 1)],
      [gone - this.E, at(0, 0, 1)],
      [gone, at(0, 0, 0)],
    ];
    if (gone < 100) fr.push([100, at(0, 0, 0)]);
    this.kf(cls, fr);
    return this.anim(cls);
  }

  // A move along a polyline of points ending where the element is drawn,
  // between a and b, the time shared by length; hidden before a, gone after.
  route(cls: string, pts: { x: number; y: number }[], a: number, b: number, gone: number) {
    const end = pts[pts.length - 1];
    const at = (p: { x: number; y: number }, o: number) => `transform:translate(${p.x - end.x}px,${p.y - end.y}px);opacity:${o}`;
    const lens = pts.slice(1).map((p, i) => Math.hypot(p.x - pts[i].x, p.y - pts[i].y));
    const total = lens.reduce((x, y) => x + y, 0);
    const fr: Frames = [
      [0, at(pts[0], 0)],
      [Math.max(a - 0.5, 0), at(pts[0], 0)],
      [a, at(pts[0], 1)],
    ];
    let t = a;
    lens.forEach((l, i) => {
      t += ((b - a) * l) / total;
      fr.push([t, at(pts[i + 1], 1)]);
    });
    fr.push([gone - this.E, at(end, 1)], [gone, at(end, 0)]);
    if (gone < 100) fr.push([100, at(end, 0)]);
    this.kf(cls, fr);
    return this.anim(cls);
  }

  // Water moving along a dashed stroke: the dashes travel about `speed`
  // pixels a second, a whole number of periods a loop so the loop closes.
  flow(cls: string, period: number, speed: number) {
    const n = Math.max(1, Math.round((speed * this.T) / period));
    this.kf(cls, [
      [0, 'stroke-dashoffset:0'],
      [100, `stroke-dashoffset:${-n * period}`],
    ]);
    return this.anim(cls);
  }

  // Elements shown whole, unanimated, to a reader who asks for less motion.
  keep(cls: string) {
    this.keepOnStill.push(cls);
    return cls;
  }

  render(o: RenderOptions): string {
    const p = o.id;
    const root = `.${p}-root`;
    const prefix = (css: string) =>
      css
        .replace(/\.(?=[a-zA-Z])/g, `.${p}-`)
        .replace(/\bk-/g, `${p}-k-`);
    // The element selectors hang off :where(root), so they weigh nothing and
    // a class such as .m wins over them.
    let style = prefix(BASE).replaceAll('ROOT_W ', `:where(${root}) `);
    style += prefix(this.css.join(''));
    const keep = this.keepOnStill.map((c) => `.${p}-${c}`).join(',');
    style +=
      `@media (prefers-reduced-motion:reduce){${root} *{animation-play-state:paused!important;` +
      `animation-delay:${(-(o.still / 100) * this.T).toFixed(2)}s!important}` +
      (keep ? `${keep}{animation:none!important;opacity:1!important}` : '') +
      `}`;
    if (o.standalone) style = `:root{${LIGHT}}@media (prefers-color-scheme:dark){:root{${DARK}}}` + style;
    const body = this.parts
      .join('\n')
      .replace(/class="([^"]*)"/g, (_, c: string) => `class="${c.split(/\s+/).filter(Boolean).map((x) => `${p}-${x}`).join(' ')}"`);
    const size = o.standalone ? ` width="${o.width}" height="${o.height}"` : '';
    return (
      `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${o.width} ${o.height}"${size} class="${p}-root dg" role="img" aria-labelledby="${p}-ttl ${p}-dsc">` +
      `<title id="${p}-ttl">${o.title}</title><desc id="${p}-dsc">${o.desc}</desc>` +
      `<style>${style}</style>\n${body}\n</svg>\n`
    );
  }
}

export type Layout = 'wide' | 'narrow';

export const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
