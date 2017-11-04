package pynq

import chisel3._
import chisel3.util._

class AxisQueue(val depth: Int, val width: Int) extends Module {
	val io = IO(new Bundle {
		// TODO: split up into separate producer / consumer boundles
		val s_axis_tvalid = Input(Bool())
		val s_axis_tready = Output(Bool())
		val s_axis_tdata = Input(UInt(width.W))
		val m_axis_tvalid = Output(Bool())
		val m_axis_tready = Input(Bool())
		val m_axis_tdata = Output(UInt(width.W))
	})
	val q = Module(new Queue(depth, width))
	q.io.push_back := io.s_axis_tvalid
	io.s_axis_tready := !q.io.full
	q.io.in := io.s_axis_tdata
	io.m_axis_tvalid := !q.io.empty
	q.io.pop_front := io.m_axis_tready
	io.m_axis_tdata := q.io.out
}

object AxisQueueGenerator extends App {
	chisel3.Driver.execute(args, () => new AxisQueue(64, 64))
}
