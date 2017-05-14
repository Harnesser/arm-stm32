DEVICE:=STM32F100xx.svd
SVD:=https://github.com/posborne/cmsis-svd/raw/python-0.4/data/STMicro/${DEVICE}

def:
	@echo "Default"


svd-download:
	#curl -LO ${SVD};
	svd2rust -i ${DEVICE} > src/lib.rs ;\
	rustfmt src/lib.rs

sanity-check:
	xargo build -j1 --target thumbv7m-none-eabi


