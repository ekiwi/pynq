// simple rust interface to some of the PYNQ peripherals

extern crate libc;
use std;
use std::ffi::CString;
use std::ops::{Index, IndexMut, Drop};

#[derive(Copy, Clone, Debug)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Yellow = 6,
	White = 7,
}

pub struct RgbLeds {
	mem : MemoryMappedIO,
}

impl RgbLeds {
	pub fn get() -> Self {
		let mut mem = MemoryMappedIO::map(0x41210000, 8);
		// configure lowest 6 gpios as output
		mem[1] = !((7 << 3) | 7);
		RgbLeds { mem }
	}
	pub fn set(&mut self, ld4_color : Color, ld5_color : Color) {
		self.mem[0] = (ld4_color as u32 & 7) | ((ld5_color as u32 & 7) << 3);
	}
	pub fn set_ld4(&mut self, color : Color) {
		let old = self.mem[0];
		self.mem[0] = (old & !7) | ((color as u32) & 7);
	}
	pub fn set_ld5(&mut self, color : Color) {
		let old = self.mem[0];
		self.mem[0] = (old & !(7 << 3)) | (((color as u32) & 7) << 3);
	}
}
impl Drop for RgbLeds {
	fn drop(&mut self) {
		// reset to all inputs
		self.mem[1] = !0u32;
	}
}

struct MemoryMappedIO {
	mem : *mut u32,
	words : usize,
}

impl MemoryMappedIO {
	fn map(phys_addr : u32, length : u32) -> Self {
		let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u32;
		assert!(phys_addr % page_size == 0, "Only page boundary aligned IO is supported!");
		let phys_mem = CString::new("/dev/mem").unwrap();
		let words = ((length + 3) / 4) as usize;
		let mem = unsafe {
			let fd = libc::open(phys_mem.as_ptr(), libc::O_RDWR | libc::O_SYNC);
			assert!(fd > -1, "Failed to open /dev/mem. Are we root?");
			let mm = libc::mmap(std::ptr::null_mut(), words * 4,
			                    libc::PROT_READ | libc::PROT_WRITE,
			                    libc::MAP_SHARED, fd, phys_addr as libc::c_long);
			assert!(mm != libc::MAP_FAILED, "Failed to mmap physical memory.");
			assert!(libc::close(fd) == 0, "Failed to close /dev/mem.");
			mm as *mut u32
		};
		MemoryMappedIO { mem, words }
	}
}

impl Drop for MemoryMappedIO {
	fn drop(&mut self) {
		unsafe {
			assert!(
				libc::munmap(self.mem as *mut libc::c_void, self.words * 4) == 0,
				"Failed to unmap IO.");
		}
	}
}

impl Index<usize> for MemoryMappedIO {
	type Output = u32;
	fn index(&self, ii : usize) -> &u32 {
		unsafe { &std::slice::from_raw_parts(self.mem, self.words)[ii] }
	}
}
impl IndexMut<usize> for MemoryMappedIO {
	fn index_mut(&mut self, ii : usize) -> &mut u32 {
		unsafe { &mut std::slice::from_raw_parts_mut(self.mem, self.words)[ii] }
	}
}

