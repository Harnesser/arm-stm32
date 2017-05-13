# Rust on ARM Microcontrollers

From [Rust your ARM microcontroller](http://blog.japaric.io/quickstart/)

# Hardware

## Eval Boards I have
* [STM32 Value Line Discovery] [1] -- ARM Cortex-M3, buttons, LEDs.
* [STM32F411 Discovery] [2] -- ARM Cortex-M4, Gyros, accels, audio.
* [STM32 F7 Discovery] [3]  -- ARM Cortex-M7, TFT LCD, Audio

## Eval Boards in the blog post
* [STM32F3 Discovery] -- ARM Cortex-M4, accels, LEDS.

## Does the blog show me how to flash the microcontroller?
OpenOCD


# Toolchain

## Nightly Rust
    
    rustup default nightly

## ARM Tools

    arm-none-eabi-binutils arm-none-eabi-gdb openocd

## Xargo

    cargo install xargo
    rustup component add rust-src

## ARM Project Template

    cargo install cargo-clone
    cargo clone cortex-m-quickstart --vers 0.1.1

# Linux Packages
    apt-get openocd  (0.7.0-2)



# My Chip
The chip on the board is a STM32F100RBT6B. This is a Cortex-M3 with:

* STM32 -- 32-bit microcontroller
* F -- general purpose
* 100 -- value line
* R -- 64 pins
* B -- 128kBytes of Flash memory
* T -- LQFP
* 6 -- -40 to 85 deg C
* B -- some internal code (probly 3rd Si rev?)

Further, theres:
* 6kBytes of SRAM.
* 6 GPTs
* 2 SPI
* 2 UART
* 2 I2C
* 3 USART
* 1 CEC
* 1 12-but ADC, 16 channels
* 51 GPIOs
* 2 12-bit DAC
* 24MHz CPU freq
* 2.0V to 3.6V operating range

Page 30/96 of the datasheet says that the memories start at the following
locations:
* SRAM  -- 0x2000_0000
* Flash -- 0x0800_0000

# Hello World!
The OpenOCD docs recommend that if you've a popular dev board, you should use
configuration file for it:

    openocd -f board/stm32vldiscovery.cfg

This gives the error:

    Info : This adapter doesn't support configurable speed
    Error: open failed
    in procedure 'transport'
    in procedure 'init'

Is this a linux permissions thing with `/etc/udev/rules.d` and such?




# Links
* [id]: www.st.com/stm32-discovery

# Bugs

## Can''t install GDB Thingy 
* [Installing arm-none-eabi-gdb](https://bugs.launchpad.net/ubuntu/+source/gdb-arm-none-eabi/+bug/1267680/comments/13)

    sudo dpkg-divert --package gdb --divert /usr/share/man/man1/arm-none-eabi-gdb.1.gz --rename /usr/share/man/man1/gdb.1.gz


## Need OpenSSL lib

   libssl-dev


## Need Ssh

   libssh2-1-dev


## Need `cmake`

   cmake

## Too Much
Had to throw `-j1` switch to restrict the compliation process to one CPU else my
computer just crashed.


## UDEV Permissions
Can't do hardware access without `udev` rules
```
kartoffel% pls
<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/etc/udev/rules.d
<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
52-digilent-usb.rules  70-persistent-net.rules  99-openocd.rules  README
kartoffel% lsusb                     
Bus 001 Device 004: ID 0bda:0111 Realtek Semiconductor Corp. RTS5111 Card Reader Controller
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 002 Device 006: ID 0483:3744 STMicroelectronics STLINK Pseudo disk
Bus 002 Device 003: ID 046d:c31c Logitech, Inc. Keyboard K120
Bus 002 Device 002: ID 03f0:a407 Hewlett-Packard 
Bus 002 Device 001: ID 1d6b:0001 Linux Foundation 1.1 root hub
kartoffel% ls -l /dev/bus/usb/002/006
crw-rw-r-- 1 root sharers 189, 133 05-May-17 /dev/bus/usb/002/006
kartoffel% groups                    
harnesser sharers
```

Don't remember what I did to 'fix'.  Added the `udev` file, and then
got USB mass storage errors.

Added myself to `plugdev`.

```
kartoffel% pls
<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
/etc/udev/rules.d
<><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
52-digilent-usb.rules  70-persistent-net.rules  99-openocd.rules  README
kartoffel% cat 99-openocd.rules 

# STLINKv1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3744", GROUP="plugdev", MODE:="660"
```


## USB Mass Storage

After googling, something is screwy with the STLINKv1 firmware, and the
best thing to avoid this issue is to tell Linux not to bother with the
mass-storage endpoint on the board.

```
kartoffel% pwd
/etc/modprobe.d
kartoffel% cat stlink.conf 
# stlink/v1 ignore mass storage
options usb-storage quirks=0x0483:0x3744:i
```

Tried rebooting, but this didn't quite work.

```
sudo modprobe -r uas usb-storage
sudo modprobe usb-storage uas
```

This worked. Hope I don't have to do this all the time. It looks like I will have
to do this all the time... Fuckit.


## Program not halted

If I try:

* Terminal 1: Run openocd to flash the thing
* Terminal 2: GDB to step through the program

I can't flash the thing because:

```
Error: 320 64836 stm32f1x.c:433 stm32x_erase(): Target not halted
Error: 321 64836 core.c:47 flash_driver_erase(): failed erasing sectors 0 to 3
Debug: 322 64836 target.c:1294 target_call_event_callbacks(): target event 26 (gdb-flash-erase-end)
Error: 323 64836 gdb_server.c:1875 gdb_v_packet(): flash_erase returned -304
```

So:

```
telnet localhost 4444
> halt
```

Nope.

Found magic:

* http://openocd.org/doc/html/GDB-and-OpenOCD.html

```
(gdb) monitor reset halt
target state: halted
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x0800066c msp: 0x20002000, semihosting
(gdb) load
Loading section .text, size 0xd88 lma 0x8000000
Loading section .debug_gdb_scripts, size 0x22 lma 0x8000d88
Start address 0x8000000, load size 3498
Transfer rate: 3 KB/sec, 1749 bytes/write.
```

OK. Is this a thing yet? That seemed to have at least overwritten the LED flasher
program that was flashed on the device when I bought it. I can't seem to get the
message printed, but maybe I'm looking in the wrong terminal?

So again:
1: terminal open ocd
2: terminal gdb

in GBD:
```
(gdb) monitor reset halt
target state: halted
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x0800066c msp: 0x20002000, semihosting
(gdb) tbreak hello::main
Temporary breakpoint 1 at 0x8000404: file examples/hello.rs, line 13.
(gdb) 
Note: breakpoint 1 also set at pc 0x8000404.
Temporary breakpoint 2 at 0x8000404: file examples/hello.rs, line 13.
(gdb) cont
Continuing.
Note: automatically using hardware breakpoints for read-only addresses.

Temporary breakpoint 1, hello::main () at examples/hello.rs:13
13	    hprintln!("Hello, world!");
(gdb) 
Continuing.
jtag status contains invalid mode value - communication failure
Polling target stm32f1x.cpu failed, GDB will be halted. Polling again in 100ms
```

But I do see the message in the openocd terminal:

```
Debug: 585 33376 hla_target.c:697 adapter_read_memory(): adapter_read_memory 0x20001f04 4 3
Debug: 586 33392 target.c:1764 target_read_buffer(): reading buffer of 14 byte at 0x08000b28
Debug: 587 33392 hla_target.c:697 adapter_read_memory(): adapter_read_memory 0x08000b28 4 3
Debug: 588 33406 hla_target.c:697 adapter_read_memory(): adapter_read_memory 0x08000b34 2 1
Hello, world!
Debug: 589 33423 target.c:1294 target_call_event_callbacks(): target event 3 (resume-start)
Debug: 590 33423 hla_target.c:551 adapter_resume(): adapter_resume 1 0x00000000 0 0
Debug: 591 33423 target.c:1615 target_free_all_working_areas_restore(): freeing all working areas
Debug: 592 33423 hla_target.c:697 adapter_read_memory(): adapter_read_memory 0x080009b4 2 1
Debug: 593 33437 target.c:1936 target_read_u16(): address: 0x080009b4, value: 0xbeab
Debug: 594 33437 armv7m.c:780 armv7m_maybe_skip_bkpt_inst(): Skipping over BKPT instruction
Debug: 595 33437 target.c:1978 target_write_u32(): address: 0xe000edfc, value: 0x01000000
Debug: 596 33437 hla_target.c:745 adapter_write_memory(): adapter_write_memory 0xe000edfc 4 1
Debug: 597 33451 armv7m.c:140 armv7m_restore_context():  
Debug: 598 33451 hla_target.c:153 adapter_store_core_reg_u32(): adapter_store_core_reg_u32
Debug: 599 33458 hla_target.c:183 adapter_store_core_reg_u32(): write core reg 15 value 0x80009b6
Debug: 600 33458 armv7m.c:249 armv7m_write_core_reg(): write core reg 15 value 0x80009b6
Debug: 601 33458 hla_target.c:153 adapter_store_core_reg_u32(): adapter_store_core_reg_u32
Debug: 602 33465 hla_target.c:183 adapter_store_core_reg_u32(): write core reg 0 value 0x0
Debug: 603 33465 armv7m.c:249 armv7m_write_core_reg(): write core reg 0 value 0x0
Debug: 604 33472 target.c:1294 target_call_event_callbacks(): target event 2 (resumed)
Debug: 605 33472 target.c:1294 target_call_event_callbacks(): target event 4 (resume-end)
Error: 606 33578 hla_target.c:389 adapter_poll(): jtag status contains invalid mode value - communication failure
```

But then the JTAG goes weird. From things I'm reading on the net, it's because
the `wfi` (Wait for Interrupt) instruction puts the chip in a low-power mode waiting
for an interrupt, which also happens to kill the JTAG clock.

So I need an LED blinker program... ... ...








