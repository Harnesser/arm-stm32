default: build_hello
#default: upload
#default: build_blinky01
#default: build_rotary_encoder

build_hello:
	xargo build -j2 --example hello

build_blinky:
	xargo build -j2 --example blinky

build_rotary_encoder:
	xargo build -j2 --example rotary-encoder --release

upload:
	openocd -f board/stm32vldiscovery.cfg
	#openocd -f board/stm32vldiscovery.cfg -d3

gdb: hello
hello:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/hello

blinky:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/blinky

rotary:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/rotary-encoder
	#arm-none-eabi-gdb target/thumbv7m-none-eabi/release/examples/rotary-encoder


