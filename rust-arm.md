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


