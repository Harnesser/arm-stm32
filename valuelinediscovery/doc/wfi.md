Help! I can't communicate with the board!
==========================================

You may have a "WFI" instruction in your code:

From: http://openocd.org/doc/html/OpenOCD-Project-Setup.html

 ARM Wait-For-Interrupt... Many ARM chips synchronize the JTAG clock using the core clock. Low power states which stop that core clock thus prevent JTAG access. Idle loops in tasking environments often enter those low power states via the WFI instruction (or its coprocessor equivalent, before ARMv7).

You may want to disable that instruction in source code, or otherwise prevent using that state, to ensure you can get JTAG access at any time.3 For example, the OpenOCD halt command may not work for an idle processor otherwise.

To fix, mass-erase the flash over telnet. (Hold reset during `reset halt`)


    kartoffel% telnet localhost 4444
    Trying 127.0.0.1...
    Connected to localhost.
    Escape character is '^]'.
    Open On-Chip Debugger
    > reset halt
    timed out while waiting for target halted
    TARGET: stm32f1x.cpu - Not halted
    
    in procedure 'reset'
    target state: halted
    target halted due to debug-request, current mode: Thread 
    xPSR: 0x01000000 pc: 0x08000130 msp: 0x20002000
    > flash probe 0
    device id = 0x10016420
    flash size = 128kbytes
    device id = 0x10016420
    flash size = 128kbytes
    flash 'stm32f1x' found at 0x08000000
    > stm32f1x mass_erase 0
    stm32x mass erase complete
    > exit
    Connection closed by foreign host.


