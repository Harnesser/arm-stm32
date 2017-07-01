Serial Comms
================

Here's what I did to enable serial comms!

1. Installed `minicom`
2. Got my user name added to `dialout` group. Needed to 
   log out and back in again for this to take effect
3. Grabbed an Arduino UNO
4. Shorted RESET to GND on that board. This keeps the main
   microcontroller in reset, and allows us to use Tx and Rx
   pins of the AtMega interface chip as a serial interface!
5. Connections:
   Arduino 0 -> PA10
   Arduino 1 -> PA9
   Arduino GND -> ST GND
6. Made a default config file for `minicom` at `~/.minirc.dfl`
   (https://japaric.github.io/discovery/10-serial-communication/nix-tooling.html)
7. Downloaded `loopback` on to the SM board
8. Found TTY that the Arduino is on:
    `dmesg | grep -i tty`
9. Fired up `minicom`
    `minicom -D /dev/ttyACM0 -b 115200
10. Success! Shit's being repeated at me! Rx and Tx lights on
   Arduino flash!
11. To test, disconnected Rx and Tx. No repeat! No Rx light on
   Arduino board!

Woo!

