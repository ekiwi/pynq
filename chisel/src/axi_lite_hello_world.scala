package pynq

import chisel3._
import chisel3.util._

class AxiLiteFollower extends Bundle {
	val data_bits = 32
	val address_bits = 4
	val words = (1 << address_bits)
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
class AxiLiteReadOnceConstant extends Module {
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

object AxiLiteHelloWorldGenerator extends App {
	chisel3.Driver.execute(args, () => new AxiLiteReadOnceConstant)
}
