//! A small `.env` parser: `KEY=value`, `KEY="value"`, `KEY='value'`, `# comments`, `export KEY=…`.
//! Values are taken literally (no `$VAR` expansion), like Node's `util.parseEnv` used by the web
//! build, so the services and the web app read exactly the same values.

/// Parses `.env` text into `(key, value)` pairs in file order.
pub fn parse(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        out.push((key.trim().to_owned(), parse_value(value.trim())));
    }
    out
}

fn parse_value(value: &str) -> String {
    if let Some(inner) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) {
        inner.replace("\\n", "\n").replace("\\\"", "\"")
    } else if let Some(inner) = value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')) {
        inner.to_owned()
    } else {
        // Unquoted: strip an inline comment.
        value.split(" #").next().unwrap_or("").trim().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quotes_comments_and_exports() {
        let parsed = parse(
            "# c\nA=1\nB=\"two \\\"words\\\"\"\nexport C='$X'\nD=val # comment\n\nE=\nF='Кафе \"Ж\"'\n",
        );
        assert_eq!(
            parsed,
            vec![
                ("A".into(), "1".into()),
                ("B".into(), "two \"words\"".into()),
                ("C".into(), "$X".into()),
                ("D".into(), "val".into()),
                ("E".into(), String::new()),
                ("F".into(), "Кафе \"Ж\"".into()),
            ]
        );
    }
}
