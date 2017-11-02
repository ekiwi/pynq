# pynq

* successfully tested accessing the rgb leds from the `Overlay` on `2017-11-01` using `Vivado 2017.2`:

```.py
> sudo ipython3
In : ov = Overlay('/home/xilinx/system.bit')
In : ov.rgbleds_gpio[0:2].write(5)
```
* make sure to have the [Digilent board files](https://github.com/Digilent/vivado-boards) installed in
  `/opt/Xilinx/Vivado/2017.2/data/boards/board_files`
