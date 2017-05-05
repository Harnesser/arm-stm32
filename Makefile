CFG=software.cfg

# List the versions of stuff I'm using
sw_cfg:
	@rustc -V > ${CFG} ; \
	arm-none-eabi-ld -V | head -n1 >> ${CFG} ; \
	arm-none-eabi-gdb -v | head -n1 >> ${CFG} ; \
	openocd -v 2>&1 | head -n1 >> ${CFG} ; \
	xargo -V 2>&1 >> ${CFG}; \
	cat ${CFG}


