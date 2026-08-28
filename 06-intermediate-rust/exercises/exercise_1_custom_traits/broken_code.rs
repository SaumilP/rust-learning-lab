trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    title: String,
}

struct Video {
    title: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("Article: {}", self.title)
    }
}

// BUG: Video is missing its Summary implementation.

// BUG: This takes a concrete Article instead of borrowing any Summary type.
fn print_summary(item: Article) {
    println!("{}", item.summary());
}

fn main() {
    let article = Article {
        title: String::from("Rust ownership"),
    };
    let video = Video {
        title: String::from("Traits in ten minutes"),
    };

    print_summary(article);
    print_summary(video);
}
