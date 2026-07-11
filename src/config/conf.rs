use crate::config::app_const::AppConst;
use crate::config::entry_const::EntryConst;
use crate::enums::{Collapse, ColorScheme};
use crate::models::Spec;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;

/// Create a [`HashMap`] from a list of key-value pairs.
macro_rules! map_str_str {
	( $($k:expr => $v:expr,)* ) => {
		core::convert::From::from([
			$( (String::from($k), String::from($v)), )*
		])
	};
}

/// Represents the complete configuration of `pls`.
///
/// `pls` comes with a lean configuration out-of-the-box and users are
/// encouraged to add their own configuration using YAML files in the home
/// directory, project Git root and/or working directory.
///
/// Note that `pls` also accepts CLI arguments, which are not represented here.
/// Refer to [`Args`](crate::config::Args) for those.
#[derive(Serialize, Deserialize)]
pub struct Conf {
	/// mapping of icon names to actual glyphs from Nerd Fonts or paths to SVGs
	pub icons: HashMap<String, String>,
	/// the installed icon pack to resolve `theme:<key>` icons against, if any
	pub icon_pack: Option<IconPackConfig>,
	/// list of node specs, in ascending order of specificity
	pub specs: Vec<Spec>,
	/// constants that determine the appearance and styling of each entry
	pub entry_const: EntryConst,
	/// constants that determine the appearance and styling of the entire UI
	pub app_const: AppConst,
}

/// The installed icon pack `pls` draws `theme:<key>` icons from, which theme
/// within it to use, and how icons adapt to the terminal's color scheme.
///
/// A pack contributes one or more themes; `pls` draws SVGs from one of them,
/// chosen either as a single [`default`](DefaultTheme) for every color scheme or
/// [`per_scheme`](PerScheme) so a light and a dark theme can differ (e.g.
/// Catppuccin's Latte/Mocha). A pack that provides a single theme needs neither.
#[derive(Serialize, Deserialize)]
pub struct IconPackConfig {
	/// the ID (`<publisher>.<name>`) of the installed pack
	pub name: String,
	/// the single theme to use for every color scheme
	#[serde(default)]
	pub default: Option<DefaultTheme>,
	/// distinct themes chosen by the detected terminal color scheme
	///
	/// Set this or [`default`](Self::default), not both; `per_scheme` takes
	/// precedence if both are present.
	#[serde(default)]
	pub per_scheme: Option<PerScheme>,
}

/// A single theme, used regardless of the terminal's color scheme.
#[derive(Serialize, Deserialize)]
pub struct DefaultTheme {
	/// the theme's ID within the pack; required only when the pack provides more
	/// than one theme
	#[serde(default)]
	pub name: Option<String>,
	/// rewrite from an icon key to its light-terminal variant, for packs that
	/// ship both within a single theme
	///
	/// In a light terminal, the rewritten key is looked up first, falling back
	/// to the original when the theme has no such variant. Ignored in a dark
	/// terminal. See [`LightTransform`] for the syntax.
	#[serde(default)]
	pub light_transform: Option<LightTransform>,
}

/// A pair of themes chosen by the detected terminal color scheme.
///
/// Both are required, so a half-configured pair cannot exist. Each value is a
/// theme ID within the [pack](IconPackConfig::name).
#[derive(Serialize, Deserialize)]
pub struct PerScheme {
	pub dark: String,
	pub light: String,
}

/// A `sed`-style `s/<pattern>/<replacement>/` rewrite from a dark-terminal icon
/// key to its light-terminal counterpart.
///
/// The pattern is a [regular expression](Regex) and the replacement uses the
/// `regex` crate's substitution syntax — `$1` or `${1}` for capture groups,
/// `$$` for a literal `$`. This is expressive enough for the differing light-key
/// conventions across packs:
///
/// * a suffix, e.g. Material's `<key>` → `<key>_light`: `s/$/_light/`
/// * an infix, e.g. vscode-icons' `_f_<name>` → `_f_light_<name>`:
///   `s/^_(fd?)_/_${1}_light_/`
///
/// The delimiter is whatever character follows the leading `s`, so a pattern
/// containing `/` can use another, e.g. `s|a/b|c|`.
pub struct LightTransform {
	regex: Regex,
	replacement: String,
}

impl LightTransform {
	/// Rewrite an icon key to its light-terminal variant.
	///
	/// Returns the input unchanged when the pattern does not match.
	pub fn apply<'key>(&self, key: &'key str) -> Cow<'key, str> {
		self.regex.replace(key, self.replacement.as_str())
	}
}

impl FromStr for LightTransform {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut chars = s.chars();
		if chars.next() != Some('s') {
			return Err(format!("expected an `s/…/…/` substitution, got {s:?}"));
		}
		let delim = chars
			.next()
			.ok_or_else(|| format!("missing delimiter in {s:?}"))?;

