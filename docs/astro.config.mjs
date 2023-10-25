import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import autoImport from "astro-auto-import";

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
        github: "https://github.com/pls-rs/pls",
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
            { label: "Markup", link: "/guides/markup/" },
            { label: "Specs", link: "/guides/specs/" },
            { label: "Contribute", link: "/guides/contribute/" },
          ],
        },
        {
          label: "Features",
          items: [
            {
              label: "Detail view",
              items: [
                { label: "View", link: "/features/detail_view/" },
                { label: "Header", link: "/features/header/" },
                { label: "Units", link: "/features/units/" },
              ],
            },
            {
              label: "Grid view",
              items: [
                { label: "View", link: "/features/grid_view/" },
                { label: "Direction", link: "/features/direction/" },
              ],
            },
            {
              label: "Presentation",
              items: [
                { label: "Icons", link: "/features/icons/" },
                { label: "Suffixes", link: "/features/suffixes/" },
                { label: "Symlinks", link: "/features/symlinks/" },
                { label: "Collapse", link: "/features/collapse/" },
                { label: "Alignment", link: "/features/alignment/" },
              ],
            },
            {
              label: "Filtering",
              items: [
                { label: "Name filter", link: "/features/name_filter/" },
                { label: "Type filter", link: "/features/type_filter/" },
                { label: "Importance", link: "/features/importance/" },
              ],
            },
            { label: "Sorting", link: "/features/sorting/" },
            { label: "Colors", link: "/features/colors/" },
            { label: "Upcoming", link: "/features/upcoming/" },
          ],
        },
        {
          label: "Cookbooks",
          autogenerate: {
            directory: "cookbooks",
          },
        },
      ],
      customCss: [
        "./src/styles/brand.css",
        "./src/styles/font.css",
        "./src/styles/layout.css",
        "./src/styles/terminal.css",
        "./src/styles/typography.css",
      ],
      editLink: {
        baseUrl: "https://github.com/pls-rs/pls/edit/main/docs/",
      },
    }),
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
  ],
  markdown: {
    smartypants: false, // SmartyPants converts '--' into en-dash, breaking alignment.
  },
});
