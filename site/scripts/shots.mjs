// Screenshots of the built site, to judge it by eye: every page at a phone's
// and a desktop's width, in both themes, plus frames of every diagram's
// animation, SVG and canvas, and its still for a reader who asks for less
// motion.
//
//   bun run build && bun scripts/shots.mjs [out-dir] [--frames-only] [--page <slug>]
//
// Serves dist/ itself under the site's base, so nothing else need run.
import { chromium } from 'playwright';
import { mkdirSync, existsSync, statSync, readFileSync } from 'node:fs';
import { createServer } from 'node:http';
import { join, extname, resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const dist = resolve(here, '../dist');
const args = process.argv.slice(2);
const out = resolve(args.find((a) => !a.startsWith('--') && args[args.indexOf(a) - 1] !== '--page') ?? resolve(here, '../shots'));
const framesOnly = args.includes('--frames-only');
const onlyPage = args.includes('--page') ? args[args.indexOf('--page') + 1] : null;
mkdirSync(out, { recursive: true });

const BASE = '/flume';
const types = { '.html': 'text/html', '.css': 'text/css', '.js': 'text/javascript', '.svg': 'image/svg+xml', '.png': 'image/png', '.woff2': 'font/woff2', '.woff': 'font/woff', '.json': 'application/json' };
const server = createServer((req, res) => {
  let p = decodeURIComponent(new URL(req.url, 'http://x').pathname);
  if (!p.startsWith(BASE)) return res.writeHead(404).end();
  p = join(dist, p.slice(BASE.length));
  if (existsSync(p) && statSync(p).isDirectory()) p = join(p, 'index.html');
  if (!existsSync(p)) return res.writeHead(404).end();
  res.writeHead(200, { 'content-type': types[extname(p)] ?? 'application/octet-stream' });
  res.end(readFileSync(p));
});
await new Promise((r) => server.listen(0, '127.0.0.1', r));
const origin = `http://127.0.0.1:${server.address().port}${BASE}`;

const pages = [
  ['start', '/'],
  ['how-it-works', '/how-it-works/'],
  ['how-it-fits', '/how-it-fits/'],
  ['design', '/design/'],
  ['roadmap', '/roadmap/'],
].filter(([slug]) => !onlyPage || slug === onlyPage);
const widths = [1280, 390];
const themes = ['dark', 'light'];

// The full Chromium in its new headless mode: Nix's browsers hold no headless shell.
const browser = await chromium.launch({ channel: 'chromium' });
async function open(width, theme, path, reduced = false, height = 900, scale = 2) {
  const ctx = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: scale, reducedMotion: reduced ? 'reduce' : 'no-preference' });
  await ctx.addInitScript((t) => localStorage.setItem('starlight-theme', t), theme);
  const page = await ctx.newPage();
  await page.goto(origin + path, { waitUntil: 'networkidle' });
  await page.evaluate(() => document.fonts.ready);
  return { ctx, page };
}
// An element's shot is taken through the viewport, so the sticky header and
// the phone's "On this page" bar would cover the top of a diagram: hide them.
const unstick = (page) =>
  page.evaluate(() => {
    for (const e of document.querySelectorAll('body *')) {
      const pos = getComputedStyle(e).position;
      if (pos === 'fixed' || pos === 'sticky') e.style.visibility = 'hidden';
    }
  });

for (const [slug, path] of pages) {
  for (const width of widths) {
    for (const theme of themes) {
      // A page is shot as a reader who asks for less motion sees it: every
      // diagram at its still, so two runs compare.
      // Chromium draws no taller than 16384 device pixels and repeats the
      // page's top below that, so a page too tall for two device pixels a
      // pixel is shot at one.
      if (!framesOnly) {
        let shot = await open(width, theme, path, true);
        const tall = await shot.page.evaluate(() => document.documentElement.scrollHeight);
        if (tall * 2 > 16000) {
          await shot.ctx.close();
          shot = await open(width, theme, path, true, 900, 1);
        }
        await shot.page.screenshot({ path: join(out, `${slug}-${width}-${theme}.png`), fullPage: true });
        await shot.ctx.close();
      }
      // A viewport taller than any diagram, so an element shot is never cut.
      const { ctx, page } = await open(width, theme, path, false, 2400);
      await unstick(page);
      // Frames of each diagram in this layout, in the dark theme at desktop
      // width and both themes on a phone.
      const figs = await page.$$('figure.diagram');
      for (const fig of figs) {
        const name = (await fig.getAttribute('class')).match(/diagram-([\w-]+)/)[1];
        const layout = width >= 600 ? 'wide' : 'narrow';
        const svg = await fig.$(`.${layout} svg`);
        if (!svg) continue;
        const dur = await svg.evaluate((s) => {
          const a = document.getAnimations().find((x) => s.contains(x.effect?.target));
          return a ? a.effect.getTiming().duration : 0;
        });
        if (!dur) continue;
        const steps = layout === 'wide' ? [0.06, 0.15, 0.24, 0.33, 0.42, 0.52, 0.6, 0.68, 0.8, 0.9] : [0.15, 0.42, 0.6, 0.9];
        if (theme === 'light' && layout === 'wide') steps.splice(0, steps.length, 0.42, 0.9);
        for (const f of steps) {
          await svg.evaluate((s, t) => {
            document.getAnimations().filter((x) => s.contains(x.effect?.target)).forEach((a) => { a.pause(); a.currentTime = t; });
          }, f * dur);
          await svg.screenshot({ path: join(out, `frame-${name}-${layout}-${theme}-${String(Math.round(f * 100)).padStart(2, '0')}.png`) });
        }
      }
      // The canvas diagrams draw any frame they are told to hold (fl:frame).
      const canvases = await page.$$('figure.fl-canvas');
      for (const [k, fig] of canvases.entries()) {
        const layout = width >= 600 ? 'wide' : 'narrow';
        const steps = layout === 'wide' ? [0.06, 0.13, 0.19, 0.25, 0.3, 0.4, 0.55, 0.7, 0.8, 0.84] : [0.13, 0.25, 0.55, 0.84];
        if (theme === 'light' && layout === 'wide') steps.splice(0, steps.length, 0.25, 0.84);
        for (const f of steps) {
          await fig.evaluate((el, t) => el.dispatchEvent(new CustomEvent('fl:frame', { detail: t })), f);
          await page.waitForTimeout(60);
          await (await fig.$('canvas')).screenshot({ path: join(out, `frame-canvas${k}-${layout}-${theme}-${String(Math.round(f * 100)).padStart(2, '0')}.png`) });
        }
      }
      await ctx.close();
    }
  }
  // The still a reader who asks for less motion sees.
  for (const width of widths) {
    const { ctx, page } = await open(width, 'dark', path, true, 2400);
    await unstick(page);
    const figs = await page.$$('figure.diagram');
    for (const fig of figs) {
      const name = (await fig.getAttribute('class')).match(/diagram-([\w-]+)/)[1];
      const layout = width >= 600 ? 'wide' : 'narrow';
      const svg = await fig.$(`.${layout} svg`);
      if (svg) await svg.screenshot({ path: join(out, `still-${name}-${layout}.png`) });
    }
    for (const [k, fig] of (await page.$$('figure.fl-canvas')).entries()) {
      await (await fig.$('canvas')).screenshot({ path: join(out, `still-canvas${k}-${width >= 600 ? 'wide' : 'narrow'}.png`) });
    }
    await ctx.close();
  }
}

await browser.close();
server.close();
console.log(`shots in ${out}`);
