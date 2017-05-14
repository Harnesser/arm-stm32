default: upload

build:
	xargo build -j1 --example hello

upload:
	openocd -f board/stm32vldiscovery.cfg
	#openocd -f board/stm32vldiscovery.cfg -d3

gdb:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/hello


