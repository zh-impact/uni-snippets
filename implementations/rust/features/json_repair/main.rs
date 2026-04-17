use std::env;
use std::fs;
use std::io::{self, Read};

// ── Error types (standalone) ──────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum ErrorKind {
    ParseError,
}

#[derive(Debug)]
struct ParseError {
    kind: ErrorKind,
    message: String,
    details: Option<String>,
}

impl ParseError {
    fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            details: None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

// ── JSON repair functions ─────────────────────────────────────────────

fn extract_json(text: &str) -> Result<String, ParseError> {
    let trimmed = text.trim();
    if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            return Ok(trimmed[start..=end].to_string());
        }
    }
    Err(ParseError::new(
        ErrorKind::ParseError,
        "No valid JSON found in input.",
    ))
}

fn remove_trailing_commas(json: &str) -> String {
    let chars: Vec<char> = json.chars().collect();
    let mut out = String::with_capacity(json.len());
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ',' {
            let mut j = i + 1;
            while j < chars.len()
                && (chars[j] == ' ' || chars[j] == '\n' || chars[j] == '\r' || chars[j] == '\t')
            {
                j += 1;
            }
            if j < chars.len() && (chars[j] == '}' || chars[j] == ']') {
                i += 1;
                continue;
            }
        }
        if chars[i] == '"' {
            out.push(chars[i]);
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    out.push(chars[i]);
                    i += 1;
                }
                out.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                out.push(chars[i]);
                i += 1;
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn count_unbalanced_brackets(json: &str) -> (i32, i32) {
    let chars: Vec<char> = json.chars().collect();
    let mut i = 0;
    let mut curly = 0i32;
    let mut bracket = 0i32;
    while i < chars.len() {
        match chars[i] {
            '"' => {
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 2;
                        continue;
                    }
                    i += 1;
                }
                i += 1;
            }
            '{' => {
                curly += 1;
                i += 1;
            }
            '}' => {
                curly -= 1;
                i += 1;
            }
            '[' => {
                bracket += 1;
                i += 1;
            }
            ']' => {
                bracket -= 1;
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    (curly, bracket)
}

fn escape_control_chars_in_strings(json: &str) -> String {
    let chars: Vec<char> = json.chars().collect();
    let mut out = String::with_capacity(chars.len() + chars.len() / 10);
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '"' {
            out.push('"');
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    out.push(chars[i]);
                    out.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                match chars[i] {
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if c.is_ascii_control() => {
                        out.push_str(&format!("\\u{:04x}", c as u32));
                    }
                    c => out.push(c),
                }
                i += 1;
            }
            if i < chars.len() {
                out.push('"');
                i += 1;
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

fn maybe_insert_comma_after(chars: &[char], n: usize, out: &mut String, pos: usize) -> usize {
    let mut j = pos;
    while j < n && chars[j].is_whitespace() {
        j += 1;
    }
    if j >= n || chars[j] != '"' {
        return pos;
    }
    let mut k = j + 1;
    while k < n && chars[k] != '"' {
        if chars[k] == '\\' && k + 1 < n {
            k += 1;
        }
        k += 1;
    }
    if k >= n {
        return pos;
    }
    let mut m = k + 1;
    while m < n && chars[m].is_whitespace() {
        m += 1;
    }
    if m < n && chars[m] == ':' {
        out.push(',');
    }
    pos
}

fn insert_missing_field_commas(json: &str) -> String {
    let chars: Vec<char> = json.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n + n / 20);
    let mut i = 0;

    while i < n {
        match chars[i] {
            '"' => {
                out.push('"');
                i += 1;
                while i < n && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < n {
                        out.push(chars[i]);
                        out.push(chars[i + 1]);
                        i += 2;
                        continue;
                    }
                    out.push(chars[i]);
                    i += 1;
                }
                if i < n {
                    out.push('"');
                    i += 1;
                }
                i = maybe_insert_comma_after(&chars, n, &mut out, i);
            }
            '}' | ']' => {
                out.push(chars[i]);
                i += 1;
                i = maybe_insert_comma_after(&chars, n, &mut out, i);
            }
            _ => {
                out.push(chars[i]);
                i += 1;
            }
        }
    }
    out
}

fn insert_missing_values(json: &str) -> String {
    let chars: Vec<char> = json.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n + n / 50);
    let mut i = 0;

    while i < n {
        if chars[i] == ':' {
            out.push(':');
            i += 1;
            while i < n && chars[i].is_whitespace() {
                i += 1;
            }
            if i >= n || chars[i] == ',' || chars[i] == '}' || chars[i] == ']' {
                out.push_str("null");
            }
            continue;
        }
        if chars[i] == '"' {
            out.push('"');
            i += 1;
            while i < n && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < n {
                    out.push(chars[i]);
                    out.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                out.push(chars[i]);
                i += 1;
            }
            if i < n {
                out.push('"');
                i += 1;
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn quote_unquoted_values(json: &str) -> String {
    let chars: Vec<char> = json.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n + n / 20);
    let mut i = 0;

    while i < n {
        if chars[i] == '"' {
            out.push('"');
            i += 1;
            while i < n && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < n {
                    out.push(chars[i]);
                    out.push(chars[i + 1]);
                    i += 2;
                    continue;
                }
                out.push(chars[i]);
                i += 1;
            }
            if i < n {
                out.push('"');
                i += 1;
            }
            continue;
        }

        if chars[i] == ':' {
            out.push(':');
            i += 1;
            while i < n && chars[i].is_whitespace() {
                out.push(chars[i]);
                i += 1;
            }
            if i >= n {
                break;
            }
            let ch = chars[i];
            if ch == '"'
                || ch == '{'
                || ch == '['
                || ch.is_ascii_digit()
                || ch == '-'
                || (ch == 't' && json[i..].starts_with("true"))
                || (ch == 'f' && json[i..].starts_with("false"))
                || (ch == 'n' && json[i..].starts_with("null"))
            {
                continue;
            }
            out.push('"');
            while i < n && chars[i] != ',' && chars[i] != '}' && chars[i] != ']' {
                out.push(chars[i]);
                i += 1;
            }
            out.push('"');
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }
    out
}

fn try_repair_json(text: &str) -> Option<String> {
    let mut json = text.to_string();

    // 1. Escape control characters inside string literals
    json = escape_control_chars_in_strings(&json);
    if serde_json::from_str::<serde_json::Value>(&json).is_ok() {
        return Some(json);
    }

    // 2. Remove stray control characters outside strings
    json = json
        .chars()
        .filter(|c| c.is_ascii_whitespace() || !c.is_ascii_control())
        .collect();

    // 3. Strip markdown code fences
    if json.contains("```") {
        json = json
            .lines()
            .filter(|l| !l.trim().starts_with("```"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
    }

    // 4. Remove trailing commas
    json = remove_trailing_commas(&json);
    if serde_json::from_str::<serde_json::Value>(&json).is_ok() {
        return Some(json);
    }

    // 5. Insert missing commas between fields
    json = insert_missing_field_commas(&json);
    if serde_json::from_str::<serde_json::Value>(&json).is_ok() {
        return Some(json);
    }

    // 6. Insert null for missing values ("key":, → "key":null)
    json = insert_missing_values(&json);
    if serde_json::from_str::<serde_json::Value>(&json).is_ok() {
        return Some(json);
    }

    // 7. Quote unquoted string values ("title":Foo → "title":"Foo")
    json = quote_unquoted_values(&json);
    if serde_json::from_str::<serde_json::Value>(&json).is_ok() {
        return Some(json);
    }

    // 8. Add missing closing brackets
    let (curly, bracket) = count_unbalanced_brackets(&json);
    if curly > 0 || bracket > 0 {
        for _ in 0..bracket {
            json.push(']');
        }
        for _ in 0..curly {
            json.push('}');
        }
        if serde_json::from_str::<serde_json::Value>(&json).is_ok() {
            return Some(json);
        }
    }

    None
}

fn format_parse_error_details(json_text: &str, err: &serde_json::Error) -> String {
    let err_msg = err.to_string();
    let mut details = String::new();

    if let Some(line_pos) = err_msg.find("line ") {
        let after = &err_msg[line_pos + 5..];
        if let Some(col_pos) = after.find(" column ") {
            let line_str = &after[..col_pos];
            if let Ok(line_num) = line_str.parse::<usize>() {
                let line_idx = line_num.max(1) - 1;
                let lines: Vec<&str> = json_text.lines().collect();
                let start = line_idx.saturating_sub(2);
                let end = (line_idx + 3).min(lines.len());

                details.push_str(&format!("JSON error at line {line_num}:\n"));
                for (i, line) in lines[start..end].iter().enumerate() {
                    let num = start + i + 1;
                    let marker = if start + i == line_idx { ">>>" } else { "   " };
                    details.push_str(&format!("  {marker} {num:>4} | {line}\n"));
                }
                details.push_str(&format!("\nRaw error: {err_msg}"));
                return details;
            }
        }
    }

    details.push_str(&format!("Raw error: {err_msg}"));
    details
}

fn parse_json_with_recovery(json_str: &str) -> Result<serde_json::Value, ParseError> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
        return Ok(value);
    }

    if let Some(repaired) = try_repair_json(json_str) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&repaired) {
            return Ok(value);
        }
    }

    let strict_err = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
    let details = format_parse_error_details(json_str, &strict_err);

    Err(ParseError {
        kind: ErrorKind::ParseError,
        message: "JSON could not be parsed".to_string(),
        details: Some(details),
    })
}

// ── Main ──────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        match fs::read_to_string(&args[1]) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading '{}': {e}", args[1]);
                std::process::exit(1);
            }
        }
    } else {
        let mut buf = String::new();
        if io::stdin().read_to_string(&mut buf).is_err() {
            eprintln!("Error reading from stdin");
            std::process::exit(1);
        }
        buf
    };

    println!("=== Input ({} bytes) ===", input.len());
    println!("{input}");
    println!();

    // Step 1: Extract JSON
    println!("--- Step 1: Extract JSON ---");
    let json_str = match extract_json(&input) {
        Ok(s) => {
            println!("Extracted {} bytes", s.len());
            println!("{s}");
            s
        }
        Err(e) => {
            eprintln!("FAILED: {e}");
            std::process::exit(1);
        }
    };
    println!();

    // Step 2: Try strict parse
    println!("--- Step 2: Strict parse ---");
    match serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(value) => {
            println!("OK (no repair needed)");
            println!("{}", serde_json::to_string_pretty(&value).unwrap());
            return;
        }
        Err(e) => {
            println!("FAILED: {e}");
        }
    }
    println!();

    // Step 3: Try repair
    println!("--- Step 3: Repair ---");
    match try_repair_json(&json_str) {
        Some(repaired) => {
            println!("Repaired ({} bytes):", repaired.len());
            println!("{repaired}");
            println!();

            println!("--- Step 4: Parse repaired ---");
            match serde_json::from_str::<serde_json::Value>(&repaired) {
                Ok(value) => {
                    println!("OK");
                    println!("{}", serde_json::to_string_pretty(&value).unwrap());
                }
                Err(e) => println!("FAILED: {e}"),
            }
        }
        None => {
            println!("Repair could not produce valid JSON.");
        }
    }
    println!();

    // Step 5: Full pipeline
    println!("--- Full pipeline ---");
    match parse_json_with_recovery(&json_str) {
        Ok(value) => {
            println!("OK");
            println!("{}", serde_json::to_string_pretty(&value).unwrap());
        }
        Err(e) => {
            eprintln!("FAILED: {e}");
            if let Some(details) = &e.details {
                eprintln!("\n{details}");
            }
        }
    }
}
