extern crate time;

mod pynq;
use pynq::Color;


pub trait ReadInts : std::io::Read {
	fn read_u16(&mut self) -> std::io::Result<u16> {
		let mut data: [u8; 2] = [0; 2];
		self.read_exact(&mut data)?;
		let val = (data[1] as u16) <<  8 | (data[0] as u16) <<  0;
		Ok(val)
	}

	fn read_u32(&mut self) -> std::io::Result<u32> {
		let mut data: [u8; 4] = [0; 4];
		self.read_exact(&mut data)?;
		let val = (data[3] as u32) << 24 | (data[2] as u32) << 16 |
		          (data[1] as u32) <<  8 | (data[0] as u32) <<  0;
		Ok(val)
	}

	fn read_u64(&mut self) -> std::io::Result<u64> {
		let mut data: [u8; 8] = [0; 8];
		self.read_exact(&mut data)?;
		let val = (data[7] as u64) << 56 | (data[6] as u64) << 48 |
		          (data[5] as u64) << 40 | (data[4] as u64) << 32 |
		          (data[3] as u64) << 24 | (data[2] as u64) << 16 |
		          (data[1] as u64) <<  8 | (data[0] as u64) <<  0;
		Ok(val)
	}
}

pub trait WriteInts : std::io::Write {
	fn write_u16(&mut self, val: u16) -> std::io::Result<()> {
		let data = [(val >>  0) as u8, (val >>  8) as u8];
		self.write_all(&data)
	}

	fn write_u32(&mut self, val: u32) -> std::io::Result<()> {
		let data = [(val >>  0) as u8, (val >>  8) as u8,
		            (val >> 16) as u8, (val >> 24) as u8];
		self.write_all(&data)
	}

	fn write_u64(&mut self, val: u64) -> std::io::Result<()> {
		let data = [(val >>  0) as u8, (val >>  8) as u8,
		            (val >> 16) as u8, (val >> 24) as u8,
		            (val >> 32) as u8, (val >> 40) as u8,
		            (val >> 48) as u8, (val >> 56) as u8];
		self.write_all(&data)
	}
}

impl ReadInts for pynq::DmaBuffer {}
impl WriteInts for pynq::DmaBuffer {}

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
	let mut rx = pynq::DmaBuffer::allocate(rx_bytes);
	let buffer_id : u64 = 0x0abcdef0;
	{
		tx.write_u64(0x19931993 << 32 | buffer_id).unwrap();
		tx.write_u64((test_count as u64) << 48 | (cycle_count as u64) << 32).unwrap();
		let d0 : u64 = (400 << 32) | 100;
		let d1 : u64 = 1 << 63 | 1 << 62;
		let total_cycles = test_count * cycle_count;
		for _ in 0..total_cycles {
			tx.write_u64(d0).unwrap();
			tx.write_u64(d1).unwrap();
		}
	}

	let start_dma = time::PreciseTime::now();
	let mut dma = pynq::Dma::get();
	dma.start_send(tx.id());
	dma.start_receive(rx.id());
	while !(dma.is_send_done() && dma.is_receive_done()) {}
	let duration_dma = start_dma.to(time::PreciseTime::now()).num_microseconds().unwrap();

	let _ = dma.finish_send();
	let _ = dma.finish_receive();
	assert_eq!(rx.read_u64().unwrap(), 0x73537353 << 32 | buffer_id);
	let cov0 = 0x0300030003000303;
	let cov1 = 0x0000030000000000;
	for _ in 0..test_count {
		assert_eq!(rx.read_u64().unwrap(), cov0);
		assert_eq!(rx.read_u64().unwrap(), cov1);
	}
	assert_eq!(rx.read_u64().unwrap(), 0);

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
