# Section 12: Embedded Rust (no_std)

## Overview

Embedded Rust enables writing firmware for microcontrollers and resource-constrained devices with Rust's memory safety guarantees. The `no_std` environment means no standard library heap allocations or operating system dependencies.

## Why Rust for Embedded?

✅ **Memory Safety** - No buffer overflows, use-after-free, or data races <br />
✅ **Zero-cost Abstractions** - High-level code compiles to same assembly as C <br />
✅ **No Runtime** - No garbage collector or runtime overhead <br />
✅ **Fearless Concurrency** - Safe interrupt handling and multi-core <br />
✅ **Small Binaries** - Optimized for flash-constrained devices

## What You'll Learn

1. **no_std Basics** - Writing Rust without the standard library
2. **Embedded HAL** - Hardware Abstraction Layer traits
3. **Peripheral Access** - Memory-mapped I/O safely
4. **Interrupts** - Safe interrupt handlers
5. **Real-Time** - RTIC framework for real-time systems
6. **Debugging** - GDB, probe-rs, and defmt logging

## Target Platforms

- **ARM Cortex-M** - STM32, nRF52, RP2040 (Raspberry Pi Pico)
- **RISC-V** - ESP32-C3, GD32VF103
- **AVR** - Arduino (experimental)
- **Xtensa** - ESP32, ESP8266

## Section Contents

### 01-no-std-basics/
Understand `no_std`, `#![no_main]`, panic handlers

### 02-blinky/
Classic LED blink on STM32 or RP2040

### 03-uart-communication/
Serial communication with PC

### 04-i2c-sensor/
Read temperature/humidity sensor via I2C

### 05-interrupt-handling/
Button press interrupts, timer interrupts

### 06-rtic-real-time/
Real-Time Interrupt-driven Concurrency framework

## Prerequisites

- Rust installed
- Basic embedded concepts (registers, peripherals, interrupts)
- Hardware: Development board (STM32 Nucleo, Raspberry Pi Pico, etc.)
- Debugger: ST-Link, J-Link, or Raspberry Pi Debug Probe

## Tools Setup

```bash
# Install embedded tools
rustup target add thumbv7em-none-eabihf  # Cortex-M4F
cargo install cargo-embed cargo-flash
cargo install probe-rs

# For RP2040 (Pico)
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs

# For ESP32
cargo install espflash cargo-espflash
```

## Project Structure

```
12-embedded-rust/
├── README.md
├── 01-no-std-basics/
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── memory.x          # Linker script
│   └── README.md
├── 02-blinky/
├── 03-uart-communication/
├── 04-i2c-sensor/
├── 05-interrupt-handling/
└── 06-rtic-real-time/
```

## Key Concepts

### no_std Environment

```rust
#![no_std]
#![no_main]

use panic_halt as _;  // Panic handler

#[entry]
fn main() -> ! {
    // Your embedded code
    loop {}
}
```

### Memory-Mapped I/O

```rust
// Safe peripheral access
use stm32f4xx_hal::pac;

let dp = pac::Peripherals::take().unwrap();
let gpioa = dp.GPIOA;

// Set PA5 high (LED on)
gpioa.odr.modify(|_, w| w.odr5().set_bit());
```

### Embedded HAL

```rust
use embedded_hal::digital::v2::OutputPin;

fn toggle_led<P: OutputPin>(led: &mut P) {
    led.toggle().ok();
}
```

### Interrupts

```rust
#[interrupt]
fn TIM2() {
    // Timer interrupt handler
    static mut COUNT: u32 = 0;
    *COUNT += 1;
}
```

## Common Patterns

### Singleton Pattern

```rust
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;

static LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));
```

### Type State Pattern

```rust
struct Pin<MODE> {
    _mode: PhantomData<MODE>,
}

impl Pin<Input> {
    fn into_output(self) -> Pin<Output> {
        // Configure as output
        Pin { _mode: PhantomData }
    }
}
```

## Memory Constraints

| Device | Flash | RAM | Example |
|--------|-------|-----|---------|
| ATmega328P | 32KB | 2KB | Arduino Uno |
| STM32F103 | 64KB | 20KB | Blue Pill |
| RP2040 | 2MB* | 264KB | Raspberry Pi Pico |
| ESP32-C3 | 4MB | 400KB | ESP32-C3 |

*External flash

## Debugging

```bash
# Using probe-rs
cargo embed --release

# GDB debugging
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/app

# Logging with defmt
DEFMT_LOG=info cargo embed
```

## Power Management

```rust
use cortex_m::asm;

loop {
    // Sleep until interrupt
    asm::wfi();
}
```

## Common Crates

- **cortex-m** - Cortex-M processor support
- **embedded-hal** - Hardware abstraction traits
- **defmt** - Efficient logging for embedded
- **heapless** - Data structures without heap
- **nb** - Non-blocking I/O
- **RTIC** - Real-time framework

## Use Cases

✅ **IoT Devices** - Sensors, actuators, gateways
✅ **Wearables** - Smartwatches, fitness trackers
✅ **Robotics** - Motor controllers, sensors
✅ **Industrial** - PLCs, process control
✅ **Automotive** - ECUs, body control modules
✅ **Medical** - Embedded medical devices

## Challenges

1. **Implement PWM** - Pulse-width modulation for LED dimming
2. **Build a Thermometer** - Read sensor, display on LCD
3. **Create a Data Logger** - Store sensor data to SD card
4. **Multi-tasking** - Run multiple tasks with RTIC
5. **Bootloader** - Write your own bootloader
6. **DMA** - Direct Memory Access for efficient I/O

## Safety Guarantees

Rust prevents:
- ❌ Buffer overflows
- ❌ Null pointer dereferences
- ❌ Data races in interrupts
- ❌ Use-after-free
- ❌ Uninitialized memory

## Resources

- [Embedded Rust Book](https://doc.rust-lang.org/embedded-book/)
- [Discovery Book](https://docs.rust-embedded.org/discovery/)
- [embedded-hal Documentation](https://docs.rs/embedded-hal/)
- [RTIC Book](https://rtic.rs/)
- [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)

## Next Steps

After completing this section:
- Build a complete IoT project
- Contribute to embedded-hal
- Explore async embedded with Embassy
- Learn about formal verification

---

**Estimated Time**: 16-24 hours
**Difficulty**: ★★★★★ (Expert)
**Prerequisites**: Strong Rust knowledge, embedded systems basics, hardware
