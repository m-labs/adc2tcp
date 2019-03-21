# Synopsis

Exposes readings from an ADC pin (currently: *PA3*) of the board via a
TCP service on the Ethernet port.


# Network Protocol

Sensor readings produce lines of `key=value` pairs, joined by `,`,
terminated by `"\r\n"`.

```
t=21000,pa3=685
t=22000,pa3=684
t=23000,pa3=681
t=24000,pa3=696
t=25000,pa3=673
t=26000,pa3=689
t=27000,pa3=657
t=28000,pa3=654
t=29000,pa3=652
t=30000,pa3=662
t=31000,pa3=663
```

| Key | Value       | Unit |
|:---:|-------------|------|
| t   | Time        | ms   |
| pa3 | ADC reading | mV   |


# LEDs

Colors indicate what the MCU is occupied with.

| Color   | Indication        |
|:-------:|-------------------|
| Green   | WFI (idle)        |
| Blue    | Network poll      |
| Red     | Message broadcast |


# Crate features

* `semihosting` enables log output via the **cortex-m-semihosting**
  crate. Use only in development! MCU will hang when no OpenOCD is
  running.

* `generate-hwaddr` generates an Ethernet MAC address by hashing the
  unique device ID from flash memory.


# Instructions

![Made for NixOS](https://nixos.org/logo/nixos-lores.png)

## Build the firmware with `default.nix`

* `nix-build`
* This uses **cargo-vendor** to bundle dependencies, so that unstable versions from git can be used.
* Run `result/bin/flash-adc2tcp` to flash a devboard with OpenOCD and quit.

## Development environment with `shell.nix`

* `nix-shell`
* Spawning `openocd`, the devboard should be connected already.
* Instructions (`cargo run --release`) are printed.
