mod pynq;
use pynq::Color;

fn blink_leds() {
	let mut leds = pynq::RgbLeds::get();
	for _ in 0..10 {
		leds.set(Color::Black, Color::Red);
		std::thread::sleep(std::time::Duration::from_millis(200));
		leds.set(Color::Blue, Color::Black);
		std::thread::sleep(std::time::Duration::from_millis(200));
	}
}

// fn test_dma(dma : &mut pynq::Dma, length : usize) {
	// TODO!



// 	println!("")
// }


fn main() {
	pynq::load_bitstream("system.bit", &[pynq::Clock{ div0: 5, div1: 2 }]).unwrap();

	// start Blinkenlights!
	let child = std::thread::spawn(blink_leds);

	// try dma
	let mut tx = pynq::DmaBuffer::allocate(0x100);
	let mut rx = pynq::DmaBuffer::allocate(0x100);
	println!("allocated DMA buffers");
	let mut dma = pynq::Dma::get();
	dma.start_send(tx);
	println!("started sending");
	dma.start_receive(rx);
	println!("started receiving");
	println!("waiting for sending and receiving to be done ....");
	while !(dma.is_send_done() && dma.is_receive_done()) {}
	println!("DONE!!!!!");
	let _ = dma.finish_send();
	let _ = dma.finish_receive();


	let _ = child.join();
}
