# Blinky - LED Blinking Example

The "Hello World" of embedded systems. Blink an LED on a microcontroller.

## Supported Boards

- **STM32 Nucleo-64** (F401RE, F411RE, etc.)
- **Raspberry Pi Pico** (RP2040)
- **Arduino Uno** (AVR - experimental)
- **ESP32-C3** (RISC-V)

## What This Does

Toggles an on-board LED every 1 second, demonstrating:
- no_std environment
- GPIO output control
- Delay timers
- Infinite loop patterns

## Building & Flashing

### For STM32:
```bash
cargo build --release
cargo flash --chip STM32F401RETx
```

### For RP2040 (Pico):
```bash
cargo build --release
elf2uf2-rs target/thumbv6m-none-eabi/release/blinky
# Copy UF2 file to Pico
```

### For ESP32-C3:
```bash
cargo espflash flash --release --monitor
```

## Key Concepts

- **#![no_std]** - No standard library
- **#![no_main]** - Custom entry point
- **Panic handler** - What happens on panic
- **GPIO abstraction** - Portable LED control
- **Delay** - Blocking delays without OS

## Memory Usage

- Flash: ~4-8 KB
- RAM: <1 KB

## Next Steps

- Change blink frequency
- Add multiple LEDs
- Use PWM for breathing effect
- Respond to button presses
