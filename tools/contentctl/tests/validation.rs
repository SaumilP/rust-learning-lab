use std::{fs, path::Path};

use contentctl::{export_path, validate_file, validate_path};
use serde_json::{json, Value};
use tempfile::tempdir;

fn valid_topic(id: &str) -> Value {
    json!({
        "schema_version": 1,
        "id": id,
        "title": "A useful topic",
        "status": "review",
        "level": "beginner",
        "estimated_minutes": 30,
        "prerequisites": [],
        "concepts": ["rust"],
        "tracks": ["core"],
        "learning_objectives": ["Explain the demonstrated Rust concept."],
        "website": { "published": false, "order": null },
        "validation": {
            "content_rubric": "pending",
            "examples": "passed",
            "exercises": "not_applicable"
        }
    })
}

fn write_topic(path: &Path, value: &Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

#[test]
fn accepts_the_documented_example() {
    let path = Path::new("../../docs/examples/topic-metadata.json");
    let metadata = validate_file(path).unwrap();
    assert_eq!(metadata.id, "ownership-borrowing");
}

#[test]
fn rejects_unknown_fields() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("metadata.json");
    let mut topic = valid_topic("unknown-field");
    topic["unexpected"] = json!(true);
    write_topic(&path, &topic);

    let errors = validate_file(&path).unwrap_err();
    assert!(errors.join("\n").contains("unknown field `unexpected`"));
}

#[test]
fn rejects_stable_topic_without_complete_evidence() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("metadata.json");
    let mut topic = valid_topic("premature-stable");
    topic["status"] = json!("stable");
    write_topic(&path, &topic);

    let errors = validate_file(&path).unwrap_err();
    assert!(errors
        .join("\n")
        .contains("Stable requires a passed content rubric"));
}

#[test]
fn rejects_publishing_non_stable_content() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("metadata.json");
    let mut topic = valid_topic("unreviewed-publication");
    topic["website"] = json!({ "published": true, "order": 10 });
    write_topic(&path, &topic);

    let errors = validate_file(&path).unwrap_err();
    assert!(errors
        .join("\n")
        .contains("only Stable topics may be published"));
}

#[test]
fn rejects_duplicate_ids_in_a_directory() {
    let directory = tempdir().unwrap();
    write_topic(
        &directory.path().join("one/metadata.json"),
        &valid_topic("same-id"),
    );
    write_topic(
        &directory.path().join("two/metadata.json"),
        &valid_topic("same-id"),
    );

    let errors = validate_path(directory.path()).unwrap_err();
    assert!(errors.join("\n").contains("duplicate topic id `same-id`"));
}

#[test]
fn export_is_sorted_and_deterministic() {
    let directory = tempdir().unwrap();
    write_topic(
        &directory.path().join("z/metadata.json"),
        &valid_topic("z-last"),
    );
    write_topic(
        &directory.path().join("a/metadata.json"),
        &valid_topic("a-first"),
    );

    let first = export_path(directory.path()).unwrap();
    let second = export_path(directory.path()).unwrap();
    assert_eq!(first, second);
    assert!(first.find("a-first").unwrap() < first.find("z-last").unwrap());
    assert!(!first.contains(&directory.path().to_string_lossy().to_string()));
    assert!(first.contains("a/metadata.json"));
}
