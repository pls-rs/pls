use crate::config::Conf;
use clap::ValueEnum;
use number_prefix::NumberPrefix;
use serde::{Deserialize, Serialize};

/// This enum contains different unit systems to express large numbers,
/// specifically node sizes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum UnitSys {
	Binary,  // higher units based on powers of 2^10
	Decimal, // higher units based on powers of 10^3
	None,    // no higher units
}

impl UnitSys {
	/// Split a natural number into a fractional magnitude and a unit prefix.
	/// This method should not be invoked on enum variant `UnitSys::None`.
	///
	/// # Arguments
	///
	/// * `size` - the natural number to split into magnitude and unit
	///
	/// # Returns
	///
	/// * the length of the prefix
	/// * the fractional magnitude
	/// * the prefix of the unit
	fn convert(&self, size: u64) -> (usize, f64, &'static str) {
		let size = size as f64;
		let (len, prefixed) = match self {
			UnitSys::Binary => (2, NumberPrefix::binary(size)),
			UnitSys::Decimal => (1, NumberPrefix::decimal(size)),
			_ => panic!("UnitSys::None cannot be converted."),
		};
		let (mag, prefix) = match prefixed {
			NumberPrefix::Standalone(mag) => (mag, ""),
			NumberPrefix::Prefixed(prefix, mag) => (mag, prefix.symbol()),
		};
		(len, mag, prefix)
	}

	/// Convert the given number of bytes to a size string that uses the
	/// preferred unit system.
	///
	/// This function returns a marked-up string.
	pub fn size(&self, size: u64, conf: &Conf) -> String {
		let mag_directive = &conf.constants.size_styles.mag;
		let base_directive = &conf.constants.size_styles.base;

		if self == &UnitSys::None {
			return format!("<{mag_directive}>{size}</> <{base_directive}>B</>");
		}

		let prefix_directive = &conf.constants.size_styles.prefix;

		let (width, mag, prefix) = self.convert(size);
		format!(
			"<{mag_directive}>{mag:.1}</> \
			 <{prefix_directive}>{prefix:>width$}</>\
			 <{base_directive}>B</>",
			width = width
		)
	}
}

#[cfg(test)]
mod tests {
	use super::UnitSys;
	use crate::config::Conf;

	macro_rules! make_test {
		( $($name:ident: $unit:expr, $num:expr => $str:expr,)* ) => {
			$(
				#[test]
				fn $name() {
                    let conf = Conf::default();
					let text = $unit.size($num, &conf);
					assert_eq!(text, $str);
				}
			)*
		};
	}

	make_test!(
		none_shows_no_unit_for_base: UnitSys::None, 617             =>        "<bold>617</> <dimmed>B</>",
		none_shows_no_unit_for_pow1: UnitSys::None, 1234_u64.pow(1) =>       "<bold>1234</> <dimmed>B</>",
		none_shows_no_unit_for_pow2: UnitSys::None, 1234_u64.pow(2) =>    "<bold>1522756</> <dimmed>B</>",
		none_shows_no_unit_for_pow3: UnitSys::None, 1234_u64.pow(3) => "<bold>1879080904</> <dimmed>B</>",

		binary_shows_no_unit_for_base: UnitSys::Binary, 512             => "<bold>512.0</> <>  </><dimmed>B</>",
		binary_shows_ki_unit_for_pow1: UnitSys::Binary, 1024_u64.pow(1) => "<bold>1.0</> <>Ki</><dimmed>B</>",
		binary_shows_mi_unit_for_pow2: UnitSys::Binary, 1024_u64.pow(2) => "<bold>1.0</> <>Mi</><dimmed>B</>",
		binary_shows_gi_unit_for_pow3: UnitSys::Binary, 1024_u64.pow(3) => "<bold>1.0</> <>Gi</><dimmed>B</>",

		decimal_shows_no_unit_for_base: UnitSys::Decimal, 500             => "<bold>500.0</> <> </><dimmed>B</>",
		decimal_shows_k_unit_for_pow1:  UnitSys::Decimal, 1000_u64.pow(1) => "<bold>1.0</> <>k</><dimmed>B</>",
		decimal_shows_m_unit_for_pow2:  UnitSys::Decimal, 1000_u64.pow(2) => "<bold>1.0</> <>M</><dimmed>B</>",
		decimal_shows_g_unit_for_pow3:  UnitSys::Decimal, 1000_u64.pow(3) => "<bold>1.0</> <>G</><dimmed>B</>",
	);
}
