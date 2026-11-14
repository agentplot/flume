// flume's one hooks module (C.4): the pinned engine loads one hooks module
// per plugin, so this registers each of the plugin's own, every one a file
// of its own reading once whether flume started its session: the place's
// line under the prompt (line.ts; design 4) and the account's usage
// windows passed to `flume hook usage` (usage.ts; 184, 211). It hooks
// nothing itself.

import type { Register } from "claude-code";
import { register as line } from "./line";
import { register as usage } from "./usage";

export const register: Register = async (on, options) => {
  await line(on, options);
  await usage(on, options);
};
