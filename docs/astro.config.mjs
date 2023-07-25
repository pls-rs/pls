import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  base: "/pls", // Search and update `/pls` links if changing this.
  integrations: [
    starlight({
      title: "pls",
      logo: {
        light: "./src/assets/logo_light.svg",
        dark: "./src/assets/logo_dark.svg",
      },
      social: {
        github: "https://github.com/dhruvkb/pls",
      },
      customCss: [
        "./src/styles/brand.css",
        "./src/styles/font.css",
        "./src/styles/layout.css",
        "./src/styles/terminal.css",
        "./src/styles/typography.css",
      ],
    }),
  ],
  markdown: {
    smartypants: false, // SmartyPants converts '--' into en-dash, breaking alignment.
  },
});
