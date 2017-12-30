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
	val bready = Output(Bool())
	val bvalid = Input(Bool())
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

class AxiLiteHelloWorld extends Module {
	val io = IO(new AxiLiteFollower)
}

object AxiLiteHelloWorldGenerator extends App {
	chisel3.Driver.execute(args, () => new AxiLiteHelloWorld)
}
