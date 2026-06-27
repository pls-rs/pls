import skipFormatting from "eslint-config-prettier/flat";
import pluginAstro from "eslint-plugin-astro";
import pluginOxlint from "eslint-plugin-oxlint";
import { globalIgnores } from "eslint/config";
import tseslint from "typescript-eslint";

export default tseslint.config(
	{ name: "app/files-to-lint", files: ["**/*.{astro,ts}"] },

	globalIgnores(["**/dist/**", "**/.astro/**"]),

	// Astro must come after `typescript-eslint`: the latter has no `files`
	// filter and sets the parser for every file, so it would clobber the Astro
	// parser on `.astro` files unless Astro is applied last.
	...tseslint.configs.recommended,

	// `eslint-plugin-astro` automatically enables the TypeScript parser for Astro
	// files if it can require `@typescript-eslint/parser` from the project root.
	// So ensure that this package is listed as a direct dev dependency.
	...pluginAstro.configs.recommended,

	...pluginOxlint.buildFromOxlintConfigFile(".oxlintrc.json"),

	// Disable formatting rules to avoid conflicts with Prettier (and also Oxfmt).
	skipFormatting,
);
