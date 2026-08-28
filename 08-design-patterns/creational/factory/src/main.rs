// Factory Pattern in Rust
// Creates objects without specifying the exact class to create

fn main() {
    println!("=== Factory Pattern in Rust ===\n");

    // APPROACH 1: Enum-based (Idiomatic Rust)
    println!("1. ENUM-BASED FACTORY (Idiomatic Rust):\n");

    let dog = AnimalFactory::create(AnimalType::Dog);
    let cat = AnimalFactory::create(AnimalType::Cat);
    let bird = AnimalFactory::create(AnimalType::Bird);

    println!("   {}", dog.speak());
    println!("   {}", cat.speak());
    println!("   {}", bird.speak());

    println!("\n   ✓ Enums provide type-safe, exhaustive matching\n");

    // APPROACH 2: Trait object factory (OOP style)
    println!("2. TRAIT OBJECT FACTORY (OOP Style):\n");

    let dog = create_animal("dog");
    let cat = create_animal("cat");
    let bird = create_animal("bird");

    if let Some(animal) = dog {
        println!("   {}", animal.make_sound());
    }
    if let Some(animal) = cat {
        println!("   {}", animal.make_sound());
    }
    if let Some(animal) = bird {
        println!("   {}", animal.make_sound());
    }

    println!("\n   ✓ Trait objects allow runtime polymorphism\n");

    // APPROACH 3: Factory with configuration
    println!("3. CONFIGURABLE FACTORY:\n");

    let vehicles = vec![
        VehicleFactory::create("car", "Toyota", 4),
        VehicleFactory::create("motorcycle", "Harley", 2),
        VehicleFactory::create("truck", "Ford", 6),
    ];

    for vehicle in vehicles.into_iter().flatten() {
        vehicle.display();
    }

    println!("\n   ✓ Factory methods can accept configuration parameters\n");

    // APPROACH 4: Abstract factory (product families)
    println!("4. ABSTRACT FACTORY (Product Families):\n");

    let modern_factory = ModernFurnitureFactory;
    let victorian_factory = VictorianFurnitureFactory;

    println!("   Modern furniture:");
    let modern_chair = modern_factory.create_chair();
    let modern_table = modern_factory.create_table();
    modern_chair.sit_on();
    modern_table.place_item();

    println!("\n   Victorian furniture:");
    let victorian_chair = victorian_factory.create_chair();
    let victorian_table = victorian_factory.create_table();
    victorian_chair.sit_on();
    victorian_table.place_item();

    println!("\n   ✓ Abstract factory creates families of related objects\n");

    // PRACTICAL EXAMPLE: Document creation
    println!("5. PRACTICAL EXAMPLE: Document Factory:\n");

    let documents = [
        DocumentFactory::create("pdf", "Report"),
        DocumentFactory::create("word", "Letter"),
        DocumentFactory::create("html", "WebPage"),
    ];

    for doc in documents.iter().flatten() {
        doc.render();
    }

    println!("\n   ✓ Real-world usage for creating different document types\n");
}

// ========== APPROACH 1: Enum-based Factory ==========

#[derive(Debug)]
enum AnimalType {
    Dog,
    Cat,
    Bird,
}

#[derive(Debug)]
enum Animal {
    Dog { name: String },
    Cat { name: String },
    Bird { name: String },
}

impl Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Dog { name } => format!("{} says: Woof!", name),
            Animal::Cat { name } => format!("{} says: Meow!", name),
            Animal::Bird { name } => format!("{} says: Tweet!", name),
        }
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn create(animal_type: AnimalType) -> Animal {
        match animal_type {
            AnimalType::Dog => Animal::Dog {
                name: "Buddy".to_string(),
            },
            AnimalType::Cat => Animal::Cat {
                name: "Whiskers".to_string(),
            },
            AnimalType::Bird => Animal::Bird {
                name: "Tweety".to_string(),
            },
        }
    }
}

// ========== APPROACH 2: Trait Object Factory ==========

