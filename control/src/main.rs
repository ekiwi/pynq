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
	pynq::load_bitstream("system.bit", &[pynq::Clock{ div0: 5, div1: 2 }]).unwrap();

	let child = std::thread::spawn(blink_leds);
	let _ = child.join();
}
