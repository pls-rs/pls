use crate::config::Conf;
use crate::enums::{Oct, Sym};
use std::collections::HashMap;

/// Represents node permissions found on UNIX-style operating systems. It can
/// account for special permissions i.e. setuid, setgid and sticky bit.
///
/// An instance of this struct can be created using `into` on the node's mode,
/// which is type [`u32`].
pub struct Perm {
	pub mode: u32,
	pub perm_map: HashMap<(Oct, Sym), bool>,
}

impl From<u32> for Perm {
	fn from(mode: u32) -> Self {
		let has_bit = |bit| mode & bit == bit;

		let octs = [Oct::Other, Oct::Group, Oct::User];
		let perms = [Sym::Execute, Sym::Write, Sym::Read, Sym::Special];

		let perm_map = octs
			.iter()
			.enumerate()
			.flat_map(|(oct_idx, oct)| {
				perms.iter().enumerate().map(move |(perm_idx, perm)| {
					let bit: u32 = if perm == &Sym::Special {
						2_u32.pow(oct_idx as u32) * 0o10_u32.pow(3)
					} else {
						2_u32.pow(perm_idx as u32) * 0o10_u32.pow(oct_idx as u32)
					};
					((*oct, *perm), has_bit(bit))
				})
			})
			.collect();

		Self { mode, perm_map }
	}
}

impl Perm {
	/// Get the symbol character for read and write permissions.
	///
	/// This function returns a marked-up string.
	fn perm_ch(&self, oct: Oct, perm: Sym, conf: &Conf) -> String {
		let has_perm = self.perm_map[&(oct, perm)];
		if has_perm {
			perm.ch(conf)
		} else {
			Sym::None.ch(conf)
		}
	}

	/// Get the symbol for the combined execute and special permissions.
	///
	/// This function returns a marked-up string.
	fn xs_perm_ch(&self, oct: Oct, conf: &Conf) -> String {
		let has_exec = self.perm_map[&(oct, Sym::Execute)];
		let has_special = self.perm_map[&(oct, Sym::Special)];
		if has_special {
			Sym::Special.special_ch(oct, has_exec, conf)
		} else if has_exec {
			Sym::Execute.ch(conf)
		} else {
			Sym::None.ch(conf)
		}
	}

	/* Renderables */
	/* =========== */

	/// Render the permissions in symbolic representation.
	///
	/// This function returns a marked-up string.
	pub fn sym(&self, conf: &Conf) -> String {
		[Oct::User, Oct::Group, Oct::Other]
			.into_iter()
			.map(|oct| {
				[
					self.perm_ch(oct, Sym::Read, conf),
					self.perm_ch(oct, Sym::Write, conf),
					self.xs_perm_ch(oct, conf),
				]
				.join("")
			})
			.collect::<Vec<_>>()
			.join(" ")
	}

	/// Render the permissions in octal representation.
	///
	/// This function returns a marked-up string.
	pub fn oct(&self, conf: &Conf) -> String {
		format!("{:04o}", self.mode % 0o10000)
			.chars()
			.zip([Oct::Special, Oct::User, Oct::Group, Oct::Other])
			.map(|(ch, oct)| match (oct, ch) {
				(Oct::Special, '0') => String::from(" "),
				_ => {
					let directives = conf.constants.oct_styles.get(&oct).unwrap();
					format!("<{directives}>{ch}</>")
				}
			})
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::Perm;
	use crate::config::Conf;

	macro_rules! make_renderables_test {
		( $($name:ident: $mode:expr => $expected_sym:expr, $expected_oct:expr,)* ) => {
			$(
				#[test]
				fn $name() {
                    let conf = Conf::default();
					let perm: Perm = $mode.into();
					assert_eq!(perm.sym(&conf), String::from($expected_sym));
					assert_eq!(perm.oct(&conf), String::from($expected_oct));
				}
			)*
		};
	}

	make_renderables_test!(
		test_1000: 0o1000 =>
			"<dimmed>-</><dimmed>-</><dimmed>-</> <dimmed>-</><dimmed>-</><dimmed>-</> <dimmed>-</><dimmed>-</><magenta>T</>",
			"<magenta>1</><blue>0</><blue dimmed>0</><dimmed>0</>",
		test_777:  0o777  =>
			"<yellow>r</><red>w</><green>x</> <yellow>r</><red>w</><green>x</> <yellow>r</><red>w</><green>x</>",
			" <blue>7</><blue dimmed>7</><dimmed>7</>",
		test_654:  0o654  =>
			"<yellow>r</><red>w</><dimmed>-</> <yellow>r</><dimmed>-</><green>x</> <yellow>r</><dimmed>-</><dimmed>-</>",
			" <blue>6</><blue dimmed>5</><dimmed>4</>",
		test_321:  0o321  =>
			"<dimmed>-</><red>w</><green>x</> <dimmed>-</><red>w</><dimmed>-</> <dimmed>-</><dimmed>-</><green>x</>",
			" <blue>3</><blue dimmed>2</><dimmed>1</>",
		test_000:  0o000  =>
			"<dimmed>-</><dimmed>-</><dimmed>-</> <dimmed>-</><dimmed>-</><dimmed>-</> <dimmed>-</><dimmed>-</><dimmed>-</>",
			" <blue>0</><blue dimmed>0</><dimmed>0</>",

		test_1666: 0o1666 =>
			"<yellow>r</><red>w</><dimmed>-</> <yellow>r</><red>w</><dimmed>-</> <yellow>r</><red>w</><magenta>T</>",
			"<magenta>1</><blue>6</><blue dimmed>6</><dimmed>6</>",
		test_2666: 0o2666 =>
			"<yellow>r</><red>w</><dimmed>-</> <yellow>r</><red>w</><magenta>S</> <yellow>r</><red>w</><dimmed>-</>",
			"<magenta>2</><blue>6</><blue dimmed>6</><dimmed>6</>",
		test_4666: 0o4666 =>
			"<yellow>r</><red>w</><magenta>S</> <yellow>r</><red>w</><dimmed>-</> <yellow>r</><red>w</><dimmed>-</>",
			"<magenta>4</><blue>6</><blue dimmed>6</><dimmed>6</>",
		test_7666: 0o7666 =>
			"<yellow>r</><red>w</><magenta>S</> <yellow>r</><red>w</><magenta>S</> <yellow>r</><red>w</><magenta>T</>",
			"<magenta>7</><blue>6</><blue dimmed>6</><dimmed>6</>",
		test_1777: 0o1777 =>
			"<yellow>r</><red>w</><green>x</> <yellow>r</><red>w</><green>x</> <yellow>r</><red>w</><magenta>t</>",
			"<magenta>1</><blue>7</><blue dimmed>7</><dimmed>7</>",
		test_2777: 0o2777 =>
			"<yellow>r</><red>w</><green>x</> <yellow>r</><red>w</><magenta>s</> <yellow>r</><red>w</><green>x</>",
			"<magenta>2</><blue>7</><blue dimmed>7</><dimmed>7</>",
		test_4777: 0o4777 =>
			"<yellow>r</><red>w</><magenta>s</> <yellow>r</><red>w</><green>x</> <yellow>r</><red>w</><green>x</>",
			"<magenta>4</><blue>7</><blue dimmed>7</><dimmed>7</>",
		test_7777: 0o7777 =>
			"<yellow>r</><red>w</><magenta>s</> <yellow>r</><red>w</><magenta>s</> <yellow>r</><red>w</><magenta>t</>",
			"<magenta>7</><blue>7</><blue dimmed>7</><dimmed>7</>",
	);
}
