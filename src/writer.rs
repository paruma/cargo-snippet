use serde_derive::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
struct VScode {
    prefix: String,
    body: Vec<String>,
}

#[cfg(feature = "inner_rustfmt")]
pub fn format_src(src: &str) -> Option<String> {
    let mut rustfmt_config = rustfmt_nightly::Config::default();
    rustfmt_config
        .set()
        .emit_mode(rustfmt_nightly::EmitMode::Stdout);
    rustfmt_config
        .set()
        .verbose(rustfmt_nightly::Verbosity::Quiet);

    let mut out = Vec::with_capacity(src.len() * 2);
    let input = rustfmt_nightly::Input::Text(src.into());

    if rustfmt_nightly::Session::new(rustfmt_config, Some(&mut out))
        .format(input)
        .is_ok()
    {
        String::from_utf8(out).ok().map(|s| s.replace("\r\n", "\n"))
    } else {
        None
    }
}

#[cfg(not(feature = "inner_rustfmt"))]
pub fn format_src(src: &str) -> Option<String> {
    use std::io::Write;
    use std::process;

    let mut command = process::Command::new("rustfmt")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process");
    {
        let mut stdin = command.stdin.take()?;
        write!(stdin, "{}", src).unwrap();
    }
    let out = command.wait_with_output().ok()?;

    if !out.status.success() {
        log::error!("rustfmt returns non-zero status");
        log::error!("[stdout]\n{}", String::from_utf8_lossy(&out.stdout));
        log::error!("[stderr]\n{}", String::from_utf8_lossy(&out.stderr));
        return None;
    }

    let stdout = out.stdout;
    let out = String::from_utf8(stdout).ok()?;
    Some(out.replace("\r\n", "\n"))
}

pub fn write_neosnippet(snippets: &BTreeMap<String, String>) {
    for (name, content) in snippets.iter() {
        if let Some(formatted) = format_src(content) {
            println!("snippet {}", name);
            for line in formatted.lines() {
                println!("    {}", line);
            }
            println!();
        }
    }
}

pub fn write_vscode(snippets: &BTreeMap<String, String>) {
    let vscode: BTreeMap<String, VScode> = snippets
        .iter()
        .filter_map(|(name, content)| {
            format_src(content).map(|formatted| {
                (
                    name.to_owned(),
                    VScode {
                        prefix: name.to_owned(),
                        body: vscode_body(&formatted),
                    },
                )
            })
        })
        .collect();

    if let Ok(json) = serde_json::to_string_pretty(&vscode) {
        println!("{}", json);
    }
}

fn vscode_body(formatted: &str) -> Vec<String> {
    formatted
        .lines()
        // Escape snippet syntax before serde applies the separate JSON escaping.
        // Backslashes must come first so the escapes added for dollars stay intact.
        .map(|line| line.replace('\\', "\\\\").replace('$', "\\$"))
        .collect()
}

pub fn write_ultisnips(snippets: &BTreeMap<String, String>) {
    for (name, content) in snippets.iter() {
        if let Some(formatted) = format_src(content) {
            println!("snippet {}", name);
            print!("{}", formatted);
            println!("endsnippet");
            println!();
        }
    }
}

#[test]
fn test_format_src() {
    assert_eq!(format_src("fn foo(){}"), Some("fn foo() {}\n".into()));

    assert_eq!(
        format_src("/// doc comment\n pub fn foo(){}"),
        Some("/// doc comment\npub fn foo() {}\n".into())
    );
}

#[test]
fn test_vscode_json_expansion_preserves_literal_text() {
    // Model only the literal-text escape rule of VS Code's snippet grammar.
    // Assert every dollar is escaped so tabstops/variables cannot be introduced.
    fn expand_literal(body: &str) -> String {
        let mut expanded = String::new();
        let mut chars = body.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\\' && matches!(chars.peek(), Some('\\' | '$' | '}')) {
                expanded.push(chars.next().unwrap());
            } else {
                assert_ne!(ch, '$', "unescaped snippet syntax: {}", body);
                expanded.push(ch);
            }
        }
        expanded
    }

    let source = include_str!("../tests/fixtures/vscode_escapes.rs");
    let formatted = format_src(source).unwrap();
    let snippet = VScode {
        prefix: "escapes".into(),
        body: vscode_body(&formatted),
    };
    let json = serde_json::to_string(&snippet).unwrap();
    let decoded: serde_json::Value = serde_json::from_str(&json).unwrap();
    let body = decoded["body"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(expand_literal(&body), formatted.trim_end_matches('\n'));

    // Exercise slash parity before every kind of snippet construct, including
    // a trailing slash and a literal closing brace.
    for count in 0..=8 {
        for suffix in ["", "$", "$0", "${1:default}", "$TM_FILENAME", "}", "日本語"] {
            let source = format!("{}{}", "\\".repeat(count), suffix);
            assert_eq!(expand_literal(&vscode_body(&source).join("\n")), source);
        }
    }
}
