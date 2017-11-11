// simple rust interface to some of the PYNQ peripherals

extern crate libc;
use std;
use std::ffi::CString;
use std::ops::{Index, IndexMut, Drop};
use std::io::{Read, Write};
use std::fs::File;

#[derive(Copy, Clone, Debug)]
pub struct Clock { pub div0 : u32, pub div1 : u32 }

pub fn load_bitstream(filename: &str, clocks: &[Clock]) -> Result<usize, String> {
	let mut buf = Vec::new();
	load_bitstream_data(filename, &mut buf);
	configure_clocks(clocks);
	set_partial_bitstream(false);
	write_bitstream_data(&buf);
	Ok(buf.len())
}

fn configure_clocks(clocks: &[Clock]) {
	assert!(clocks.len() >  0, "Need to enable at least 1 clock!");
	assert!(clocks.len() <= 4, "Only four clocks (FCLK{0-3}) can be configured!");
	let disabled = Clock { div0 : 0, div1: 0 };
	let base_addr = 0xf8000000;
	fn offset(ii : usize) -> usize { (0x170 / 4) + (ii * (0x10 / 4)) }
	let mut mem = MemoryMappedIO::map(base_addr, 0x170 + 0x10 * 4);
	for (ii, clk) in clocks.iter().enumerate() {
		mem[offset(ii)] = calc_divs(clk, mem[offset(ii)]);
	}
	for ii in clocks.len()..4 {
		mem[offset(ii)] = calc_divs(&disabled, mem[offset(ii)]);
	}
}

fn calc_divs(clk: &Clock, old : u32) -> u32 {
	(old & !((0x3f << 20) | (0x3f << 8))) |
	(((clk.div1 & 0x3f) << 20) | ((clk.div0 & 0x3f) << 8))
}

fn set_partial_bitstream(enabled : bool) {
	let partial_bitstream = "/sys/devices/soc0/amba/f8007000.devcfg/is_partial_bitstream";
	let mut file = File::create(partial_bitstream).expect("Failed to open partial bitstream file!");
	file.write(if enabled { b"1" } else { b"0" }).unwrap();
}

fn load_bitstream_data(filename : &str, buf: &mut Vec<u8>) {
	let mut file = File::open(filename).expect("Failed to open bitstream file!");
	file.read_to_end(buf).expect("Failed to read bitstream file!");
}

fn write_bitstream_data(buf : &[u8]) {
	let mut file = File::create("/dev/xdevcfg").unwrap();
	file.write_all(buf).expect("Failed to write bitstream to FPGA");
}

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

