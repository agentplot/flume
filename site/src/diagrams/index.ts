// Every SVG diagram the site draws, by name, each a function of its layout.
// Prompt delivery and stage ends are drawn on a canvas instead
// (src/components/Delivery.astro).
import type { Layout } from './kit';
import { hero } from './hero';
import { kinds } from './kinds';
import { herdr } from './herdr';
import { tidy } from './tidy';

export const diagrams: Record<string, (layout: Layout) => string> = {
  hero,
  kinds,
  herdr,
  tidy,
};
