#NAME:=hello
#NAME:=blinky
#NAME:=roulette
#NAME:=loopback
#NAME:=concurrency
#NAME:=parse
#NAME:=count_button_presses
#NAME:=liquid_crystal
#NAME:=button_and_lcd
NAME:=rotary_and_lcd

TTY:=/dev/ttyACM0
BAUD:=115200

TGT:=debug
LINK:=
#LINK:=-d3

build:
	xargo build -j2 --example ${NAME}

release:
	xargo build -j2 --release --example ${NAME}

upload:
	openocd -f board/stm32vldiscovery.cfg ${LINK}

gdb:
	arm-none-eabi-gdb target/thumbv7m-none-eabi/${TGT}/examples/${NAME}

minicom:
	minicom -D ${TTY} -b ${BAUD}

objdump:
	arm-none-eabi-objdump -Cd target/thumbv7m-none-eabi/${TGT}/examples/${NAME}
