use crate::args::{Group, Input};
use crate::config::{Args, ConfMan};
use crate::fmt::render;
use crate::models::OwnerMan;

/// Represents the entire application state.
#[derive(Default)]
pub struct Pls {
	/// configuration manager for `.pls.yml` files
	pub conf_man: ConfMan,
	/// command-line arguments
	pub args: Args,
}

impl Pls {
	/// Run `pls`.
	///
	/// This is the entrypoint of the `Pls` class, and once control is passed
	/// to it from `main`, it handles everything.
	///
	/// The primary function of this method is to organise the input list of
	/// paths into groups and then delegate to each group the job of listing
	/// their entries and rendering the layout.
	pub fn run(&self) {
		let inputs: Vec<_> = self
			.args
			.paths
			.iter()
			.filter_map(|path| {
				let input = Input::new(path, &self.conf_man);
				match input {
					Ok(input) => Some(input),
					Err(exc) => {
						let loc = render(format!("<bold>{}</>", path.display()));
						println!("{loc}:");
						println!("\t{exc}");
						None
					}
				}
			})
			.collect();

		let show_title = self.args.paths.len() > 1;
		let groups = Group::partition(inputs, &self.conf_man);

		groups
			.iter()
			.map(|group| group.render(show_title, &mut OwnerMan::default()))
			.filter_map(|res| res.err())
			.for_each(|res| println!("{res}"));
	}
}
