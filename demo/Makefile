#default: upload
#default: build_blinky01
default: build_rotary_encoder

build_hello:
	xargo build -j1 --example hello

build_crash:
	xargo build -j1 --example crash

build_blinky01:
	xargo build -j1 --example blinky01

build_rotary_encoder:
	xargo build -j1 --example rotary-encoder

upload:
	openocd -f board/stm32vldiscovery.cfg
	#openocd -f board/stm32vldiscovery.cfg -d3

gdb: hello
hello:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/hello

crash:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/crash

blinky01:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/blinky01

rotary:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/rotary-encoder


