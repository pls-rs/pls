use libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use log::warn;

/// See http://www.delorie.com/djgpp/doc/libc/libc_495.html
#[repr(C)]
#[derive(Default)]
pub struct Window {
	pub ws_row: c_ushort,    /* rows, in characters */
	pub ws_col: c_ushort,    /* columns, in characters */
	pub ws_xpixel: c_ushort, /* horizontal size, pixels */
	pub ws_ypixel: c_ushort, /* vertical size, pixels */
}

impl Window {
	/// Get a new `Window` instance with the terminal measurements.
	///
	/// This function returns `None` if the ioctl call fails.
	pub fn try_new() -> Option<Self> {
		let mut win = Self::default();
		#[allow(clippy::useless_conversion)]
		let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut win) };
		if r == 0 && win.ws_row > 0 && win.ws_col > 0 {
			return Some(win);
		}
		warn!("Could not determine cell dimensions.");
		None
	}

	pub fn cell_width(&self) -> u8 {
		(self.ws_xpixel / self.ws_col) as u8
	}

	pub fn cell_height(&self) -> u8 {
		(self.ws_ypixel / self.ws_row) as u8
	}
}
