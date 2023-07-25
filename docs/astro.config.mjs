import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import autoImport from "astro-auto-import";

// https://astro.build/config
export default defineConfig({
  base: "/pls", // Search and update `/pls` links if changing this.
  integrations: [
    autoImport({
      imports: [
        "@/components/Dhruv.astro",
        "@/components/Footer.astro",
        "@/components/Icon.astro",
        "@/components/Pls.astro",
        "@/components/Stars.astro",
        "@/components/Stat.astro",
        "@/components/Version.astro",
      ],
    }),
    starlight({
      title: "pls",
      logo: {
        light: "./src/assets/logo_light.svg",
        dark: "./src/assets/logo_dark.svg",
      },
      social: {
        github: "https://github.com/dhruvkb/pls",
      },
      sidebar: [
        {
          label: "About",
          items: [
            { label: "Introduction", link: "/about/intro" },
            { label: "FAQ", link: "/about/faq/" },
            { label: "Comparison", link: "/about/comparison/" },
          ],
        },
        {
          label: "Guides",
          items: [
            { label: "Get started", link: "/guides/get_started/" },
          ],
        },
      ],
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
