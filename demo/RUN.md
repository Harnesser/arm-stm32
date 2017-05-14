
# Demo

Eval board: SMT32VLDISCOVERY
All setup as in ../rust-arm.md


To run:

1. For some reason, I need to unload then reload the usb-storage module
  > sudo modprobe -r uab usb-storage
  > sudo modprobe uab usb-storage

2. In terminal #1, run openocd to allow us to talk to the chip
  > cd arm-rust/demo
  > make

3. In terminal #2, run the GNU Debugger:
  > cd arm-rust/demo
  > make gdb

4. There will be some problem. To fix:
  (gbd) monitor reset halt
  (gdb) load # if you want to download the program to the board
  (gdb) tbreak hello::main
  (gdb) cont
  (gdb) (press enter, then "Hello World!" should appear in terminal #1)
  (gdb) cont (things will break now, `wfi` instruction kills JTAG)



