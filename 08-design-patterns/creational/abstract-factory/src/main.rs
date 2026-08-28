//! Abstract Factory: create a matching family of related products.

trait Button {
    fn render(&self) -> &'static str;
}

trait Checkbox {
    fn render(&self) -> &'static str;
}

trait UiFactory {
    fn button(&self) -> Box<dyn Button>;
    fn checkbox(&self) -> Box<dyn Checkbox>;
}

struct LightButton;
struct LightCheckbox;
struct DarkButton;
struct DarkCheckbox;

impl Button for LightButton {
    fn render(&self) -> &'static str {
        "light button"
    }
}
impl Checkbox for LightCheckbox {
    fn render(&self) -> &'static str {
        "light checkbox"
    }
}
impl Button for DarkButton {
    fn render(&self) -> &'static str {
        "dark button"
    }
}
impl Checkbox for DarkCheckbox {
    fn render(&self) -> &'static str {
        "dark checkbox"
    }
}

struct LightTheme;
struct DarkTheme;

impl UiFactory for LightTheme {
    fn button(&self) -> Box<dyn Button> {
        Box::new(LightButton)
    }
    fn checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(LightCheckbox)
    }
}

impl UiFactory for DarkTheme {
    fn button(&self) -> Box<dyn Button> {
        Box::new(DarkButton)
    }
    fn checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(DarkCheckbox)
    }
}

fn render_screen(factory: &dyn UiFactory) -> String {
    format!(
        "{}, {}",
        factory.button().render(),
        factory.checkbox().render()
    )
}

fn main() {
    println!("{}", render_screen(&LightTheme));
    println!("{}", render_screen(&DarkTheme));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_creates_a_matching_product_family() {
        assert_eq!(render_screen(&LightTheme), "light button, light checkbox");
    }
}
