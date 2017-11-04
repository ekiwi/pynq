package pynq

import chisel3._
import chisel3.util._

class AxisQueue(val depth: Int, val width: Int) extends Module {
	val bit_count = (width + 7) / 8
	val io = IO(new Bundle {
		// TODO: split up into separate producer / consumer boundles
		val s_axis_tvalid = Input(Bool())
		val s_axis_tready = Output(Bool())
		val s_axis_tdata = Input(UInt(width.W))
		val s_axis_tkeep = Input(UInt(bit_count.W))
		val s_axis_tlast = Input(Bool())
		val m_axis_tvalid = Output(Bool())
		val m_axis_tready = Input(Bool())
		val m_axis_tdata = Output(UInt(width.W))
		val m_axis_tkeep = Output(UInt(bit_count.W))
		val m_axis_tlast = Output(Bool())
	})
	val data = Module(new Queue(depth, width))
	val keep = Module(new Queue(depth, bit_count))

	// we do not care about packets
	io.m_axis_tlast := false.B

	data.io.push_back := io.s_axis_tvalid
	keep.io.push_back := io.s_axis_tvalid
	io.s_axis_tready := !data.io.full && !keep.io.full
	data.io.in := io.s_axis_tdata
	keep.io.in := io.s_axis_tkeep

	io.m_axis_tvalid := !data.io.empty && !keep.io.empty
	data.io.pop_front := io.m_axis_tready
	keep.io.pop_front := io.m_axis_tready
	io.m_axis_tdata := data.io.out
	io.m_axis_tkeep := keep.io.out
}

object AxisQueueGenerator extends App {
	chisel3.Driver.execute(args, () => new AxisQueue(64, 64))
}
