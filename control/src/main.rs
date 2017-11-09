extern crate libc;
use std::ffi::CString;

fn blink_leds() {
	// map memory for rgb leds
	let rgbleds_gpio_addr = 0x41210000 as u32;
	let length = 0x10000 as u32;
	let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u32;
	let base_addr = rgbleds_gpio_addr & !page_size;
	let offset = rgbleds_gpio_addr - base_addr;
	println!("offset: {}", offset);
	let phys_mem = CString::new("/dev/mem").unwrap();
	let mem = unsafe {
	let fd = libc::open(phys_mem.as_ptr(), libc::O_RDWR | libc::O_SYNC);
	assert!(fd > -1, "Failed to open /dev/mem. Are we root?");
	let mm = libc::mmap(std::ptr::null_mut(), (length + offset) as usize,
	                     libc::PROT_READ | libc::PROT_WRITE,
	                     libc::MAP_SHARED, fd, base_addr as i32);
	assert!(mm != libc::MAP_FAILED, "Failed to mmap physical memory.");
	std::slice::from_raw_parts_mut(mm.offset(offset as isize) as *mut u32, length as usize)
	};
	let ld4 = 1 << 2;
	let ld5 = 1 << 0;
	loop {
		mem[0] = (ld5 & 7) << 3;
		std::thread::sleep(std::time::Duration::from_millis(200));
		mem[0] = (ld4 & 7);
		std::thread::sleep(std::time::Duration::from_millis(200));
	}
}


fn main() {
	let msg = CString::new("Hello, world!\n").unwrap();
	unsafe { libc::write(0, msg.as_ptr() as *const libc::c_void, 14); }

	let child = std::thread::spawn(blink_leds);
	let _ = child.join();
}
