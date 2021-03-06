# FFP Firmware

## Building

```
cargo build --release
```

## Setting Option Bytes

Note that the built in bootloader will just keep jumping to the user
application if the `BOOT_SEL` option bit is set (the default). You have to
clear this to 0 to force always booting from main flash, at which point the
built in bootloader can be jumped to from the user application. Wild.

```
(gdb) mon option 0x1FFFF802 0x807F
0x1FFFF800: 0x55AA
0x1FFFF802: 0x807F
0x1FFFF804: 0x00FF
0x1FFFF806: 0x00FF
0x1FFFF808: 0x00FF
0x1FFFF80A: 0x00FF
0x1FFFF80C: 0x00FF
0x1FFFF80E: 0x00FF
```

## Bootloading

To use the built-in ST USB bootloader:

```
cargo build --release
arm-none-eabi-objcopy -O binary -S target/thumbv6m-none-eabi/release/ffp_firmware ffp.bin
ffp bootload
dfu-util -a 0 -s 0x08000000 -D ffp.bin
```

## Licence

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
