use serde_json::{from_str, Value};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const LANGUAGES_DIR: &str = "/usr/local/bin/languages";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <OATLANG code or path to .oat file> <language>",
            args[0]
        );
        std::process::exit(1);
    }

    let oatlang_input = &args[1];
    let language = &args[2];

    let (oatlang_code, base_name) = if Path::new(oatlang_input).is_file() {
        let path = Path::new(oatlang_input);
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        (
            fs::read_to_string(oatlang_input).expect("Failed to read .oat file"),
            file_name.to_string(),
        )
    } else {
        ("main".to_string(), oatlang_input.clone())
    };

    let resolved_language = resolve_language(language);
    let file_path = PathBuf::from(LANGUAGES_DIR).join(format!("{}.json", resolved_language));
    let json_content = fs::read_to_string(&file_path).expect(&format!(
        "Could not read language file: {}",
        file_path.display()
    ));
    let mappings: Value = from_str(&json_content).expect("JSON was not well-formatted");

    let result = process_commands(&oatlang_code, &mappings, resolved_language);

    let output_file_bin = base_name.clone();
    let output_file_c = format!("{}.c", base_name);
    let output_file = format!("{}.{}", base_name, language.to_lowercase());

    if language.to_lowercase() == "bin" {
        fs::write(&output_file_c, result).expect("Unable to write C file");
        compile_and_convert_to_binary(&output_file_c, &output_file_bin);
    } else {
        fs::write(&output_file, result).expect("Unable to write file");
        println!("Output written to {}", output_file);
    }
}

fn process_commands(commands: &str, mappings: &Value, language: &str) -> String {
    let mut output = mappings["HEAD"].as_str().unwrap_or_default().to_string();

    for command in commands.split('$') {
        if command.trim().is_empty() {
            continue; // Skip empty commands due to trailing $
        }
        let expanded_command = expand_command(command, mappings, language);
        output.push_str(&expanded_command);
        output.push('\n'); // Add a newline for separation between commands
    }

    output.push_str(mappings["FOOT"].as_str().unwrap_or_default());
    output
}

fn preprocess_boolean_literals(command: &str, language: &str) -> String {
    match language.to_lowercase().as_str() {
        "py" => command.replace("true", "True").replace("false", "False"),
        "js" => command.replace("True", "true").replace("False", "false"),
        "c" => command
            .replace("True", "1")
            .replace("False", "0")
            .replace("true", "1")
            .replace("false", "0"),
        "cpp" => command
            .replace("True", "1")
            .replace("False", "0")
            .replace("true", "1")
            .replace("false", "0"),
        _ => command.to_string(), // Default case, no changes
    }
}

fn expand_command(command: &str, mappings: &Value, language: &str) -> String {
    let command = preprocess_boolean_literals(command, language); // Adjust boolean literals based on the target language
    let parts: Vec<&str> = command.splitn(2, '{').collect();
    if parts.len() < 2 {
        return format!("Invalid command format: {}", command);
    }
    let cmd_key = parts[0].trim();
    let args_str = parts[1].trim_end_matches('}').trim_end_matches('$');
    let arguments: Vec<&str> = args_str.split(',').map(|arg| arg.trim()).collect();

    let template = mappings[cmd_key].as_str().unwrap_or("");
    if template.is_empty() {
        return format!("Invalid command format: {}", cmd_key);
    }

    let mut result = template.to_string();
    for (i, arg) in arguments.iter().enumerate() {
        if arg.contains('{') {
            result = result.replace(
                &format!("{{{}}}", i),
                &expand_command(arg, mappings, language),
            );
        } else {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
    }
    result
}

fn compile_and_convert_to_binary(output_file_c: &str, output_file_bin: &str) {
    let compile_status = Command::new("gcc")
        .arg(output_file_c)
        .arg("-o")
        .arg(output_file_bin)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to compile C code");

    if compile_status.success() {
        println!("Compiled to binary {}", output_file_bin);
        if let Err(e) = fs::remove_file(output_file_c) {
            eprintln!(
                "Warning: Could not delete intermediate C file {}: {}",
                output_file_c, e
            );
        }
    } else {
        eprintln!(
            "Failed to compile {} to binary {}",
            output_file_c, output_file_bin
        );
    }
}

fn resolve_language(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "bin" => "c",
        _ => language,
    }
}
