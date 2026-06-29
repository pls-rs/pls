import {
	defineMdastPlugin,
	type MdastContent,
	type MdastNode,
	type MdastPluginDefinition,
	type MdastPluginInstance,
} from "satteri";

export interface AutoImport {
	/** Local binding name as used in MDX, e.g. `Pls`. */
	name: string;
	/** Module specifier to import from, e.g. `@/components/Pls.astro`. */
	from: string;
}

// Sätteri doesn't export the visitor context type directly, so recover it from
// a visitor signature.
type VisitorContext = Parameters<NonNullable<MdastPluginInstance["paragraph"]>>[1];

/**
 * Sätteri mdast plugin that injects component imports into every MDX document,
 * replacing `astro-auto-import`. That package is a remark plugin, and remark
 * plugins no longer run once the Markdown pipeline moves from `unified()` to
 * Sätteri.
 *
 * Imports are emitted as `raw` nodes: Sätteri re-parses them through the MDX
 * parser into `mdxjsEsm` nodes, which MDX hoists to the top of the module. Going
 * through `raw` keeps the insert type-safe, since the `mdxjsEsm` node type isn't
 * part of the standard `MdastNode` union accepted by the insert helpers.
 */
export function autoImport(imports: readonly AutoImport[]): MdastPluginDefinition {
	const importNodes: MdastContent[] = imports.map((it) => ({
		raw: `import ${it.name} from ${JSON.stringify(it.from)};`,
	}));

	const inject = (node: Readonly<MdastNode>, ctx: VisitorContext): void => {
		// A raw import is only valid in MDX; in plain `.md` it renders as literal
		// text, so skip anything that isn't an MDX document.
		if (!ctx.fileURL?.pathname.endsWith(".mdx")) return;
		// `ctx.data` is a fresh bag per document, so it doubles as an inject-once
		// guard without per-document plugin state.
		if (ctx.data.autoImportInjected) return;
		if (ctx.parent(node)?.type !== "root") return;

		ctx.data.autoImportInjected = true;
		// Imports must stay below frontmatter; everywhere else, prepend.
		if (node.type === "yaml" || node.type === "toml") {
			ctx.insertAfter(node, importNodes);
		} else {
			ctx.insertBefore(node, importNodes);
		}
	};

	// The same visitor is attached to every node type that can be a direct child
	// of the root; whichever comes first (pre-order) is the document's first
	// top-level node, where the imports get anchored.
	return defineMdastPlugin({
		name: "auto-import",
		yaml: inject,
		toml: inject,
		paragraph: inject,
		heading: inject,
		thematicBreak: inject,
		blockquote: inject,
		list: inject,
		code: inject,
		html: inject,
		table: inject,
		definition: inject,
		footnoteDefinition: inject,
		math: inject,
		containerDirective: inject,
		leafDirective: inject,
		mdxJsxFlowElement: inject,
		mdxFlowExpression: inject,
	});
}
