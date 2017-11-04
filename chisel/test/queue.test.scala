// See LICENSE for license details.

package pynq

import chisel3._
import chisel3.iotesters
import chisel3.iotesters.{ChiselFlatSpec, Driver, PeekPokeTester}

class QueueUnitTester[T <: Data](queue: Queue[T], val depth: Int) extends PeekPokeTester(queue) {
	private val q = queue

	poke(q.io.push_back, false)
	poke(q.io.pop_front, false)
	//poke(q.io.in, 0)


	// for(i <- 1 to 40 by 3) {
	// for (j <- 1 to 40 by 7) {
	// 	poke(gcd.io.value1, i)
	// 	poke(gcd.io.value2, j)
	// 	poke(gcd.io.loadingValues, 1)
	// 	step(1)
	// 	poke(gcd.io.loadingValues, 0)

	// 	val (expected_gcd, steps) = computeGcd(i, j)

	// 	step(steps - 1) // -1 is because we step(1) already to toggle the enable
	// 	expect(gcd.io.outputGCD, expected_gcd)
	// 	expect(gcd.io.outputValid, 1)
	// }
	// }
}

class GCDTester extends ChiselFlatSpec {
	private val backendNames =
		if(firrtl.FileUtils.isCommandAvailable("verilator")) {
			Array("firrtl", "verilator") } else { Array("firrtl") }

	for ( backendName <- backendNames ) {
		"Queue" should s"behave like a bounded queue (with $backendName)" in {
			Driver(() => new Queue(4, UInt(8.W)), backendName) {
				queue => new QueueUnitTester(queue, 4)
			} should be (true)
		}
	}
}
