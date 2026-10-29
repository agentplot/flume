// Every SVG diagram the site draws, by name, each a function of its layout.
// Prompt delivery and stage ends are drawn on a canvas instead
// (src/components/Delivery.astro).

import { herdr } from "./herdr";
import { hero } from "./hero";
import { kinds } from "./kinds";
import type { Layout } from "./kit";
import { tidy } from "./tidy";

export const diagrams: Record<string, (layout: Layout) => string> = {
  hero,
  kinds,
  herdr,
  tidy,
};