		// The remainder must be exactly `<pattern><delim><replacement><delim>`,
		// i.e. three delimiter-separated parts with empty trailing flags.
		let parts: Vec<&str> = chars.as_str().split(delim).collect();
		let [pattern, replacement, ""] = parts.as_slice() else {
			return Err(format!(
				"expected `s{delim}pattern{delim}replacement{delim}`, got {s:?}"
			));
		};

		let regex = Regex::new(pattern).map_err(|e| format!("invalid pattern in {s:?}: {e}"))?;
		Ok(Self {
			regex,
			replacement: replacement.to_string(),
		})
	}
}

impl Serialize for LightTransform {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&format!("s/{}/{}/", self.regex.as_str(), self.replacement))
	}
}

impl<'de> Deserialize<'de> for LightTransform {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let s = String::deserialize(deserializer)?;
		s.parse().map_err(serde::de::Error::custom)
	}
}

impl IconPackConfig {
	/// Whether resolving an icon needs the terminal's color scheme detected.
	///
	/// Detection is comparatively expensive (it queries the terminal), so it is
	/// only worthwhile when the scheme actually changes the outcome: a
	/// `per_scheme` theme pair or a configured [`light_transform`].
	///
	/// [`light_transform`]: DefaultTheme::light_transform
	pub fn needs_scheme(&self) -> bool {
		self.per_scheme.is_some() || self.light_transform().is_some()
	}

	/// The ID of the theme to use for the given color scheme, or `None` when the
	/// pack needs no disambiguation (it provides a single theme).
	pub fn theme_id(&self, color_scheme: ColorScheme) -> Option<&str> {
		if let Some(per_scheme) = &self.per_scheme {
			return Some(match color_scheme {
				ColorScheme::Dark => &per_scheme.dark,
				ColorScheme::Light => &per_scheme.light,
			});
		}
		self.default.as_ref().and_then(|d| d.name.as_deref())
	}

	/// The light-variant key rewrite, if one is configured on the default theme.
	pub fn light_transform(&self) -> Option<&LightTransform> {
		self.default
			.as_ref()
			.and_then(|d| d.light_transform.as_ref())
	}
}

