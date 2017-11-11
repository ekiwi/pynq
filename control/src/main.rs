mod pynq;
use pynq::Color;

pub fn blink_leds() {
	let mut leds = pynq::RgbLeds::get();
	for _ in 0..10 {
		leds.set(Color::Black, Color::Red);
		std::thread::sleep(std::time::Duration::from_millis(200));
		leds.set(Color::Blue, Color::Black);
		std::thread::sleep(std::time::Duration::from_millis(200));
	}
}


fn main() {
	//let msg = CString::new("Hello, world!\n").unwrap();
	//unsafe { libc::write(0, msg.as_ptr() as *const libc::c_void, 14); }

	let child = std::thread::spawn(blink_leds);
	let _ = child.join();
}
