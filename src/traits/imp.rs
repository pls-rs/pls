use crate::config::{Args, Conf};
use crate::models::Node;
use log::debug;

pub trait Imp {
	fn default_imp(&self) -> i8;
	fn imp_val(&self, args: &Args) -> i8;

	fn is_visible(&self, conf: &Conf, args: &Args) -> bool;

	fn directives(&self, conf: &Conf, args: &Args) -> Option<String>;
}

impl Imp for Node<'_> {
	/// Get the implicit relative importance of the node.
	///
	/// This is the importance associated with a node if it has not been set by
	/// any matching spec. By default we assume nodes with a leading dot to be
	/// less important, as they are normally hidden by the `ls(1)` command.
	fn default_imp(&self) -> i8 {
		if self.name.starts_with('.') {
			-1
		} else {
			0
		}
	}

	/// Get the relative importance of the node.
	///
	/// This iterates through the specs in reverse, finding the first available
	/// importance or falling back the the [default](Imp::default_imp). Then it
	/// subtracts the baseline level from the CLI args.
	fn imp_val(&self, args: &Args) -> i8 {
		self.specs
			.iter()
			.rev()
			.find_map(|spec| spec.importance)
			.unwrap_or(self.default_imp())
			- args.imp
	}

	/// Determine whether the node should be displayed in the list.
	///
	/// Elements below the lowest-defined relative-importance are hidden.
	fn is_visible(&self, conf: &Conf, args: &Args) -> bool {
		debug!("Checking visibility of \"{self}\" based on importance.");
		let rel_imp = self.imp_val(args);
		let min_val = conf.constants.min_imp();

		let is_visible = rel_imp >= min_val;
		if !is_visible {
			debug!("\"{self}\" with relative importance {rel_imp} (min: {min_val}) is hidden.")
		}
		is_visible
	}

	/* Directives */
	/* ========== */

	/// Get the directives associated with the node's relative importance.
	///
	/// The directives are read from the configuration with any missing values
	/// having no directives set for them.
	///
	/// If the node's importance is above the maximum defined, it will be set to
	/// the maximum. If it is below the minimum defined, it will already be
	/// hidden by [`is_visible`](Imp::is_visible).
	fn directives(&self, conf: &Conf, args: &Args) -> Option<String> {
		let mut rel_imp = self.imp_val(args);
		let max_val = conf.constants.max_imp();
		let min_val = conf.constants.min_imp();
		debug!("\"{self}\" has relative importance {rel_imp} (min: {min_val}, max: {max_val})");

		rel_imp = rel_imp.clamp(min_val, max_val);
		conf.constants.imp_map.get(&rel_imp).cloned()
	}
}