impl Default for Conf {
	fn default() -> Self {
		Self {
			icon_pack: None,
			icons: map_str_str!(
				// pls
				"pls"          => "", // nf-oct-primitive_dot
				"missing"      => "", // nf-cod-error
				// Node types
				"file"         => "",
				"dir"          => "", // nf-fa-folder
				"symlink"      => "󰌹", // nf-md-link-variant
				"fifo"         => "󰟥", // nf-md-pipe
				"socket"       => "󰟨", // nf-md-power_socket_uk
				"char_device"  => "", // nf-fa-paragraph
				"block_device" => "󰋊", // nf-md-harddisk
				// Generic
				"audio"        => "󰓃", // nf-md-speaker
				"book"         => "", // nf-fa-book
				"broom"        => "󰃢", // nf-md-broom
				"config"       => "", // nf-seti-config
				"container"    => "", // nf-oct-container
				"env"          => "", // nf-fae-plant
				"image"        => "󰋩", // nf-md-image
				"json"         => "", // nf-seti-json
				"law"          => "", // nf-oct-law
				"lock"         => "", // nf-oct-lock
				"package"      => "", // nf-oct-package
				"runner"       => "󰜎", // nf-md-run
				"shell"        => "", // nf-oct-terminal
				"source"       => "", // nf-oct-file_code
				"test"         => "󰙨", // nf-md-test_tube
				"text"         => "", // nf-seti-text
				"video"        => "󰕧", // nf-md-video
				// Brands
				"apple"        => "", // nf-fa-apple
				"git"          => "󰊢", // nf-md-git
				"github"       => "", // nf-oct-mark_github
				"markdown"     => "", // nf-oct-markdown
				"rust"         => "", // nf-seti-rust
			),
			specs: vec![
				// Extensions
				Spec::new(r"\.sh$", "shell"),
				Spec::new(r"\.rs$", "rust").style("rgb(247,76,0)"),
				Spec::new(r"\.(txt|rtf)$", "text"),
				Spec::new(r"\.mdx?$", "markdown"),
				Spec::new(r"\.ini$", "config"),
				Spec::new(r"\.(json|toml|yml|yaml)$", "json"),
				Spec::new(r"\.(jpg|jpeg|png|svg|webp|gif|ico)$", "image"),
				Spec::new(r"\.(mov|mp4|mkv|webm|avi|flv)$", "video"),
				Spec::new(r"\.(mp3|flac|ogg|wav)$", "audio"),
				// Partial names
				Spec::new(r"^\.env\b", "env"),
				Spec::new(r"^README\b", "book").importance(2),
				Spec::new(r"^LICENSE\b", "law"),
				Spec::new(r"docker-compose.*\.yml$", "container"),
				Spec::new(r"Dockerfile", "container"),
				// Exact names
				Spec::new(r"^\.DS_Store$", "apple").importance(-2),
				Spec::new(r"^\.pls\.yml$", "pls").importance(0),
				Spec::new(r"^\.git$", "git").importance(-2),
				Spec::new(r"^\.gitignore$", "git"),
				Spec::new(r"^\.github$", "github"),
				Spec::new(r"^src$", "source").importance(1),
				Spec::new(r"^(justfile|Makefile)$", "runner"),
				Spec::new(r"^Cargo\.toml$", "package"),
				Spec::new(r"^Cargo\.lock$", "lock")
					.importance(-1)
					.collapse(Collapse::Name(String::from("Cargo.toml"))),
				Spec::new(r"^rustfmt.toml$", "broom"),
			],
			entry_const: EntryConst::default(),
			app_const: AppConst::default(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{IconPackConfig, LightTransform};
	use crate::enums::ColorScheme;

	#[test]
	fn test_sole_theme_needs_no_disambiguation() {
		let config: IconPackConfig = json5::from_str(r#"{ "name": "pub.pack" }"#).unwrap();
		assert_eq!(config.theme_id(ColorScheme::Dark), None);
		assert_eq!(config.theme_id(ColorScheme::Light), None);
		assert!(!config.needs_scheme());
	}

	#[test]
	fn test_default_theme_used_on_both_schemes() {
		let config: IconPackConfig =
			json5::from_str(r#"{ "name": "pub.pack", "default": { "name": "only" } }"#).unwrap();
		assert_eq!(config.theme_id(ColorScheme::Dark), Some("only"));
		assert_eq!(config.theme_id(ColorScheme::Light), Some("only"));
	}

	#[test]
	fn test_per_scheme_picks_matching_theme() {
		let config: IconPackConfig = json5::from_str(
			r#"{ "name": "pub.pack", "per_scheme": { "dark": "mocha", "light": "latte" } }"#,
		)
		.unwrap();
		assert_eq!(config.theme_id(ColorScheme::Dark), Some("mocha"));
		assert_eq!(config.theme_id(ColorScheme::Light), Some("latte"));
		assert!(config.needs_scheme());
	}

	#[test]
	fn test_light_transform_deserializes_on_default_theme() {
		let config: IconPackConfig = json5::from_str(
			r#"{ "name": "pub.pack", "default": { "light_transform": "s/$/_light/" } }"#,
		)
		.unwrap();
		assert_eq!(
			config.light_transform().unwrap().apply("rust"),
			"rust_light"
		);
		assert!(config.needs_scheme());
	}

	#[test]
	fn test_light_transform_suffix() {
		let transform: LightTransform = "s/$/_light/".parse().unwrap();
		assert_eq!(transform.apply("rust"), "rust_light");
	}

	#[test]
	fn test_light_transform_infix() {
		let transform: LightTransform = r"s/^_(fd?)_/_${1}_light_/".parse().unwrap();
		assert_eq!(transform.apply("_f_astro"), "_f_light_astro");
		assert_eq!(transform.apply("_fd_src"), "_fd_light_src");
	}

	#[test]
	fn test_light_transform_leaves_non_matching_key() {
		let transform: LightTransform = r"s/^_f_/_f_light_/".parse().unwrap();
		assert_eq!(transform.apply("folder-src"), "folder-src");
	}

	#[test]
	fn test_light_transform_honours_custom_delimiter() {
		let transform: LightTransform = "s|a/b|c|".parse().unwrap();
		assert_eq!(transform.apply("a/b"), "c");
	}

	#[test]
	fn test_light_transform_rejects_malformed() {
		assert!("x/a/b/".parse::<LightTransform>().is_err()); // not an `s` substitution
		assert!("s/only-two/".parse::<LightTransform>().is_err()); // missing a part
		assert!("s/a/b/g".parse::<LightTransform>().is_err()); // trailing flags
		assert!(r"s/([/x/".parse::<LightTransform>().is_err()); // invalid regex
	}

	#[test]
	fn test_per_scheme_takes_precedence_over_default() {
		let config: IconPackConfig = json5::from_str(
			r#"{ "name": "pub.pack", "default": { "name": "only" }, "per_scheme": { "dark": "mocha", "light": "latte" } }"#,
		)
		.unwrap();
		assert_eq!(config.theme_id(ColorScheme::Dark), Some("mocha"));
	}

	#[test]
	fn test_half_configured_pair_is_rejected() {
		// `per_scheme` demands both keys; a lone `light` cannot deserialize.
		assert!(
			json5::from_str::<IconPackConfig>(
				r#"{ "name": "pub.pack", "per_scheme": { "light": "latte" } }"#
			)
			.is_err()
		);
	}
}
