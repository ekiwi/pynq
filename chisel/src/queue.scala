package pynq

import chisel3._
import chisel3.util._

class Queue[T <: Data](val depth: Int, data_t: T) extends Module {
	val io = IO(new Bundle {
		val in   = Input(data_t.chiselCloneType)
		val out  = Output(data_t.chiselCloneType)
		val push_back = Input(Bool())
		val pop_front = Input(Bool())
		val full = Output(Bool())
		val empty = Output(Bool())
		val len = Output(UInt(log2Ceil(depth).W))
	})
}
