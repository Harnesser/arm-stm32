default: upload

build:
	xargo build -j1 --example hello

upload:
	openocd -f board/stm32vldiscovery.cfg
	#openocd -f board/stm32vldiscovery.cfg -d3

#	openocd -f interface/stlink-v2-1.cfg -f target/stm32f100.cfg


gdb:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/examples/hello


