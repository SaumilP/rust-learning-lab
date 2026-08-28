//! Bridge: vary an abstraction and its implementation independently.

trait Device {
    fn name(&self) -> &'static str;
    fn turn_on(&mut self);
    fn is_on(&self) -> bool;
}

struct Television {
    on: bool,
}
struct Radio {
    on: bool,
}

impl Device for Television {
    fn name(&self) -> &'static str {
        "television"
    }
    fn turn_on(&mut self) {
        self.on = true;
    }
    fn is_on(&self) -> bool {
        self.on
    }
}

impl Device for Radio {
    fn name(&self) -> &'static str {
        "radio"
    }
    fn turn_on(&mut self) {
        self.on = true;
    }
    fn is_on(&self) -> bool {
        self.on
    }
}

struct Remote<D> {
    device: D,
}

impl<D: Device> Remote<D> {
    fn power_on(&mut self) -> String {
        self.device.turn_on();
        format!("{} is on: {}", self.device.name(), self.device.is_on())
    }
}

fn main() {
    let mut tv_remote = Remote {
        device: Television { on: false },
    };
    let mut radio_remote = Remote {
        device: Radio { on: false },
    };
    println!("{}", tv_remote.power_on());
    println!("{}", radio_remote.power_on());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_abstraction_controls_different_implementations() {
        let mut remote = Remote {
            device: Television { on: false },
        };
        assert_eq!(remote.power_on(), "television is on: true");
    }
}
