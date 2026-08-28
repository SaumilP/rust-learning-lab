//! Composite: treat individual values and groups through one operation.

enum FileNode {
    File {
        name: String,
        bytes: u64,
    },
    Directory {
        name: String,
        children: Vec<FileNode>,
    },
}

impl FileNode {
    fn name(&self) -> &str {
        match self {
            Self::File { name, .. } | Self::Directory { name, .. } => name,
        }
    }

    fn size(&self) -> u64 {
        match self {
            Self::File { bytes, .. } => *bytes,
            Self::Directory { children, .. } => children.iter().map(Self::size).sum(),
        }
    }
}

fn main() {
    let project = FileNode::Directory {
        name: "project".to_string(),
        children: vec![
            FileNode::File {
                name: "main.rs".to_string(),
                bytes: 120,
            },
            FileNode::File {
                name: "README.md".to_string(),
                bytes: 80,
            },
        ],
    };
    println!("{} uses {} bytes", project.name(), project.size());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_size_is_the_sum_of_its_children() {
        let directory = FileNode::Directory {
            name: "docs".to_string(),
            children: vec![
                FileNode::File {
                    name: "a".to_string(),
                    bytes: 10,
                },
                FileNode::File {
                    name: "b".to_string(),
                    bytes: 15,
                },
            ],
        };
        assert_eq!(directory.size(), 25);
    }
}
