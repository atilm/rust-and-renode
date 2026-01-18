# Template for Rust projects on STM32F103 Nucleo-64 boards

## Machine setup

* `sudo apt-get install gdb-multiarch openocd`
* Add rule to use ST-Link without root privileges:
    * Connect ST-Link and execute `lsusb`
        * Output: `Bus 003 Device 003: ID 0483:374b STMicroelectronics ST-LINK/V2.1`
        * Note `idVendor: 0483` and `idProduct:374b`
    * Create `/etc/udev/rules.d/99-openocd.rules` with content
```
# STM32F3DISCOVERY - ST-LINK/V2.1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE:="0666"
```    
    * sudo udevadm control --reload-rules

* `rustup target add thumbv7m-none-eabi` (Add Rust ARM Cortex-M3 build target)

## Flashing and Debugging

* Start openocd server in a terminal: `openocd -f interface/stlink.cfg -f target/stm32f1x.cfg`
* cargo run
or
    * Start debugger manually: `gdb-multiarch -q -ex "target remote :3333" target/thumbv7m-none-eabi/debug/cortex-m-quickstart`

## Renode setup

The `.repl` file [renode/platforms/boards/nucleo_f103rb.repl](renode/platforms/boards/nucleo_f103rb.repl) is
copied from renodes Zephyr Dashboard.

Start the emulation with 

```sh
renode -e 'include @./renode/nucleo_f103rb.resc'
```

In the monitor execute

```
gpioc.button Press
gpioc.button Release
```

and watch the LED state change in the console log.

## References

([1] The embedded rust discovery book)[https://docs.rust-embedded.org/discovery/f3discovery/]
([2] The cortex-m-quickstart crate)[https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/]