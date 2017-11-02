system.bit: system.tcl
	vivado -mode batch -source system.tcl
clean:
	rm -r ekiwi
	rm vivado*
	rm system.bit
