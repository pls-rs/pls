use crate::exc::Exc;
use crossterm::terminal::*;

/// Perform the given query in the terminal raw mode.
///
/// This function enables the terminal raw mode, performs the query,
/// records the response and then disables the terminal raw mode. The
/// response is returned as a string.
///
/// # Arguments
///
/// * `query` - the query to perform
/// * `timeout_ms` - the timeout in milliseconds
pub(crate) fn query_raw(query: &str, timeout_ms: u64) -> Result<String, Exc> {
	enable_raw_mode().map_err(Exc::Io)?;
	let res = xterm_query::query_osc(query, timeout_ms).map_err(|e| Exc::Xterm(Box::new(e)));
	disable_raw_mode().map_err(Exc::Io)?;

	res
}
