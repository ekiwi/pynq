package pynq

import chisel3._
import chisel3.util._

object MakeCircuits extends App {
	def compile(target: String, dut: () => chisel3.experimental.RawModule) = {
		val args = s"--target-dir ip/${target}".split(" +")
		chisel3.Driver.execute(args, dut)
	}

	compile("axi_lite_read_one", () => new AxiLiteReadOneConstant)
	compile("axi_lite_read_different", () => new AxiLiteReadDifferentConstants)
	compile("axi_lite_loopback", () => new AxiLiteLoopBack)
	compile("axis_queue", () => new AxisQueue(64, 64))
}
