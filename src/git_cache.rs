use git2::Repository;
use log::warn;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A cache for git status information to avoid repeated git calls
pub struct GitStatusCache {
	/// Cache mapping repository root paths to their status maps
	/// The inner HashMap maps relative file paths to their git status strings
	cache: HashMap<PathBuf, HashMap<String, String>>,
	/// Cache for unpushed commits per repository
	unpushed_cache: HashMap<PathBuf, Vec<String>>,
}

impl GitStatusCache {
	pub fn new() -> Self {
		Self {
			cache: HashMap::new(),
			unpushed_cache: HashMap::new(),
		}
	}

	/// Get the git status for a specific file path, using cached results when possible
	pub fn get_status(&mut self, file_path: &Path) -> Option<String> {
		// Convert to absolute path first
		let absolute_path = match file_path.canonicalize() {
			Ok(path) => path,
			Err(_) => return Some("  ".to_string()),
		};

		// Try to find the git repository from the file's path
		let repo = match Repository::discover(&absolute_path) {
			Ok(repo) => repo,
			Err(_) => return Some("  ".to_string()), // Not in a git repo
		};

		// Get the repository root
		let repo_root = match repo.workdir() {
			Some(root) => root.to_path_buf(),
			None => return Some("  ".to_string()),
		};

		// Check if we have cached results for this repository
		if !self.cache.contains_key(&repo_root) {
			self.populate_cache(&repo_root);
		}

		// Get the relative path from the repository root
		let relative_path = match absolute_path.strip_prefix(&repo_root) {
			Ok(path) => path.to_string_lossy().to_string(),
			Err(_) => return Some("  ".to_string()),
		};

		// Look up the status in our cache
		if let Some(repo_cache) = self.cache.get(&repo_root) {
			if let Some(status) = repo_cache.get(&relative_path) {
				Some(status.clone())
			} else {
				// File is not in git status output, check if it's in unpushed commits (committed but not pushed)
				if let Some(unpushed_files) = self.unpushed_cache.get(&repo_root) {
					if unpushed_files.contains(&relative_path) {
						Some("<yellow>↑ </>".to_string())
					} else {
						Some("  ".to_string())
					}
				} else {
					Some("  ".to_string())
				}
			}
		} else {
			Some("  ".to_string())
		}
	}

	/// Populate the cache for a specific repository using `git status --short`
	fn populate_cache(&mut self, repo_root: &Path) {
		let mut status_map = HashMap::new();

		// Run git status --short to get all statuses at once
		let output = match Command::new("git")
			.arg("status")
			.arg("--short")
			.arg("--porcelain")
			.current_dir(repo_root)
			.output()
		{
			Ok(output) => output,
			Err(e) => {
				warn!("Failed to run git status: {}", e);
				self.cache.insert(repo_root.to_path_buf(), status_map);
				return;
			}
		};

		if !output.status.success() {
			warn!("git status command failed");
			self.cache.insert(repo_root.to_path_buf(), status_map);
			return;
		}

		// Parse the git status output
		let status_output = String::from_utf8_lossy(&output.stdout);
		for line in status_output.lines() {
			if line.len() >= 3 {
				let status_chars = &line[0..2];
				let file_path = &line[3..];

				// Format the status with colors similar to the original implementation
				let formatted_status = self.format_git_status_from_chars(status_chars);
				status_map.insert(file_path.to_string(), formatted_status);
			}
		}

		// Populate the unpushed commits cache first
		self.populate_unpushed_cache(repo_root);

		// Handle directories that contain modified files
		// For each file, mark its parent directories appropriately
		let file_paths: Vec<String> = status_map.keys().cloned().collect();
		for file_path in file_paths {
			let path = Path::new(&file_path);
			let mut current_dir = path.parent();

			while let Some(dir) = current_dir {
				let dir_str = dir.to_string_lossy().to_string();
				if !dir_str.is_empty() && !status_map.contains_key(&dir_str) {
					status_map.insert(dir_str, "<red> *</>".to_string());
				}
				current_dir = dir.parent();
			}
		}

		// Handle directories that contain unpushed files (committed but not pushed)
		// For each unpushed file, mark its parent directories with yellow up arrow if they don't already have a status
		if let Some(unpushed_files) = self.unpushed_cache.get(repo_root) {
			for file_path in unpushed_files {
				let path = Path::new(file_path);
				let mut current_dir = path.parent();

				while let Some(dir) = current_dir {
					let dir_str = dir.to_string_lossy().to_string();
					if !dir_str.is_empty() && !status_map.contains_key(&dir_str) {
						// Check if this directory has any uncommitted files by looking at status_map
						let has_uncommitted = status_map.keys().any(|key| {
							let key_path = Path::new(key);
							key_path.starts_with(&dir_str) && status_map.get(key).map(|s| s.contains("*")).unwrap_or(false)
						});
						
						// If directory doesn't have uncommitted files, show yellow up arrow for unpushed
						if !has_uncommitted {
							status_map.insert(dir_str, "<yellow>↑ </>".to_string());
						}
					}
					current_dir = dir.parent();
				}
			}
		}

		self.cache.insert(repo_root.to_path_buf(), status_map);

		// Also populate the unpushed commits cache
		self.populate_unpushed_cache(repo_root);
	}

