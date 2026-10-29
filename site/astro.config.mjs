// flume's site, built by Astro with Starlight and published to GitHub Pages
// at https://agentplot.github.io/flume by .github/workflows/pages.yml. The SVG
// diagrams are drawn at build time by src/diagrams/*.ts and inlined, so they
// follow the theme toggle and need no script; the one canvas,
// src/components/Delivery.astro, draws itself from the time in its loop.

import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";
import starlightLinksValidator from "starlight-links-validator";

export default defineConfig({
  site: "https://agentplot.github.io",
  base: "/flume",
  integrations: [
    starlight({
      title: "flume",
      description:
        "One launch, one worktree, a fresh agent for each stage, and the place tidied once the work lands.",
      logo: {
        light: "./src/assets/mark-light.svg",
        dark: "./src/assets/mark-dark.svg",
        alt: "flume",
      },
      favicon: "/favicon.svg",
      // The family's header (its bar above Starlight's) and footer, from
      // @agentplot/site-theme; the start page's hero, with the animated
      // diagram, is flume's own. The theme's banner says a tool is designed
      // and not built while its family list says so, and flume runs, so
      // Starlight's own banner stands.
      components: {
        Header: "@agentplot/site-theme/starlight/Header.astro",
        Footer: "@agentplot/site-theme/starlight/Footer.astro",
        Hero: "./src/components/Hero.astro",
      },
      // The family's type, ink and accent, then the diagrams' palette.
      customCss: ["@agentplot/site-theme/starlight.css", "./src/styles/brand.css"],
      social: [{ icon: "github", label: "GitHub", href: "https://github.com/agentplot/flume" }],
      editLink: {
        baseUrl: "https://github.com/agentplot/flume/edit/main/site/",
      },
      plugins: [starlightLinksValidator()],
      sidebar: [
        { label: "Start", link: "/" },
        { label: "How it works", link: "/how-it-works/" },
        { label: "Install and set up", link: "/install/" },
        { label: "How it fits", link: "/how-it-fits/" },
        { label: "Design", link: "/design/" },
        { label: "Roadmap", link: "/roadmap/" },
      ],
    }),
  ],
});
