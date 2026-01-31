# Template for Rust projects on STM32F103 Nucleo-64 boards

## Machine setup

### Install `probe-rs` 

and add the rules file `/etc/udev/rules.d/69-probe-rs.rules`
for your ST-LINK

E.g. for ST-LINK V2:

```
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE="660", GROUP="plugdev", TAG+="uaccess"
```    
* Run `sudo udevadm control --reload` to ensure the new rules are used.
* Run `sudo udevadm trigger` to ensure the new rules are applied to already added devices.

* `rustup target add thumbv7m-none-eabi` (Add Rust ARM Cortex-M3 build target)

### Install cargo-embassy

`cargo install cargo-embassy`

A tool to set up an embassy project with

`cargo embassy init <project-name> --chip <your chip>`

Find supported chips with `probe-rs chip list`

## Flashing and Debugging

cargo run

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