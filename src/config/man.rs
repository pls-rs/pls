use crate::config::Conf;
use crate::exc::Exc;
use figment::providers::{Data, Format, Serialized, Yaml};
use figment::Figment;
use git2::Repository;
use log::{debug, info};
use std::path::Path;

/// Manages the configuration system of the application. This manager provides
/// `Conf` instances tailored to each path, while caching the base configuration
/// for performance.
pub struct ConfMan {
	/// the base configuration, i.e. the serialized output of [`Conf::default`]
	pub base: Figment,
}

impl Default for ConfMan {
	/// This includes config files from the following locations:
	///
	/// * the home directory, if determined
	fn default() -> Self {
		info!("Preparing base configuration.");

		let mut base = Figment::from(Serialized::defaults(Conf::default()));
		if let Some(home_yaml) = home::home_dir().and_then(Self::conf_at) {
			base = base.admerge(home_yaml);
		}

		info!("Base configuration prepared.");
		Self { base }
	}
}

impl ConfMan {
	/// Look for a config file in the given directory and return its contents.
	///
	/// This function will return `None` if no config file is found inside the
	/// given directory.
	fn conf_at<P>(dir: P) -> Option<Data<Yaml>>
	where
		P: AsRef<Path>,
	{
		let conf_file = dir.as_ref().join(".pls.yml");
		conf_file.exists().then(|| {
			debug!("Found config file {conf_file:?}.");
			Yaml::file(conf_file)
		})
	}

	/// Collects all the relevant `.pls.yml` config files into a vector.
	///
	/// This includes config files from the following locations:
	///
	/// * all parent directories up to the Git root, if Git-tracked
	/// * the given path, if a directory, or it's parent
	///
	/// # Arguments
	///
	/// * `path` - the path to scan for config files
	fn yaml_contents(path: &Path) -> Vec<Data<Yaml>> {
		// the given path, if a directory, or it's parent; Note that symlinks
		// are treated as files in this situation.
		let mut curr = if !path.is_symlink() && path.is_dir() {
			path.to_path_buf()
		} else {
			match path.parent() {
				Some(par) => par.to_path_buf(),
				None => return vec![],
			}
		};

		let mut paths = vec![curr.clone()];

		let repo_root = Repository::discover(path)
			.ok()
			.and_then(|repo| repo.workdir().map(Path::to_path_buf));
		if let Some(repo_root) = repo_root {
			while curr.pop() {
				paths.push(curr.clone());
				if curr == repo_root {
					break;
				}
			}
		}
		debug!("Checking for configs in {paths:?}.");

		paths.iter().rev().filter_map(Self::conf_at).collect()
	}

	/// Get a `Conf` instance for the given path.
	///
	/// This merges the path-specific config files with the base and returns the
	/// resulting [`Conf`] instance, If there is an error parsing the config
	/// files, an [`Exc`] instance will be returned.
	pub fn get<P>(&self, path: Option<P>) -> Result<Conf, Exc>
	where
		P: AsRef<Path>,
	{
		let mut fig = self.base.clone();

		if let Some(path) = path {
			for file in Self::yaml_contents(path.as_ref()) {
				fig = fig.admerge(file);
			}
		}

		fig.extract().map_err(Exc::Conf)
	}
}
