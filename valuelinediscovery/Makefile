#NAME:=hello
#NAME:=blinky
NAME:=roulette

TGT:=debug
LINK:=
#LINK:=-d3

build:
	xargo build -j2 --example ${NAME}

upload:
	openocd -f board/stm32vldiscovery.cfg ${LINK}

gdb:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/${TGT}/examples/${NAME}

