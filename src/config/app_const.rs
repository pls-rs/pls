use crate::enums::DetailField;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AppConst {
	/// configuration for the table view
	pub table: TableInfo,
	/// shapes to use to print trees
	pub tree: TreeInfo,
	/// pairings of importance levels with styling directives
	pub imp_styles: Vec<(i8, String)>,

	/// mapping of importance levels to styling directives, derived from `imp`
	#[serde(skip)]
	pub imp_map: HashMap<i8, String>,
}

#[derive(Serialize, Deserialize)]
pub struct TableInfo {
	/// mapping of detail field to column name
	pub column_names: HashMap<DetailField, String>,
	/// the styles to apply to the text in the header row
	pub header_style: String,
}

#[derive(Serialize, Deserialize)]
pub struct TreeInfo {
	/// "│  "
	pub pipe_space: String,
	/// "   "
	pub space_space: String,
	/// "├─ "
	pub tee_dash: String,
	/// "└─ "
	pub bend_dash: String,
}

impl Default for AppConst {
	fn default() -> Self {
		Self {
			table: TableInfo {
				header_style: String::from("bold italic underline"),
				column_names: [
					(DetailField::Dev, "Device"),
					(DetailField::Ino, "inode"),
					(DetailField::Nlink, "Link#"),
					(DetailField::Typ, "T"),
					(DetailField::Perm, "Permissions"),
					(DetailField::Oct, "SUGO"),
					(DetailField::User, "User"),
					(DetailField::Uid, "UID"),
					(DetailField::Group, "Group"),
					(DetailField::Gid, "GID"),
					(DetailField::Size, "Size"),
					(DetailField::Blocks, "Blocks"),
					(DetailField::Btime, "Created"),
					(DetailField::Ctime, "Changed"),
					(DetailField::Mtime, "Modified"),
					(DetailField::Atime, "Accessed"),
					(DetailField::Git, "Git"),
					(DetailField::Name, "Name"),
				]
				.into_iter()
				.map(|(k, v)| (k, v.to_string()))
				.collect(),
			},
			tree: TreeInfo {
				pipe_space: String::from("│  "),
				space_space: String::from("   "),
				tee_dash: String::from("├─ "),
				bend_dash: String::from("└─ "),
			},
			imp_styles: [(-1, "dimmed"), (1, "italic"), (2, "underline")]
				.into_iter()
				.map(|(k, v)| (k, v.to_string()))
				.collect(),

			imp_map: HashMap::new(), // set in Constants::set_imp_map
		}
	}
}

impl AppConst {
	/// Set `imp_map` from `imp` and then use it to clean the latter.
	///
	/// If a level has been defined multiple times, only the last definition
	/// will be retained in `imp_map`. The levels will be sorted by importance
	/// in `imp`.
	pub fn massage_imps(&mut self) {
		self.imp_map = self
			.imp_styles
			.iter()
			.map(|(k, v)| (*k, v.to_string()))
			.collect();

		self.imp_styles = self.imp_map.clone().into_iter().collect();
		self.imp_styles.sort_by_cached_key(|entry| entry.0);
	}

	/// Get the lowest configured importance level, i.e. zeroth index in `imp`.
	pub fn min_imp(&self) -> i8 {
		self.get_imp(0)
	}

	/// Get the highest configured importance level, i.e. last index in `imp`.
	pub fn max_imp(&self) -> i8 {
		self.get_imp(self.imp_styles.len() - 1)
	}

	/// Get the importance level at the given index.
	///
	/// This returns 0 (default for `i8`) if the `imp` vector does not have the
	/// given index.
	fn get_imp(&self, idx: usize) -> i8 {
		self.imp_styles
			.get(idx)
			.map(|row| row.0)
			.unwrap_or_default()
	}
}

#[cfg(test)]
mod tests {
	use super::AppConst;
	use std::collections::HashMap;

	macro_rules! make_massage_imps_test {
        ( $($name:ident: $imp:expr => $exp_imp:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let imp_styles: Vec<(i8, String)> = $imp
                        .into_iter()
                        .map(|(k, v): (i8, &str)| (k, v.to_string()))
                        .collect();
                    let exp_imp: Vec<(i8, String)> = $exp_imp
                        .into_iter()
                        .map(|(k, v): (i8, &str)| (k, v.to_string()))
                        .collect();
                    let exp_imp_map: HashMap<i8, String> = $exp_imp
                        .into_iter()
                        .map(|(k, v): (i8, &str)| (k, v.to_string()))
                        .collect();

                    let mut app_const = AppConst {
                        imp_styles,
                        ..AppConst::default()
                    };
                    app_const.massage_imps();
                    assert_eq!(app_const.imp_styles, exp_imp);
                    assert_eq!(app_const.imp_map, exp_imp_map);
                }
            )*
        }
    }

	make_massage_imps_test!(
		test_empty: vec![] => vec![],
		test_sorting: vec![
			(2, "underline"),
			(1, "italic"),
		] => vec![(1, "italic"), (2, "underline")],
		test_deduplication: vec![
			(2, "bold"),
			(2, "dimmed"),
			(1, "reversed"),
			(2, "underline"),
			(1, "italic"),
		] => vec![(1, "italic"), (2, "underline")],
	);
}
