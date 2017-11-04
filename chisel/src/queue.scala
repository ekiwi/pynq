package pynq

import chisel3._
import chisel3.util._

class Queue(val depth: Int, val width: Int) extends Module {
	val io = IO(new Bundle {
		val in   = Input(UInt(width.W))
		val out  = Output(UInt(width.W))
		val push_back = Input(Bool())
		val pop_front = Input(Bool())
		val full = Output(Bool())
		val empty = Output(Bool())
		val len = Output(UInt(log2Ceil(depth).W))
	})

	io.out := 0.U
	io.full := false.B
	io.empty := true.B
	io.len := 0.U
}
