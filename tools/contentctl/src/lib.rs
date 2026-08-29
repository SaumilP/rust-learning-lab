use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

const SCHEMA_VERSION: u32 = 1;
const QUIZ_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TopicMetadata {
    #[serde(rename = "$schema", default, skip_serializing)]
    pub schema: Option<String>,
    pub schema_version: u32,
    pub id: String,
    pub title: String,
    pub status: Status,
    pub level: Level,
    pub estimated_minutes: u32,
    pub prerequisites: Vec<String>,
    pub concepts: Vec<String>,
    pub tracks: Vec<Track>,
    pub learning_objectives: Vec<String>,
    pub website: Website,
    pub validation: Validation,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Planned,
    Draft,
    Review,
    Stable,
    Deprecated,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Track {
    Core,
    Java,
    Python,
    Cpp,
    Go,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Website {
    pub published: bool,
    pub order: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Validation {
    pub content_rubric: Evidence,
    pub examples: Evidence,
    pub exercises: Evidence,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QuizItem {
    #[serde(rename = "$schema", default, skip_serializing)]
    pub schema: Option<String>,
    pub schema_version: u32,
    pub id: String,
    #[serde(rename = "type")]
    pub question_type: QuizType,
    pub topic: String,
    pub difficulty: QuizDifficulty,
    pub concepts: Vec<String>,
    pub question: String,
    pub code: Option<String>,
    pub choices: Vec<QuizChoice>,
    pub answer: QuizAnswer,
    pub explanation: String,
    pub references: Vec<QuizReference>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuizType {
    MultipleChoice,
    WillItCompile,
    PredictOutput,
    CodeReview,
    FixError,
    DesignChoice,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuizDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QuizChoice {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QuizAnswer {
    pub kind: QuizAnswerKind,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuizAnswerKind {
    Choice,
    Compiles,
    Output,
    Review,
    Fix,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct QuizReference {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    NotApplicable,
    Pending,
    Passed,
}

#[derive(Debug, Serialize)]
struct ExportedTopic {
    source_path: String,
    #[serde(flatten)]
    metadata: TopicMetadata,
}

pub fn validate_path(path: &Path) -> Result<Vec<(PathBuf, TopicMetadata)>, Vec<String>> {
    let files = metadata_files(path).map_err(|error| vec![error])?;
    if files.is_empty() {
        return Err(vec![format!(
            "{} contains no topic metadata JSON files",
            path.display()
        )]);
    }

    let mut topics = Vec::new();
    let mut errors = Vec::new();

    for file in files {
        match parse_file(&file) {
            Ok((metadata, mut file_errors)) => {
                topics.push((file, metadata));
                errors.append(&mut file_errors);
            }
            Err(mut file_errors) => errors.append(&mut file_errors),
        }
    }

    let mut ids = HashSet::new();
    for (file, topic) in &topics {
        if !ids.insert(&topic.id) {
            errors.push(format!(
                "{}: duplicate topic id `{}`",
                file.display(),
                topic.id
            ));
        }
    }

    for (file, topic) in &topics {
        if topic.website.published {
            for prerequisite in &topic.prerequisites {
                if !ids.contains(prerequisite) {
                    errors.push(format!(
                        "{}: published topic prerequisite `{prerequisite}` has no metadata in this validation scope",
                        file.display()
                    ));
                }
            }
        }
    }

    if errors.is_empty() {
        topics.sort_by(|left, right| left.1.id.cmp(&right.1.id).then(left.0.cmp(&right.0)));
        Ok(topics)
    } else {
        Err(errors)
    }
}

pub fn export_path(path: &Path) -> Result<String, Vec<String>> {
    let topics = validate_path(path)?;
    let directory_root = path.is_dir().then_some(path);
    let exported: Vec<_> = topics
        .into_iter()
        .map(|(source_path, metadata)| ExportedTopic {
            source_path: normalized_path(
                directory_root
                    .and_then(|root| source_path.strip_prefix(root).ok())
                    .unwrap_or(&source_path),
            ),
            metadata,
        })
        .collect();

    serde_json::to_string_pretty(&exported)
        .map(|json| format!("{json}\n"))
        .map_err(|error| vec![format!("could not serialize export: {error}")])
}

pub fn validate_file(path: &Path) -> Result<TopicMetadata, Vec<String>> {
    let (metadata, errors) = parse_file(path)?;

    if errors.is_empty() {
        Ok(metadata)
    } else {
        Err(errors)
    }
}

pub fn validate_quiz_path(path: &Path) -> Result<Vec<(PathBuf, QuizItem)>, Vec<String>> {
    let files = quiz_files(path).map_err(|error| vec![error])?;
    if files.is_empty() {
        return Err(vec![format!(
            "{} contains no quiz item JSON files",
            path.display()
        )]);
    }

    let mut items = Vec::new();
    let mut errors = Vec::new();
    for file in files {
        match parse_quiz_file(&file) {
            Ok((item, mut item_errors)) => {
                items.push((file, item));
                errors.append(&mut item_errors);
            }
            Err(mut item_errors) => errors.append(&mut item_errors),
        }
    }

    let mut ids = HashSet::new();
    for (file, item) in &items {
        if !ids.insert(&item.id) {
            errors.push(format!(
                "{}: duplicate quiz item id `{}`",
                file.display(),
                item.id
            ));
        }
    }

    if errors.is_empty() {
        items.sort_by(|left, right| left.1.id.cmp(&right.1.id));
        Ok(items)
    } else {
        Err(errors)
    }
}

pub fn validate_quiz_file(path: &Path) -> Result<QuizItem, Vec<String>> {
    let (item, errors) = parse_quiz_file(path)?;
    if errors.is_empty() {
        Ok(item)
    } else {
        Err(errors)
    }
}

pub fn unresolved_prerequisites(topics: &[(PathBuf, TopicMetadata)]) -> Vec<(String, String)> {
    let ids: HashSet<_> = topics.iter().map(|(_, topic)| topic.id.as_str()).collect();
    let mut unresolved = topics
        .iter()
        .flat_map(|(_, topic)| {
            topic
                .prerequisites
                .iter()
                .filter(|prerequisite| !ids.contains(prerequisite.as_str()))
                .map(|prerequisite| (topic.id.clone(), prerequisite.clone()))
        })
        .collect::<Vec<_>>();
    unresolved.sort();
    unresolved
}

fn parse_file(path: &Path) -> Result<(TopicMetadata, Vec<String>), Vec<String>> {
    let contents = fs::read_to_string(path)
        .map_err(|error| vec![format!("{}: could not read file: {error}", path.display())])?;
    let metadata: TopicMetadata = serde_json::from_str(&contents)
        .map_err(|error| vec![format!("{}: invalid metadata: {error}", path.display())])?;
    let errors = semantic_errors(path, &metadata);
    Ok((metadata, errors))
}

fn parse_quiz_file(path: &Path) -> Result<(QuizItem, Vec<String>), Vec<String>> {
    let contents = fs::read_to_string(path)
        .map_err(|error| vec![format!("{}: could not read file: {error}", path.display())])?;
    let item: QuizItem = serde_json::from_str(&contents)
        .map_err(|error| vec![format!("{}: invalid quiz item: {error}", path.display())])?;
    let errors = quiz_semantic_errors(path, &item);
    Ok((item, errors))
}

fn metadata_files(path: &Path) -> Result<Vec<PathBuf>, String> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }
    if !path.is_dir() {
        return Err(format!("{} is not a file or directory", path.display()));
    }

    let mut files = Vec::new();
    collect_metadata_files(path, &mut files)?;
    files.sort();
    Ok(files)
}

fn quiz_files(path: &Path) -> Result<Vec<PathBuf>, String> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }
    if !path.is_dir() {
        return Err(format!("{} is not a file or directory", path.display()));
    }

    let mut files = Vec::new();
    collect_quiz_files(path, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_metadata_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("{}: could not read directory: {error}", directory.display()))?;

    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", directory.display()))?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .is_some_and(|name| name == "target" || name == ".git")
            {
                continue;
            }
            collect_metadata_files(&path, files)?;
        } else if path.file_name().is_some_and(|name| name == "metadata.json") {
            files.push(path);
        }
    }

    Ok(())
}

fn collect_quiz_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("{}: could not read directory: {error}", directory.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("{}: {error}", directory.display()))?;
        let path = entry.path();
        if path.is_dir() {
            if path
                .file_name()
                .is_some_and(|name| name == "target" || name == ".git")
            {
                continue;
            }
            collect_quiz_files(&path, files)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            files.push(path);
        }
    }
    Ok(())
}

fn semantic_errors(path: &Path, metadata: &TopicMetadata) -> Vec<String> {
    let mut errors = Vec::new();
    let prefix = || path.display().to_string();

    if metadata.schema_version != SCHEMA_VERSION {
        errors.push(format!(
            "{}: schema_version must be {SCHEMA_VERSION}",
            prefix()
        ));
    }
    if !is_slug(&metadata.id) {
        errors.push(format!(
            "{}: id must be a lowercase kebab-case slug",
            prefix()
        ));
    }
    if metadata.title.trim().is_empty() {
        errors.push(format!("{}: title must not be empty", prefix()));
    }
    if metadata.estimated_minutes == 0 {
        errors.push(format!(
            "{}: estimated_minutes must be greater than zero",
            prefix()
        ));
    }
    validate_unique_slugs(path, "prerequisites", &metadata.prerequisites, &mut errors);
    validate_unique_slugs(path, "concepts", &metadata.concepts, &mut errors);
    validate_non_empty(path, "concepts", &metadata.concepts, &mut errors);
    validate_non_empty(
        path,
        "learning_objectives",
        &metadata.learning_objectives,
        &mut errors,
    );

    if metadata.prerequisites.iter().any(|id| id == &metadata.id) {
        errors.push(format!("{}: a topic cannot require itself", prefix()));
    }
    if metadata.tracks.is_empty() {
        errors.push(format!(
            "{}: tracks must contain at least one entry",
            prefix()
        ));
    }
    if metadata
        .tracks
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .len()
        != metadata.tracks.len()
    {
        errors.push(format!("{}: tracks must not contain duplicates", prefix()));
    }
    if metadata
        .learning_objectives
        .iter()
        .any(|objective| objective.trim().is_empty())
    {
        errors.push(format!(
            "{}: learning_objectives must not contain empty text",
            prefix()
        ));
    }
    if metadata.website.published && metadata.status != Status::Stable {
        errors.push(format!("{}: only Stable topics may be published", prefix()));
    }
    if metadata.website.published && metadata.website.order.is_none() {
        errors.push(format!(
            "{}: a published topic requires website.order",
            prefix()
        ));
    }
    if metadata.status == Status::Stable
        && (metadata.validation.content_rubric != Evidence::Passed
            || !matches!(
                metadata.validation.examples,
                Evidence::Passed | Evidence::NotApplicable
            )
            || !matches!(
                metadata.validation.exercises,
                Evidence::Passed | Evidence::NotApplicable
            ))
    {
        errors.push(format!("{}: Stable requires a passed content rubric and passed or not-applicable automated evidence", prefix()));
    }

    errors
}

fn quiz_semantic_errors(path: &Path, item: &QuizItem) -> Vec<String> {
    let mut errors = Vec::new();
    let prefix = || path.display().to_string();
    if item.schema_version != QUIZ_SCHEMA_VERSION {
        errors.push(format!(
            "{}: schema_version must be {QUIZ_SCHEMA_VERSION}",
            prefix()
        ));
    }
    if !is_slug(&item.id) {
        errors.push(format!(
            "{}: id must be a lowercase kebab-case slug",
            prefix()
        ));
    }
    if !is_slug(&item.topic) {
        errors.push(format!(
            "{}: topic must be a lowercase kebab-case slug",
            prefix()
        ));
    }
    validate_non_empty(path, "concepts", &item.concepts, &mut errors);
    validate_unique_slugs(path, "concepts", &item.concepts, &mut errors);
    if item.question.trim().is_empty() {
        errors.push(format!("{}: question must not be empty", prefix()));
    }
    if item.explanation.trim().is_empty() {
        errors.push(format!("{}: explanation must not be empty", prefix()));
    }
    if item.references.is_empty() {
        errors.push(format!(
            "{}: references must contain at least one entry",
            prefix()
        ));
    }
    for reference in &item.references {
        if reference.title.trim().is_empty() || reference.path.trim().is_empty() {
            errors.push(format!(
                "{}: references must have non-empty title and path",
                prefix()
            ));
        }
    }
    if item
        .code
        .as_ref()
        .is_some_and(|code| code.trim().is_empty())
    {
        errors.push(format!(
            "{}: code must not be empty when supplied",
            prefix()
        ));
    }

    let code_required = matches!(
        item.question_type,
        QuizType::WillItCompile
            | QuizType::PredictOutput
            | QuizType::CodeReview
            | QuizType::FixError
    );
    if code_required && item.code.is_none() {
        errors.push(format!("{}: this question type requires code", prefix()));
    }
    let choice_required = matches!(
        item.question_type,
        QuizType::MultipleChoice | QuizType::DesignChoice
    );
    if choice_required && item.choices.len() < 2 {
        errors.push(format!(
            "{}: this question type requires at least two choices",
            prefix()
        ));
    }
    if !choice_required && !item.choices.is_empty() {
        errors.push(format!(
            "{}: only choice questions may define choices",
            prefix()
        ));
    }
    let mut choice_ids = HashSet::new();
    for choice in &item.choices {
        if !is_slug(&choice.id) || choice.text.trim().is_empty() {
            errors.push(format!(
                "{}: choices require a slug id and non-empty text",
                prefix()
            ));
        }
        if !choice_ids.insert(&choice.id) {
            errors.push(format!(
                "{}: choice id `{}` is duplicated",
                prefix(),
                choice.id
            ));
        }
    }

    let expected_answer = match item.question_type {
        QuizType::MultipleChoice | QuizType::DesignChoice => QuizAnswerKind::Choice,
        QuizType::WillItCompile => QuizAnswerKind::Compiles,
        QuizType::PredictOutput => QuizAnswerKind::Output,
        QuizType::CodeReview => QuizAnswerKind::Review,
        QuizType::FixError => QuizAnswerKind::Fix,
    };
    if item.answer.kind != expected_answer {
        errors.push(format!(
            "{}: answer kind does not match question type",
            prefix()
        ));
    }
    if item.answer.value.trim().is_empty() {
        errors.push(format!("{}: answer value must not be empty", prefix()));
    }
    if item.answer.kind == QuizAnswerKind::Choice && !choice_ids.contains(&item.answer.value) {
        errors.push(format!(
            "{}: answer choice `{}` is not defined",
            prefix(),
            item.answer.value
        ));
    }
    if item.answer.kind == QuizAnswerKind::Compiles
        && !matches!(item.answer.value.as_str(), "true" | "false")
    {
        errors.push(format!(
            "{}: compiles answer must be `true` or `false`",
            prefix()
        ));
    }
    errors
}

fn validate_unique_slugs(path: &Path, field: &str, values: &[String], errors: &mut Vec<String>) {
    let mut seen = HashSet::new();
    for value in values {
        if !is_slug(value) {
            errors.push(format!(
                "{}: {field} entry `{value}` must be a lowercase kebab-case slug",
                path.display()
            ));
        }
        if !seen.insert(value) {
            errors.push(format!(
                "{}: {field} entry `{value}` is duplicated",
                path.display()
            ));
        }
    }
}

fn validate_non_empty(path: &Path, field: &str, values: &[String], errors: &mut Vec<String>) {
    if values.is_empty() {
        errors.push(format!(
            "{}: {field} must contain at least one entry",
            path.display()
        ));
    }
}

fn is_slug(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.contains("--")
}

fn normalized_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
