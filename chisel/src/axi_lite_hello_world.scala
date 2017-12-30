package pynq

import chisel3._
import chisel3.util._

class AxiLiteHelloWorld extends Module {
	val width = 32
	val bit_count = (width + 7) / 8
	val address_bits = 4
	val words = (1 << address_bits)
	val io = IO(new Bundle {
		// TODO: split up into separate bundles
		// TODO: maybe use ReadyValid interface from stdlib
		// read address interface
		val s_axi_lite_araddr  = Input(UInt(address_bits.W))
		val s_axi_lite_arready = Output(Bool())
		val s_axi_lite_arvalid = Input(Bool())
		// write address interface
		val s_axi_lite_awaddr  = Input(UInt(address_bits.W))
		val s_axi_lite_awready = Output(Bool())
		val s_axi_lite_awvalid = Input(Bool())
		// write response interface
		val s_axi_lite_bresp  = Output(UInt(2.W))
		val s_axi_lite_bready = Output(Bool())
		val s_axi_lite_bvalid = Input(Bool())
		// read data interface
		val s_axi_lite_rdata = Output(UInt(width.W))
		val s_axi_lite_rresp = Output(UInt(2.W))
		val s_axi_lite_rready = Input(Bool())
		val s_axi_lite_rvalid = Output(Bool())
		// write data interface
		val s_axi_lite_wdata = Input(UInt(width.W))
		val s_axi_lite_wready = Output(Bool())
		val s_axi_lite_wvalid = Input(Bool())
	})
}

object AxiLiteHelloWorldGenerator extends App {
	chisel3.Driver.execute(args, () => new AxiLiteHelloWorld)
}
