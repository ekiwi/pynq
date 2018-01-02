# PYNQ with Chisel and Rust

This repository contains some examples on how Chisel and Rust can be
used to create custom peripherals on the FPGA and interface them from
the ARM core of the Zynq device.

The documentation helps you [setup your system](doc/system_setup.md).


## Test Log

* successfully tested accessing the rgb leds from the `Overlay` on `2017-11-01` using `Vivado 2017.2`:

```.py
> sudo ipython3
In : ov = Overlay('/home/xilinx/system.bit')
In : ov.rgbleds_gpio[0:2].write(5)
```
* make sure to have the [Digilent board files](https://github.com/Digilent/vivado-boards) installed in
  `/opt/Xilinx/Vivado/2017.2/data/boards/board_files`

* successfully tested accessing the loopback DMA from the `Overlay` on `2017-11-01` using `Vivado 2017.2`:

```.py
> sudo ipython3
In ....: ov = Overlay('/home/xilinx/system.bit')
In ....: xlnk = Xlnk()
    ...: in_buffer = xlnk.cma_array(shape=(5,), dtype=np.uint32)
    ...: out_buffer = xlnk.cma_array(shape=(5,), dtype=np.uint32)
    ...: 
    ...: for i in range(5):
    ...:     in_buffer[i] = i
    ...: 
    ...: dma = ov.axi_dma_0
    ...: dma.sendchannel.transfer(in_buffer)
    ...: dma.recvchannel.transfer(out_buffer)
    ...: dma.sendchannel.wait()
    ...: dma.recvchannel.wait()
    ...: 
    ...: out_buffer
Out ...:  ContiguousArray([0, 1, 2, 3, 4], dtype=uint32)
```
