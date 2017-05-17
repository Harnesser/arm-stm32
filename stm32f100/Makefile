DEVICE:=STM32F100xx.svd
SVD:=https://github.com/posborne/cmsis-svd/raw/python-0.4/data/STMicro/${DEVICE}
DOC:=/osshare/Documents/Datasheets/microcontroller/ST/

def:
	@echo "Default"


svd-download:
	curl -LO ${SVD};

svd:
	svd2rust -i ${DEVICE} > src/lib.rs ;\
	rustfmt src/lib.rs

sanity-check:
	xargo build -j1 --target thumbv7m-none-eabi

# Chip Datasheed
datasheet:
	evince ${DOC}/STM32F100RBT6B_rev9.pdf

# Chip Hardware Reference Manual
hrm:
	evince ${DOC}/STM32F100xx-Reference-Manual.pdf
