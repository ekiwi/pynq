extern crate libc;
use std::ffi::CString;

fn main() {
	let msg = CString::new("Hello, world!\n").unwrap();
	unsafe { libc::write(0, msg.as_ptr() as *const libc::c_void, 14); }
}
