import { satteri } from "@astrojs/markdown-satteri";
import starlight from "@astrojs/starlight";
import { defineConfig } from "astro/config";

import { autoImport, type AutoImport } from "./src/plugins/auto-import";

// Components auto-imported into every MDX document.
const autoImports: AutoImport[] = [
	{ name: "Pls", from: "@/components/Pls.astro" },
	{ name: "Stars", from: "@/components/Stars.astro" },
	{ name: "Version", from: "@/components/Version.astro" },
	{ name: "DocBlock", from: "@/components/DocBlock.astro" },
];

// https://astro.build/config
export default defineConfig({
	devToolbar: {
		enabled: false,
	},
	markdown: {
		processor: satteri({
			features: { smartPunctuation: false },
			mdastPlugins: [autoImport(autoImports)],
		}),
	},
	integrations: [
		starlight({
			title: "pls",
			favicon: "/favicon.ico",
			logo: {
				light: "./src/assets/logo_light.svg",
				dark: "./src/assets/logo_dark.svg",
			},
			expressiveCode: {
				themes: ["catppuccin-mocha", "catppuccin-latte"],
			},
			social: [
				{
					icon: "github",
					label: "GitHub",
					href: "https://github.com/pls-rs/pls",
				},
			],
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
						{ label: "Paths", link: "/guides/paths/" },
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
					items: [{ autogenerate: { directory: "cookbooks" } }],
				},
				{
					label: "Reference",
					items: [{ autogenerate: { directory: "reference" } }],
				},
			],
			customCss: [
				"./src/styles/brand.css",
				"./src/styles/color.css",
				"./src/styles/font.css",
				"./src/styles/layout.css",
				"./src/styles/terminal.css",
				"./src/styles/typography.css",
			],
			components: {
				Footer: "./src/components/Footer.astro",
				SocialIcons: "./src/components/SocialIcons.astro",
			},
			editLink: {
				baseUrl: "https://github.com/pls-rs/pls/edit/main/docs/",
			},
		}),
	],
});
