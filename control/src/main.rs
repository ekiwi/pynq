extern crate time;

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

fn main() {
	pynq::load_bitstream("system.bit", &[pynq::Clock{ div0: 5, div1: 2 }]).unwrap();

	// start Blinkenlights!
	let child = std::thread::spawn(blink_leds);


	// settings
	let test_count = 10000;
	let cycle_count = 3;

	// resulting
	let tx_words = 2 + test_count * (cycle_count * 2);
	let tx_bytes = tx_words * 8;
	let rx_words = 2 + test_count * 2;
	let rx_bytes = rx_words * 8;

	// anounce test
	println!("Testing the fuzz harness on FPGA:");
	println!("Test Count: {}", test_count);
	println!("Cycles per Test: {}", cycle_count);
	println!("TX: {} KiB", tx_bytes / 1024);
	println!("RX: {} KiB", rx_bytes / 1024);

	// try dma
	let start = time::PreciseTime::now();

	let mut tx = pynq::DmaBuffer::allocate(tx_bytes);
	let rx = pynq::DmaBuffer::allocate(rx_bytes);
	let buffer_id : u64 = 0x0abcdef0;
	{
		let tx_data = tx.as_slice_u64_mut();
		tx_data[0] = 0x19931993 << 32 | buffer_id;
		//tx_data[1] = (test_count as u64) << 48 | (cycle_count as u64) << 32;
		tx_data[1] = (cycle_count as u64) << 48 | (test_count as u64) << 32;
		let d0 : u64 = (400 << 32) | 100;
		let d1 : u64 = 1 << 63 | 1 << 62;
		let total_cycles = test_count * cycle_count;
		for ii in 0..total_cycles {
			tx_data[2 + ii * 2 + 0] = d0;
			tx_data[2 + ii * 2 + 1] = d1;
		}
	}

	let start_dma = time::PreciseTime::now();
	let mut dma = pynq::Dma::get();
	dma.start_send(tx);
	dma.start_receive(rx);
	while !(dma.is_send_done() && dma.is_receive_done()) {}
	let duration_dma = start_dma.to(time::PreciseTime::now()).num_microseconds().unwrap();

	let _ = dma.finish_send();
	let rx_back = dma.finish_receive();
	let rx_back_data = rx_back.as_slice_u64();
	assert_eq!(rx_back_data[0], 0x73537353 << 32 | buffer_id);
	let cov0 = 0x0300030003000303;
	let cov1 = 0x0000030000000000;
	for ii in 0..test_count {
		assert_eq!(rx_back_data[1 + ii * 2 + 0], cov0);
		assert_eq!(rx_back_data[1 + ii * 2 + 1], cov1);
	}
	assert_eq!(rx_back_data[rx_words - 1], 0);

	let duration = start.to(time::PreciseTime::now()).num_microseconds().unwrap();

	println!();
	println!("Test Results Correct?: 👌");
	let exec_per_second = ((test_count as u64) * 1000 * 1000) as f64 / duration_dma as f64;
	println!("DMA only:");
	println!("Execution Rate: {:.1} tests/s", exec_per_second);
	println!("Execution Time: {} us", duration_dma);
	let runs_per_second = ((test_count as u64) * 1000 * 1000) as f64 / duration as f64;
	println!("Writing inputs and reading results + DMA:");
	println!("Test Rate: {:.1} tests/s", runs_per_second);
	println!("Total Time: {} us", duration);

}
