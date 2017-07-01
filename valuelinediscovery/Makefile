#NAME:=hello
#NAME:=blinky
#NAME:=roulette
#NAME:=loopback
NAME:=concurrency

TTY:=/dev/ttyACM0
BAUD:=115200

TGT:=debug
LINK:=
#LINK:=-d3

build:
	xargo build -j2 --example ${NAME}

upload:
	openocd -f board/stm32vldiscovery.cfg ${LINK}

gdb:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/${TGT}/examples/${NAME}

minicom:
	minicom -D ${TTY} -b ${BAUD}
