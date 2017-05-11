default: upload

build:
	xargo build --example hello

upload:
	openocd -f board/stm32vldiscovery.cfg
	#openocd -f board/stm32vldiscovery.cfg -d3

#	openocd -f interface/stlink-v2-1.cfg -f target/stm32f100.cfg
