//! Facade: expose one simple operation over several cooperating subsystems.

struct Inventory;

impl Inventory {
    fn reserve(&self, item: &str) -> bool {
        item == "Rust Book"
    }
}

struct PaymentGateway;

impl PaymentGateway {
    fn charge(&self, amount: u32) -> bool {
        amount > 0
    }
}

struct Shipping;

impl Shipping {
    fn create_label(&self, item: &str) -> String {
        format!("SHIP-{item}")
    }
}

struct CheckoutFacade {
    inventory: Inventory,
    payments: PaymentGateway,
    shipping: Shipping,
}

impl CheckoutFacade {
    fn new() -> Self {
        Self {
            inventory: Inventory,
            payments: PaymentGateway,
            shipping: Shipping,
        }
    }

    fn checkout(&self, item: &str, price: u32) -> Result<String, String> {
        if !self.inventory.reserve(item) {
            return Err(format!("{item} is out of stock"));
        }
        if !self.payments.charge(price) {
            return Err("payment was declined".to_string());
        }
        Ok(self.shipping.create_label(item))
    }
}

fn main() {
    let checkout = CheckoutFacade::new();
    match checkout.checkout("Rust Book", 40) {
        Ok(label) => println!("Order accepted. Shipping label: {label}"),
        Err(error) => eprintln!("Checkout failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_coordinates_the_complete_checkout() {
        let checkout = CheckoutFacade::new();
        assert_eq!(
            checkout.checkout("Rust Book", 40),
            Ok("SHIP-Rust Book".to_string())
        );
    }

    #[test]
    fn facade_reports_a_subsystem_failure() {
        let checkout = CheckoutFacade::new();
        assert_eq!(
            checkout.checkout("Other Book", 40),
            Err("Other Book is out of stock".to_string())
        );
    }
}
