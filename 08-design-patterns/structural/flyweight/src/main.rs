//! Flyweight: share immutable intrinsic data between many small objects.

use std::rc::Rc;

#[derive(Debug)]
struct TextStyle {
    font: String,
    size: u8,
}

struct Glyph {
    character: char,
    x: u16,
    style: Rc<TextStyle>,
}

fn main() {
    let body_style = Rc::new(TextStyle {
        font: "Mono".to_string(),
        size: 14,
    });
    let first = Glyph {
        character: 'R',
        x: 0,
        style: Rc::clone(&body_style),
    };
    let second = Glyph {
        character: 's',
        x: 1,
        style: Rc::clone(&body_style),
    };

    println!(
        "{}{} in {} {}pt",
        first.character, second.character, first.style.font, first.style.size
    );
    println!("positions: {}, {}", first.x, second.x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyphs_share_the_same_style_allocation() {
        let style = Rc::new(TextStyle {
            font: "Mono".to_string(),
            size: 12,
        });
        let a = Glyph {
            character: 'a',
            x: 0,
            style: Rc::clone(&style),
        };
        let b = Glyph {
            character: 'b',
            x: 1,
            style: Rc::clone(&style),
        };
        assert!(Rc::ptr_eq(&a.style, &b.style));
    }
}
