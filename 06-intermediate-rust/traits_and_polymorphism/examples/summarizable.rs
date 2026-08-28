trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
}

struct Status {
    message: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Summary for Status {
    fn summary(&self) -> String {
        format!("Status: {}", self.message)
    }
}

fn announce(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    let article = Article {
        title: String::from("Learning traits"),
    };
    announce(&article);

    let feed: Vec<Box<dyn Summary>> = vec![
        Box::new(article),
        Box::new(Status {
            message: String::from("Examples compile"),
        }),
    ];

    for item in feed {
        println!("{}", item.summary());
    }
}
