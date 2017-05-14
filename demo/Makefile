default: upload

build_hello:
	xargo build -j1 --example hello

build_crash:
	xargo build -j1 --example crash

upload:
	openocd -f board/stm32vldiscovery.cfg
	#openocd -f board/stm32vldiscovery.cfg -d3

gdb:
hello:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/hello

crash:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/crash


