# Blinky on an STM32F401

This `no_std` example blinks the PA5 LED on an STM32F401-based Nucleo board. Its dependencies and target configuration are board-specific; it is not a portable example for RP2040, AVR, or ESP32 boards.

## Requirements

- The `thumbv7em-none-eabihf` Rust target
- An STM32F401-based board when flashing the program
- A compatible flashing tool such as `cargo-flash` or `probe-rs`

Install the compilation target once:

```bash
rustup target add thumbv7em-none-eabihf
```

The local `.cargo/config.toml` selects that target and passes the Cortex-M linker script automatically.

## Check and build

```bash
cargo check
cargo build --release
```

To flash a connected Nucleo-F401RE with `cargo-flash`:

```bash
cargo flash --release --chip STM32F401RETx
```

## What the example demonstrates

- A `#![no_std]` and `#![no_main]` binary
- A Cortex-M runtime entry point
- Peripheral ownership through the STM32 peripheral access crate
- GPIO output configuration
- A hardware-timer delay in a non-terminating loop

The target configuration is part of the example. Running `cargo check` for the host target is not a meaningful validation of this binary because the host supplies `std`, an operating-system entry point, and a different instruction set.
