use serde::{Deserialize, Serialize};

/// This enum contains different unit systems to express large numbers,
/// specifically node sizes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitSys {
	Binary,  // higher units based on powers of 2^10
	Decimal, // higher units based on powers of 10^3
	None,    // no higher units
}
