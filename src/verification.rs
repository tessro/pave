//! Verification specification model for structured verification data.
//!
//! This module defines the data structures for representing verification commands
//! and expected behaviors extracted from PAVED documents.

use std::path::PathBuf;

use crate::parser::ParsedDoc;

/// A verification specification extracted from a PAVED document.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationSpec {
    /// Path to the source markdown file.
    pub source_file: PathBuf,
    /// Line number where the Verification section starts (1-indexed).
    pub section_line: usize,
    /// Individual verification items (commands to run).
    pub items: Vec<VerificationItem>,
}

/// A single verification item representing a command to execute.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationItem {
    /// The shell command to run.
    pub command: String,
    /// Optional working directory for the command.
    pub working_dir: Option<PathBuf>,
    /// Expected exit code (default: 0).
    pub expected_exit_code: Option<i32>,
    /// Expected output matcher.
    pub expected_output: Option<OutputMatcher>,
    /// Timeout in seconds (default: 30).
    pub timeout_secs: Option<u32>,
}

impl Default for VerificationItem {
    fn default() -> Self {
        Self {
            command: String::new(),
            working_dir: None,
            expected_exit_code: Some(0),
            expected_output: None,
            timeout_secs: Some(30),
        }
    }
}

/// Matcher for verifying command output.
#[derive(Debug, Clone, PartialEq)]
pub enum OutputMatcher {
    /// Stdout must contain the given substring.
    Contains(String),
    /// Stdout must match the given regex pattern.
    Regex(String),
    /// Only check exit code, ignore output.
    ExitCodeOnly,
}

/// Extract a verification specification from a parsed document.
///
/// Returns `None` if the document has no Verification section or
/// if the Verification section contains no executable code blocks.
pub fn extract_verification_spec(doc: &ParsedDoc) -> Option<VerificationSpec> {
    let section = doc.get_section("Verification")?;

    if section.code_blocks.is_empty() {
        return None;
    }

    let mut items = Vec::new();

    for block in &section.code_blocks {
        // Only extract from executable code blocks (uses parser's is_executable detection)
        if block.is_executable {
            let commands = extract_commands_from_block(&block.content);
            for command in commands {
                items.push(VerificationItem {
                    command,
                    ..Default::default()
                });
            }
        }
    }

    if items.is_empty() {
        return None;
    }

    Some(VerificationSpec {
        source_file: doc.path.clone(),
        section_line: section.start_line,
        items,
    })
}

