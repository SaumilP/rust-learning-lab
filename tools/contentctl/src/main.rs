use std::{env, path::PathBuf, process::ExitCode};

use contentctl::{
    export_path, unresolved_prerequisites, validate_interview_path, validate_path,
    validate_quiz_path,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("contentctl: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or_else(usage)?;
    let path = PathBuf::from(args.next().ok_or_else(usage)?);

    if args.next().is_some() {
        return Err(usage());
    }

    match command.as_str() {
        "validate" => {
            let topics = validate_path(&path).map_err(|errors| errors.join("\n"))?;
            for (topic, prerequisite) in unresolved_prerequisites(&topics) {
                eprintln!(
                    "contentctl: warning: topic `{topic}` references `{prerequisite}`, which has no metadata in this validation scope"
                );
            }
            println!("Validated {} topic metadata file(s).", topics.len());
            Ok(())
        }
        "export" => {
            let json = export_path(&path).map_err(|errors| errors.join("\n"))?;
            println!("{json}");
            Ok(())
        }
        "validate-quiz" => {
            let items = validate_quiz_path(&path).map_err(|errors| errors.join("\n"))?;
            println!("Validated {} quiz item(s).", items.len());
            Ok(())
        }
        "validate-interview" => {
            let categories = validate_interview_path(&path).map_err(|errors| errors.join("\n"))?;
            println!("Validated {} interview category file(s).", categories.len());
            Ok(())
        }
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage: contentctl <validate|export|validate-quiz|validate-interview> <file-or-directory>"
        .to_owned()
}