	/// Populate the cache for unpushed commits
	fn populate_unpushed_cache(&mut self, repo_root: &Path) {
		let mut unpushed_files = Vec::new();

		// First, check if there's an upstream branch
		let upstream_check = Command::new("git")
			.arg("rev-parse")
			.arg("--abbrev-ref")
			.arg("@{upstream}")
			.current_dir(repo_root)
			.output();

		if upstream_check.is_err() || !upstream_check.as_ref().unwrap().status.success() {
			// No upstream branch, check if there are any commits ahead of origin/main or origin/master
			let origins = ["origin/main", "origin/master"];
			for origin in &origins {
				let check_origin = Command::new("git")
					.arg("rev-parse")
					.arg("--verify")
					.arg(origin)
					.current_dir(repo_root)
					.output();

				if check_origin.is_ok() && check_origin.unwrap().status.success() {
					// Found this origin branch, check for unpushed commits
					if let Ok(output) = Command::new("git")
						.arg("log")
						.arg(format!("{}..HEAD", origin))
						.arg("--name-only")
						.arg("--pretty=format:")
						.current_dir(repo_root)
						.output()
					{
						if output.status.success() {
							let log_output = String::from_utf8_lossy(&output.stdout);
							for line in log_output.lines() {
								let line = line.trim();
								if !line.is_empty() {
									unpushed_files.push(line.to_string());
								}
							}
						}
					}
					break;
				}
			}

			// Remove duplicates and store
			unpushed_files.sort();
			unpushed_files.dedup();
			self.unpushed_cache
				.insert(repo_root.to_path_buf(), unpushed_files);
			return;
		}

		// Get the list of commits that are ahead of the upstream
		let output = match Command::new("git")
			.arg("log")
			.arg("@{upstream}..HEAD")
			.arg("--name-only")
			.arg("--pretty=format:")
			.current_dir(repo_root)
			.output()
		{
			Ok(output) => output,
			Err(e) => {
				warn!("Failed to run git log for unpushed commits: {}", e);
				self.unpushed_cache
					.insert(repo_root.to_path_buf(), unpushed_files);
				return;
			}
		};

		if !output.status.success() {
			// Could be that there are no unpushed commits, which is fine
			self.unpushed_cache
				.insert(repo_root.to_path_buf(), unpushed_files);
			return;
		}

		// Parse the output to get file names
		let log_output = String::from_utf8_lossy(&output.stdout);
		for line in log_output.lines() {
			let line = line.trim();
			if !line.is_empty() {
				unpushed_files.push(line.to_string());
			}
		}

		// Remove duplicates
		unpushed_files.sort();
		unpushed_files.dedup();

		self.unpushed_cache
			.insert(repo_root.to_path_buf(), unpushed_files);
	}

	/// Format git status characters into colored output similar to the original implementation
	fn format_git_status_from_chars(&self, status_chars: &str) -> String {
		let chars: Vec<char> = status_chars.chars().collect();
		if chars.len() != 2 {
			return "  ".to_string();
		}

		let staged_char = chars[0];
		let unstaged_char = chars[1];

		// Handle special cases
		if status_chars == "!!" {
			return "<red>!!</>".to_string();
		}
		if status_chars == "UU" {
			return "<red>UU</>".to_string();
		}
		if status_chars == "??" {
			return "<red>??</>".to_string();
		}

		// Format with colors: green for staged, red for unstaged
		let staged_formatted = if staged_char == ' ' {
			" ".to_string()
		} else {
			format!("<green>{}</>", staged_char)
		};

		let unstaged_formatted = if unstaged_char == ' ' {
			" ".to_string()
		} else {
			format!("<red>{}</>", unstaged_char)
		};

		format!("{}{}", staged_formatted, unstaged_formatted)
	}

	/// Clear the cache (useful for testing or when repository state might have changed)
	#[allow(dead_code)]
	pub fn clear(&mut self) {
		self.cache.clear();
		self.unpushed_cache.clear();
	}
}
