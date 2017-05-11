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
sudo modprobe usb-storage
```

This worked. Hope I don't have to do this all the time.