trait SoundMaker {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;
struct Bird;

impl SoundMaker for Dog {
    fn make_sound(&self) -> String {
        "Dog: Woof!".to_string()
    }
}

impl SoundMaker for Cat {
    fn make_sound(&self) -> String {
        "Cat: Meow!".to_string()
    }
}

impl SoundMaker for Bird {
    fn make_sound(&self) -> String {
        "Bird: Tweet!".to_string()
    }
}

fn create_animal(animal_type: &str) -> Option<Box<dyn SoundMaker>> {
    match animal_type {
        "dog" => Some(Box::new(Dog)),
        "cat" => Some(Box::new(Cat)),
        "bird" => Some(Box::new(Bird)),
        _ => None,
    }
}

// ========== APPROACH 3: Configurable Factory ==========

trait Vehicle {
    fn display(&self);
}

struct Car {
    brand: String,
    wheels: u8,
}

struct Motorcycle {
    brand: String,
    wheels: u8,
}

struct Truck {
    brand: String,
    wheels: u8,
}

impl Vehicle for Car {
    fn display(&self) {
        println!("   Car: {} with {} wheels", self.brand, self.wheels);
    }
}

impl Vehicle for Motorcycle {
    fn display(&self) {
        println!("   Motorcycle: {} with {} wheels", self.brand, self.wheels);
    }
}

impl Vehicle for Truck {
    fn display(&self) {
        println!("   Truck: {} with {} wheels", self.brand, self.wheels);
    }
}

struct VehicleFactory;

impl VehicleFactory {
    fn create(vehicle_type: &str, brand: &str, wheels: u8) -> Option<Box<dyn Vehicle>> {
        match vehicle_type {
            "car" => Some(Box::new(Car {
                brand: brand.to_string(),
                wheels,
            })),
            "motorcycle" => Some(Box::new(Motorcycle {
                brand: brand.to_string(),
                wheels,
            })),
            "truck" => Some(Box::new(Truck {
                brand: brand.to_string(),
                wheels,
            })),
            _ => None,
        }
    }
}

// ========== APPROACH 4: Abstract Factory ==========

trait Chair {
    fn sit_on(&self);
}

trait Table {
    fn place_item(&self);
}

struct ModernChair;
struct ModernTable;
struct VictorianChair;
struct VictorianTable;

impl Chair for ModernChair {
    fn sit_on(&self) {
        println!("   Sitting on a sleek modern chair");
    }
}

impl Table for ModernTable {
    fn place_item(&self) {
        println!("   Placing item on minimalist modern table");
    }
}

impl Chair for VictorianChair {
    fn sit_on(&self) {
        println!("   Sitting on an ornate Victorian chair");
    }
}

impl Table for VictorianTable {
    fn place_item(&self) {
        println!("   Placing item on elaborate Victorian table");
    }
}

trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_table(&self) -> Box<dyn Table>;
}

struct ModernFurnitureFactory;
struct VictorianFurnitureFactory;

impl FurnitureFactory for ModernFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair)
    }

    fn create_table(&self) -> Box<dyn Table> {
        Box::new(ModernTable)
    }
}

impl FurnitureFactory for VictorianFurnitureFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair)
    }

    fn create_table(&self) -> Box<dyn Table> {
        Box::new(VictorianTable)
    }
}

// ========== PRACTICAL EXAMPLE: Documents ==========

trait Document {
    fn render(&self);
}

struct PdfDocument {
    title: String,
}

struct WordDocument {
    title: String,
}

struct HtmlDocument {
    title: String,
}

impl Document for PdfDocument {
    fn render(&self) {
        println!("   Rendering PDF: {}.pdf", self.title);
    }
}

impl Document for WordDocument {
    fn render(&self) {
        println!("   Rendering Word: {}.docx", self.title);
    }
}

impl Document for HtmlDocument {
    fn render(&self) {
        println!("   Rendering HTML: {}.html", self.title);
    }
}

struct DocumentFactory;

impl DocumentFactory {
    fn create(doc_type: &str, title: &str) -> Option<Box<dyn Document>> {
        match doc_type {
            "pdf" => Some(Box::new(PdfDocument {
                title: title.to_string(),
            })),
            "word" => Some(Box::new(WordDocument {
                title: title.to_string(),
            })),
            "html" => Some(Box::new(HtmlDocument {
                title: title.to_string(),
            })),
            _ => None,
        }
    }
}

/*
Factory Pattern Summary:
========================

WHEN TO USE:
- Object creation is complex
- Need to centralize creation logic
- Want to decouple client from concrete types
- Need to create families of related objects

RUST APPROACHES:

1. ENUM-BASED (Preferred):
   - Type-safe, exhaustive matching
   - No heap allocation
   - Best performance
   - Compile-time errors

2. TRAIT OBJECTS:
   - Runtime polymorphism
   - Extensible (can add types)
   - Heap allocation required
   - Dynamic dispatch cost

3. ABSTRACT FACTORY:
   - Creates families of objects
   - Ensures compatibility
   - More complex setup

BENEFITS:
- Encapsulates object creation
- Single responsibility (creation logic)
- Makes code more maintainable
- Easy to extend

TRADEOFFS:
- Enum: Less flexible but safer
- Trait objects: More flexible but runtime cost
- Abstract factory: Ensures compatibility but complex

COMPARED TO OTHER LANGUAGES:
Java:    Uses classes and inheritance
Python:  Uses functions or classes
Go:      Uses functions returning interfaces
Rust:    Enums (preferred) or trait objects

Run this:
    cargo run

Experiment:
    - Add new animal types
    - Create your own factory
    - Compare enum vs trait object performance
    - Implement abstract factory for different themes
*/