/// Extract individual commands from a code block's content.
///
/// Handles:
/// - Lines starting with `$ ` (shell prompt syntax)
/// - Plain commands (each non-empty line is a command)
/// - Multi-line commands with backslash continuations
fn extract_commands_from_block(content: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut current_command = String::new();
    let mut in_continuation = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comment-only lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            if !in_continuation && !current_command.is_empty() {
                commands.push(current_command.trim().to_string());
                current_command.clear();
            }
            continue;
        }

        // Handle shell prompt syntax ($ command)
        let command_part = if let Some(cmd) = trimmed.strip_prefix("$ ") {
            cmd
        } else {
            trimmed
        };

        // Handle line continuations (backslash at end)
        if let Some(without_backslash) = command_part.strip_suffix('\\') {
            if in_continuation {
                current_command.push_str(without_backslash);
            } else {
                current_command = without_backslash.to_string();
            }
            current_command.push(' ');
            in_continuation = true;
        } else if in_continuation {
            current_command.push_str(command_part);
            commands.push(current_command.trim().to_string());
            current_command.clear();
            in_continuation = false;
        } else {
            commands.push(command_part.to_string());
        }
    }

    // Handle any remaining command
    if !current_command.is_empty() {
        commands.push(current_command.trim().to_string());
    }

    commands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_simple_command_from_verification_section() {
        let content = r#"# My Component

## Purpose
A test component.

## Verification
Run the tests:
```bash
cargo test
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.source_file, PathBuf::from("test.md"));
        assert_eq!(spec.items.len(), 1);
        assert_eq!(spec.items[0].command, "cargo test");
        assert_eq!(spec.items[0].expected_exit_code, Some(0));
        assert_eq!(spec.items[0].timeout_secs, Some(30));
    }

    #[test]
    fn handle_multiple_commands() {
        let content = r#"# Test

## Verification
```bash
cargo build
cargo test
cargo clippy
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 3);
        assert_eq!(spec.items[0].command, "cargo build");
        assert_eq!(spec.items[1].command, "cargo test");
        assert_eq!(spec.items[2].command, "cargo clippy");
    }

    #[test]
    fn default_expected_exit_code_is_zero() {
        let content = r#"# Test

## Verification
```bash
echo "hello"
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items[0].expected_exit_code, Some(0));
    }

    #[test]
    fn document_without_verification_section_returns_none() {
        let content = r#"# Test

## Purpose
Just a purpose section.

## Interface
API description.
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc);

        assert!(spec.is_none());
    }

    #[test]
    fn empty_verification_section_returns_none() {
        let content = r#"# Test

## Verification
This section has no code blocks.
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc);

        assert!(spec.is_none());
    }

    #[test]
    fn handle_shell_prompt_syntax() {
        let content = r#"# Test

## Verification
```bash
$ cargo test
$ cargo build --release
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 2);
        assert_eq!(spec.items[0].command, "cargo test");
        assert_eq!(spec.items[1].command, "cargo build --release");
    }

    #[test]
    fn handle_multiple_code_blocks() {
        let content = r#"# Test

## Verification
First set of tests:
```bash
cargo test
```
Second set:
```sh
make lint
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 2);
        assert_eq!(spec.items[0].command, "cargo test");
        assert_eq!(spec.items[1].command, "make lint");
    }

    #[test]
    fn skip_non_executable_code_blocks() {
        let content = r#"# Test

## Verification
Example output:
```json
{"status": "ok"}
```
Run this:
```bash
curl localhost:8080
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 1);
        assert_eq!(spec.items[0].command, "curl localhost:8080");
    }

    #[test]
    fn handle_line_continuations() {
        let content = r#"# Test

## Verification
```bash
cargo build \
  --release \
  --features all
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 1);
        assert_eq!(
            spec.items[0].command,
            "cargo build  --release  --features all"
        );
    }

    #[test]
    fn skip_comment_lines() {
        let content = r#"# Test

## Verification
```bash
# This is a comment
cargo test
# Another comment
cargo build
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 2);
        assert_eq!(spec.items[0].command, "cargo test");
        assert_eq!(spec.items[1].command, "cargo build");
    }

    #[test]
    fn section_line_is_correct() {
        let content = r#"# Title

## Purpose
Some content.

## Verification
```bash
cargo test
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        // Line 1: # Title
        // Line 2: blank
        // Line 3: ## Purpose
        // Line 4: Some content.
        // Line 5: blank
        // Line 6: ## Verification
        assert_eq!(spec.section_line, 6);
    }

    #[test]
    fn handle_code_block_without_language_but_with_prompt() {
        // Code blocks without language are only executable if they contain $ or > prompts
        let content = r#"# Test

## Verification
```
$ cargo test
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 1);
        assert_eq!(spec.items[0].command, "cargo test");
    }

    #[test]
    fn code_block_without_language_or_prompt_is_not_executable() {
        // Code blocks without language and without prompts are not treated as executable
        let content = r#"# Test

## Verification
```
cargo test
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc);

        // Returns None because the plain code block is not detected as executable
        assert!(spec.is_none());
    }

    #[test]
    fn verification_item_default_values() {
        let item = VerificationItem::default();

        assert!(item.command.is_empty());
        assert!(item.working_dir.is_none());
        assert_eq!(item.expected_exit_code, Some(0));
        assert!(item.expected_output.is_none());
        assert_eq!(item.timeout_secs, Some(30));
    }

    #[test]
    fn handle_empty_lines_in_code_block() {
        let content = r#"# Test

## Verification
```bash
cargo build

cargo test
```
"#;

        let doc = ParsedDoc::parse_content(PathBuf::from("test.md"), content).unwrap();
        let spec = extract_verification_spec(&doc).unwrap();

        assert_eq!(spec.items.len(), 2);
        assert_eq!(spec.items[0].command, "cargo build");
        assert_eq!(spec.items[1].command, "cargo test");
    }

    #[test]
    fn verification_spec_clone_and_eq() {
        let spec = VerificationSpec {
            source_file: PathBuf::from("test.md"),
            section_line: 10,
            items: vec![VerificationItem {
                command: "cargo test".to_string(),
                ..Default::default()
            }],
        };

        let cloned = spec.clone();
        assert_eq!(spec, cloned);
    }

    #[test]
    fn output_matcher_variants() {
        let contains = OutputMatcher::Contains("success".to_string());
        let regex = OutputMatcher::Regex(r"\d+ tests passed".to_string());
        let exit_only = OutputMatcher::ExitCodeOnly;

        // Test clone and eq
        assert_eq!(contains.clone(), contains);
        assert_eq!(regex.clone(), regex);
        assert_eq!(exit_only.clone(), exit_only);

        // Test they're different
        assert_ne!(contains, regex);
        assert_ne!(regex, exit_only);
    }
}
