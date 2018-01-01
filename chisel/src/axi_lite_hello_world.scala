package pynq

import chisel3._
import chisel3.util._

class AxiLiteFollower extends Bundle {
	val data_bits = 32
	val address_bits = 8
	val words = (1 << (address_bits - 2))
	// TODO: maybe use ReadyValid interface from stdlib
	// read address interface
	val araddr  = Input(UInt(address_bits.W))
	val arready = Output(Bool())
	val arvalid = Input(Bool())
	// write address interface
	val awaddr  = Input(UInt(address_bits.W))
	val awready = Output(Bool())
	val awvalid = Input(Bool())
	// write response interface
	val bresp  = Output(UInt(2.W))
	val bready = Input(Bool())
	val bvalid = Output(Bool())
	// read data interface
	val rdata = Output(UInt(data_bits.W))
	val rresp = Output(UInt(2.W))
	val rready = Input(Bool())
	val rvalid = Output(Bool())
	// write data interface
	val wdata = Input(UInt(data_bits.W))
	val wready = Output(Bool())
	val wvalid = Input(Bool())
}

// This is the simplest AxiLite Follower I could think of.
// It does not accept writes and always returns a single constant, no matter what
// address is read from.
//
// Test from `ipython3` like this:
// ```
// > "0x{:2x}".format(ov.AxiLiteReadOneConstant_0.read(0))
// -> 0x12345678
// ```
class AxiLiteReadOneConstant extends Module {
	val io = IO(new AxiLiteFollower)

	val magic = 0x12345678

	val OK = 0.U

	// read
	io.arready := true.B
	io.rdata := magic.U
	io.rresp := OK
	io.rvalid := true.B

	// write
	io.awready := false.B
	io.bresp := 0.U // does not matter
	io.bvalid := false.B
	io.wready := false.B

}

// This extends the above example to provide a different constant for
// each address.
//
// Test from `ipython3` like this:
// ```
// > "0x{:2x}".format(ov.AxiLiteReadDifferentConstants_0.read(0 * 4))
// -> 0x11111111
// > "0x{:2x}".format(ov.AxiLiteReadDifferentConstants_0.read(1 * 4))
// -> 0x2222222
// [...]
// ```
class AxiLiteReadDifferentConstants extends Module {
	val io = IO(new AxiLiteFollower)

	val data = Seq(0x11111111, 0x2222222, 0x3333333, 0x4444444, 0x5555555,
	               0x66666666, 0x7777777, 0x8888888, 0x9999999, 0xaaaaaaa)
	val default = 0

	val OK = 0.U

	// read state
	val sWaitForAddress :: sSendData :: Nil = Enum(2)
	val read_state = RegInit(sWaitForAddress)
	switch(read_state) {
		is(sWaitForAddress) { when(io.arready && io.arvalid) { read_state := sSendData } }
		is(sSendData) { when(io.rready && io.rvalid) { read_state := sWaitForAddress } }
	}

	// read
	// the two least significant address bits are ignored
	// since all accesses will be 32bit word aligned
	val read_address = RegInit(0.U((io.address_bits - 2).W))
	when(io.arready && io.arvalid) { read_address := io.araddr(io.address_bits-1, 2) }
	io.arready := read_state === sWaitForAddress
	// TODO: ignore lower two bits!
	io.rdata := MuxLookup(read_address, default.U, data.zipWithIndex.map{ case(d, ii) => ii.U -> d.U })
	io.rresp := OK
	io.rvalid := read_state === sSendData

	// write
	io.awready := false.B
	io.bresp := 0.U // does not matter
	io.bvalid := false.B
	io.wready := false.B

}


// This provides a loop back register that can be read and written at
// address zero.
//
// Test from `ipython3` like this:
// ```
// > "0x{:2x}".format(ov.AxiLiteLoopBack_0.read(0))
// -> 0x1993
// > ov.AxiLiteLoopBack_0.write(0, 1234)
// > ov.AxiLiteLoopBack_0.read(0)
// ```
class AxiLiteLoopBack extends Module {
	val io = IO(new AxiLiteFollower)

	val default = 0

	val OK = 0.U

	// loopback register
	val loopback = RegInit(0x1993.U(32.W))

	// read state
	val sWaitForAddress :: sSendData :: Nil = Enum(2)
	val read_state = RegInit(sWaitForAddress)
	switch(read_state) {
		is(sWaitForAddress) { when(io.arready && io.arvalid) { read_state := sSendData } }
		is(sSendData) { when(io.rready && io.rvalid) { read_state := sWaitForAddress } }
	}

	// read
	// the two least significant address bits are ignored
	// since all accesses will be 32bit word aligned
	val read_address = RegInit(0.U((io.address_bits - 2).W))
	when(io.arready && io.arvalid) { read_address := io.araddr(io.address_bits-1, 2) }
	io.arready := read_state === sWaitForAddress

	io.rdata := Mux(read_address === 0.U, loopback, default.U)
	io.rresp := OK
	io.rvalid := read_state === sSendData

	// write state
	val sWaitForWAddress :: sReceiveData :: sSendFeedback :: Nil = Enum(3)
	val write_state = RegInit(sWaitForWAddress)
	switch(write_state) {
		is(sWaitForWAddress) { when(io.awready && io.awvalid) { write_state := sReceiveData } }
		is(sReceiveData) { when(io.wready && io.wvalid) { write_state := sSendFeedback } }
		is(sSendFeedback) { when(io.bready && io.bvalid) { write_state := sWaitForWAddress } }
	}

	// write
	// the two least significant address bits are ignored
	// since all accesses will be 32bit word aligned
	val write_address = RegInit(0.U((io.address_bits - 2).W))
	when(io.awready && io.awvalid) { write_address := io.awaddr(io.address_bits-1, 2) }
	io.awready := write_state === sWaitForWAddress

	when(io.wready && io.wvalid && write_address === 0.U) { loopback := io.wdata }
	io.wready := write_state === sReceiveData

	io.bvalid := write_state === sSendFeedback
	io.bresp := OK // this is sort of a lie

}
