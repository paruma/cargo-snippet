#[snippet]
fn escapes() {
    let _ = ["hello"].map(|s| s.replace("\\'", "'").replace('"', "\\\""));
    let _ = "\\\"\n\t\r\0\x41\u{3042}";
    let _ = '\\';
    let _ = r#"raw: \ \\ \" \' \} $0 ${1:default} ${2|a,b|} $TM_FILENAME"#;
    let _ = r#"\$ \\$ \\\$ \${1:default} \\${TM_FILENAME} } 日本語"#;
    macro_rules! identity {
        ($value:expr) => {
            $value
        };
    }
    let _ = identity!("$crate $value $(repeat)*");
}
