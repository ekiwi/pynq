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
	let mut tx = pynq::DmaBuffer::allocate(100);
	let mut rx = pynq::DmaBuffer::allocate(100);
	{
		let tx_data = tx.as_slice_mut();
		let rx_data = rx.as_slice_mut();
		for ii in 0..100 {
			tx_data[ii] = ii as u8;
			rx_data[ii] = 255 - ii as u8;
		}
	}
	let mut dma = pynq::Dma::get();
	dma.start_send(tx);
	dma.start_receive(rx);
	while !(dma.is_send_done() && dma.is_receive_done()) {}
	let _ = dma.finish_send();
	let rx_back = dma.finish_receive();
	let rx_back_data = rx_back.as_slice();
	for ii in 0..100 {
		assert!(rx_back_data[ii] == ii as u8, "Unexpected return value!");
	}
	println!("Simple DMA loopback test 👌");

	// wait for leds to finish blinking
	let _ = child.join();
}
