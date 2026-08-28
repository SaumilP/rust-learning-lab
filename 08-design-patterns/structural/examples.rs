// Structural Patterns Examples

// ============= Adapter Pattern Example =============
pub trait ModernApi {
    fn fetch_data(&self) -> Vec<String>;
}

pub struct LegacySystem;

impl LegacySystem {
    pub fn get_records(&self) -> Vec<(u32, String)> {
        vec![(1, "Record1".to_string()), (2, "Record2".to_string())]
    }
}

pub struct LegacyToModernAdapter {
    legacy: LegacySystem,
}

impl LegacyToModernAdapter {
    pub fn new(legacy: LegacySystem) -> Self {
        Self { legacy }
    }
}

impl ModernApi for LegacyToModernAdapter {
    fn fetch_data(&self) -> Vec<String> {
        self.legacy
            .get_records()
            .iter()
            .map(|(_, data)| data.clone())
            .collect()
    }
}

// ============= Decorator Pattern Example =============
pub trait Logger {
    fn log(&self, message: &str);
}

pub struct SimpleLogger;

impl Logger for SimpleLogger {
    fn log(&self, message: &str) {
        println!("LOG: {}", message);
    }
}

pub struct TimestampDecorator {
    inner: Box<dyn Logger>,
}

impl TimestampDecorator {
    pub fn new(inner: Box<dyn Logger>) -> Self {
        Self { inner }
    }
}

impl Logger for TimestampDecorator {
    fn log(&self, message: &str) {
        let timestamp = "2024-01-15 10:30:45";
        self.inner.log(&format!("[{}] {}", timestamp, message));
    }
}

pub struct LevelDecorator {
    inner: Box<dyn Logger>,
    level: String,
}

impl LevelDecorator {
    pub fn new(inner: Box<dyn Logger>, level: &str) -> Self {
        Self {
            inner,
            level: level.to_string(),
        }
    }
}

impl Logger for LevelDecorator {
    fn log(&self, message: &str) {
        self.inner.log(&format!("[{}] {}", self.level, message));
    }
}

// ============= Facade Pattern Example =============
pub struct EmailClient;

impl EmailClient {
    pub fn validate_email(&self, email: &str) -> bool {
        email.contains('@')
    }

    pub fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<String, String> {
        println!("Sending '{subject}' to {to}: {body}");
        Ok("Email sent successfully".to_string())
    }
}

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn render_template(&self, template: &str, data: &str) -> String {
        format!("{} with {}", template, data)
    }
}

pub struct NotificationService {
    email: EmailClient,
    template: TemplateEngine,
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            email: EmailClient,
            template: TemplateEngine,
        }
    }

    // Simplified facade method
    pub fn send_notification(
        &self,
        to: &str,
        template_name: &str,
        data: &str,
    ) -> Result<String, String> {
        if !self.email.validate_email(to) {
            return Err("Invalid email".to_string());
        }

        let body = self.template.render_template(template_name, data);
        self.email.send_email(to, "Notification", &body)?;

        Ok("Notification sent".to_string())
    }
}

// ============= Usage Example =============
pub fn example_structural_patterns() {
    println!("=== Adapter Pattern ===");
    let legacy = LegacySystem;
    let adapter = LegacyToModernAdapter::new(legacy);
    println!("Data: {:?}", adapter.fetch_data());

    println!("\n=== Decorator Pattern ===");
    let simple_logger = Box::new(SimpleLogger);
    let with_timestamp = Box::new(TimestampDecorator::new(simple_logger));
    let with_level = Box::new(LevelDecorator::new(with_timestamp, "ERROR"));
    with_level.log("Something went wrong!");

    println!("\n=== Facade Pattern ===");
    let notifier = NotificationService::new();
    match notifier.send_notification("user@example.com", "welcome_template", "Alice") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    example_structural_patterns();
}
