use proc_macro::TokenStream;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const FILESTEM: &str = "temphdr";
const C_ARGS: &[&str] = &[
    "--use-core",
    "--no-layout-tests",
    "--no-doc-comments",
    "--no-prepend-enum-name",
    "--disable-header-comment",
    "--",
    "-std=c17",
    "-I.",
];
const CPP_ARGS: &[&str] = &[
    "--use-core",
    "--generate-inline-functions",
    "--no-layout-tests",
    "--no-doc-comments",
    "--no-prepend-enum-name",
    "--disable-header-comment",
    "--",
    "-xc++",
    "-std=c++17",
    "-I.",
];

fn gen_header(input: String, is_cpp: bool) -> PathBuf {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    input.hash(&mut hasher);
    let input_hash = hasher.finish();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time!");
    let filename = format!(
        "{:?}{}{}.{}",
        now.as_millis(),
        input_hash,
        FILESTEM,
        if is_cpp { "hpp" } else { "h" }
    );
    let f = std::env::temp_dir().join(filename);
    let input = "#pragma once\n".to_string() + &input;
    std::fs::write(&f, &input).expect("Failed to generate temporary header!");
    f
}

fn del_header(header: Option<PathBuf>) {
    if let Some(f) = header {
        if f.exists() {
            std::fs::remove_file(f).expect("Failed to delete temporary header!");
        }
    }
}

fn gen_command(header: String, args: &[&str], is_cpp: bool) -> (Command, Option<PathBuf>) {
    let mut cmd = Command::new("bindgen");
    let (path, header) = {
        let header = gen_header(header, is_cpp);
        let path = format!("{}", header.display());
        (path, Some(header))
    };
    let mut args = args.to_vec();
    args.insert(0, &path);
    cmd.args(args);
    (cmd, header)
}

fn run_cmd(cmd: &str) -> Vec<String> {
    let v: Vec<&str> = cmd.split_whitespace().collect();
    let mut cmd = Command::new(v[0]);
    cmd.args(&v[1..]);
    let cmd = cmd.output().expect("Failed to invoke command!");
    String::from_utf8(cmd.stdout)
        .expect("Failed to parse output")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn common(input: TokenStream, is_cpp: bool) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split(',').collect();
    let mut headers = vec![];
    let mut extra_args = vec![];
    for elem in input {
        let elem = elem.trim();
        if elem.starts_with("\"-") {
            extra_args.push(elem.to_string());
        } else if elem.starts_with("\"<") {
            headers.push(&elem[1..elem.len() - 1]);
        } else if elem.starts_with("\"$") {
            let mut temp = run_cmd(&elem[2..elem.len() - 1]);
            extra_args.append(&mut temp);
        } else {
            headers.push(elem);
        }
    }
    let extra_args: Vec<String> = extra_args.iter().map(|s| s.replace('"', "")).collect();
    let mut args: Vec<&str> = if is_cpp {
        CPP_ARGS.to_vec()
    } else {
        C_ARGS.to_vec()
    };
    let header = headers
        .iter()
        .map(|s| format!("#include {}\n", s))
        .collect();
    args.append(&mut extra_args.iter().map(|s| s.trim()).collect());
    let (mut cmd, header) = gen_command(header, &args, is_cpp);
    let cmd = cmd.output().expect("Failed to invoke bindgen!");
    del_header(header);
    if !cmd.status.success() {
        std::io::stderr().write_all(&cmd.stderr).unwrap();
    }
    String::from_utf8(cmd.stdout)
        .expect("Failed to parse bindgen output")
        .parse()
        .unwrap()
}
