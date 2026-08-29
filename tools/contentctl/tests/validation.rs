use std::{fs, path::Path};

use contentctl::{
    export_path, unresolved_prerequisites, validate_file, validate_path, validate_quiz_file,
    validate_quiz_path,
};
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

fn valid_quiz(id: &str) -> Value {
    json!({
        "schema_version": 1,
        "id": id,
        "type": "multiple_choice",
        "topic": "ownership",
        "difficulty": "beginner",
        "concepts": ["ownership"],
        "question": "Which choice is correct?",
        "choices": [
            { "id": "correct", "text": "The correct choice" },
            { "id": "incorrect", "text": "The incorrect choice" }
        ],
        "answer": { "kind": "choice", "value": "correct" },
        "explanation": "It matches the stated rule.",
        "references": [{ "title": "Ownership", "path": "README.md" }]
    })
}

fn write_quiz(path: &Path, value: &Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

fn published_stable_topic(id: &str) -> Value {
    let mut topic = valid_topic(id);
    topic["status"] = json!("stable");
    topic["website"] = json!({ "published": true, "order": 10 });
    topic["validation"] = json!({
        "content_rubric": "passed",
        "examples": "passed",
        "exercises": "not_applicable"
    });
    topic
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
fn rejects_duplicate_ids_even_when_one_file_has_other_validation_errors() {
    let directory = tempdir().unwrap();
    write_topic(
        &directory.path().join("one/metadata.json"),
        &valid_topic("same-id"),
    );
    let mut invalid_duplicate = valid_topic("same-id");
    invalid_duplicate["estimated_minutes"] = json!(0);
    write_topic(
        &directory.path().join("two/metadata.json"),
        &invalid_duplicate,
    );

    let errors = validate_path(directory.path()).unwrap_err().join("\n");
    assert!(errors.contains("estimated_minutes must be greater than zero"));
    assert!(errors.contains("duplicate topic id `same-id`"));
}

#[test]
fn rejects_a_published_topic_with_an_unmigrated_prerequisite() {
    let directory = tempdir().unwrap();
    let mut topic = published_stable_topic("published-topic");
    topic["prerequisites"] = json!(["missing-topic"]);
    write_topic(&directory.path().join("metadata.json"), &topic);

    let errors = validate_path(directory.path()).unwrap_err().join("\n");
    assert!(errors.contains("published topic prerequisite `missing-topic` has no metadata"));
}

#[test]
fn accepts_a_published_topic_with_a_migrated_prerequisite() {
    let directory = tempdir().unwrap();
    write_topic(
        &directory.path().join("required/metadata.json"),
        &published_stable_topic("required-topic"),
    );
    let mut topic = published_stable_topic("published-topic");
    topic["website"]["order"] = json!(20);
    topic["prerequisites"] = json!(["required-topic"]);
    write_topic(&directory.path().join("published/metadata.json"), &topic);

    assert!(validate_path(directory.path()).is_ok());
}

#[test]
fn reports_unmigrated_prerequisites_without_blocking_unpublished_topics() {
    let directory = tempdir().unwrap();
    let mut topic = valid_topic("review-topic");
    topic["prerequisites"] = json!(["future-topic"]);
    write_topic(&directory.path().join("metadata.json"), &topic);

    let topics = validate_path(directory.path()).unwrap();
    assert_eq!(
        unresolved_prerequisites(&topics),
        vec![("review-topic".to_owned(), "future-topic".to_owned())]
    );
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

#[test]
fn accepts_the_first_foundation_quiz_item() {
    let path = Path::new("../../challenges/quiz-items/01-mutability-binding.json");
    let item = validate_quiz_file(path).unwrap();
    assert_eq!(item.id, "quiz-mutability-binding");
}

#[test]
fn accepts_all_fifty_foundation_quiz_items() {
    let items = validate_quiz_path(Path::new("../../challenges/quiz-items")).unwrap();
    assert_eq!(items.len(), 50);
}

#[test]
fn rejects_choice_questions_without_a_valid_answer_choice() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("quiz.json");
    let mut quiz = valid_quiz("bad-answer");
    quiz["answer"]["value"] = json!("missing");
    write_quiz(&path, &quiz);

    let errors = validate_quiz_file(&path).unwrap_err().join("\n");
    assert!(errors.contains("answer choice `missing` is not defined"));
}

#[test]
fn rejects_code_questions_without_code() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("quiz.json");
    let mut quiz = valid_quiz("missing-code");
    quiz["type"] = json!("will_it_compile");
    quiz["choices"] = json!([]);
    quiz["answer"] = json!({ "kind": "compiles", "value": "true" });
    write_quiz(&path, &quiz);

    let errors = validate_quiz_file(&path).unwrap_err().join("\n");
    assert!(errors.contains("this question type requires code"));
}

#[test]
fn rejects_duplicate_quiz_ids_in_a_directory() {
    let directory = tempdir().unwrap();
    write_quiz(&directory.path().join("one.json"), &valid_quiz("same-quiz"));
    write_quiz(&directory.path().join("two.json"), &valid_quiz("same-quiz"));

    let errors = validate_quiz_path(directory.path()).unwrap_err().join("\n");
    assert!(errors.contains("duplicate quiz item id `same-quiz`"));
}
