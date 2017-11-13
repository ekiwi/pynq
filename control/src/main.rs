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
	let mut tx = pynq::DmaBuffer::allocate(20 * 8);
	let rx = pynq::DmaBuffer::allocate(10 * 8);
	let buffer_id : u64 = 0x0abcdef0;
	{
		let tx_data = tx.as_slice_u64_mut();
		tx_data[0] = 0x19931993 << 32 | buffer_id;
		tx_data[1] = 3 << 48 | 3 << 32;
		let d0 : u64 = (400 << 32) | 100;
		let d1 : u64 = 1 << 63 | 1 << 62;
		for ii in 1..10 {
			tx_data[ii * 2 + 0] = d0;
			tx_data[ii * 2 + 1] = d1;
		}
	}
	let mut dma = pynq::Dma::get();
	dma.start_send(tx);
	dma.start_receive(rx);
	while !(dma.is_send_done() && dma.is_receive_done()) {}
	let _ = dma.finish_send();
	let rx_back = dma.finish_receive();
	let rx_back_data = rx_back.as_slice_u64();
	assert_eq!(rx_back_data[0], 0x73537353 << 32 | buffer_id);
	let cov0 = 0x0300030003000303;
	let cov1 = 0x0000030000000000;
	for ii in 0..3 {
		assert_eq!(rx_back_data[1 + ii * 2 + 0], cov0);
		assert_eq!(rx_back_data[1 + ii * 2 + 1], cov1);
	}
	println!("Simple DMA fuzz test 👌");

	// wait for leds to finish blinking
	let _ = child.join();
}
