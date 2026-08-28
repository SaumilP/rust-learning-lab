//! State: keep valid state transitions in one place.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrderState {
    Draft,
    Paid,
    Shipped,
}

struct Order {
    state: OrderState,
}

impl Order {
    fn new() -> Self {
        Self {
            state: OrderState::Draft,
        }
    }

    fn pay(&mut self) -> Result<(), &'static str> {
        match self.state {
            OrderState::Draft => {
                self.state = OrderState::Paid;
                Ok(())
            }
            _ => Err("only a draft order can be paid"),
        }
    }

    fn ship(&mut self) -> Result<(), &'static str> {
        match self.state {
            OrderState::Paid => {
                self.state = OrderState::Shipped;
                Ok(())
            }
            _ => Err("only a paid order can be shipped"),
        }
    }

    fn state(&self) -> OrderState {
        self.state
    }
}

fn main() -> Result<(), &'static str> {
    let mut order = Order::new();
    println!("Created: {:?}", order.state());
    order.pay()?;
    println!("Paid: {:?}", order.state());
    order.ship()?;
    println!("Shipped: {:?}", order.state());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_valid_order_workflow() {
        let mut order = Order::new();
        order.pay().unwrap();
        order.ship().unwrap();
        assert_eq!(order.state(), OrderState::Shipped);
    }

    #[test]
    fn rejects_shipping_before_payment() {
        assert_eq!(Order::new().ship(), Err("only a paid order can be shipped"));
    }
}
