# The site

flume's site at https://agentplot.github.io/flume, an
[Astro](https://astro.build) site with
[Starlight](https://starlight.astro.build), published by
`.github/workflows/pages.yml` on every push to `main` that touches it.

```sh
devenv shell -- site-build     # moon run site:build: dist/, every internal link checked
devenv shell -- prose          # Vale over the pages and the README
cd site && bun run dev         # http://localhost:4321/flume/
devenv shell -- site-shots <dir>   # screenshots: every page, both widths, both themes, and diagram frames
```

The family's look is `@agentplot/site-theme`: its `starlight.css` carries
the type, the ink and paper of both themes and the accent, `--ap-accent`,
which its family bar sets from the family list (flume's teal). Its header
puts the family bar above Starlight's, its footer adds the family's, its
banner says flume is designed and not built, and How it fits shows its
family diagram with flume highlighted.

## Where things live

| path | what |
|---|---|
| `src/content/docs/` | one page per file: start, how it works, how it fits, design, roadmap |
| `src/diagrams/` | the SVG diagrams, drawn at build time: `kit.ts` the builder, one file per diagram |
| `src/components/Diagram.astro` | inlines a diagram's wide and narrow layouts and shows the one its column fits |
| `src/components/Delivery.astro` | prompt delivery and stage ends, drawn on a canvas from the time in its loop |
| `src/components/Hero.astro` | the start page's hero, with the animated diagram |
| `src/styles/brand.css` | the diagrams' palette, `--dg-*`, drawn from the theme's `--ap-*` tokens |
| `src/assets/`, `public/favicon.svg` | the mark: a trestle carrying the flow, one item in, in the family's 112 by 88 frame |
| `scripts/shots.mjs` | the screenshots, with Playwright and the Chromium devenv provides |

## The diagrams

Each SVG diagram uses CSS keyframes only, so it animates with no script
and follows the theme toggle through the `--dg-*` variables. Each makes
one claim, and its animation shows the mechanism moving: the hero a launch
from its caller to its tidied place, `kinds` the three kinds as tracks of
stages, `tidy` what tidying removes and keeps, and `herdr` the plugin's
sidebar and keys. A reader who asks for less motion sees one still frame
chosen to hold the whole claim. Every diagram is drawn twice, a wide
layout for a column of 600 pixels or more and a narrow one for a phone.

Prompt delivery and stage ends are a canvas, since the pane's output is a
trace over time. It reads the same `--dg-*` variables, redraws when the
theme changes, and draws every frame from the time in its loop, so its
still and a screenshot's frame are exact; an `fl:frame` event on its
figure holds one.

The callers' accents, millpond's `--dg-pond`, headwater's `--dg-head` and
sounding's `--dg-sound`, come from the family list too.

`bun run readme-diagram` writes the hero as a standalone SVG with its own
palette, `docs/diagrams/flume.svg`, for the README.
