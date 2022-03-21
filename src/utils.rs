use proc_macro::TokenStream;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const FILESTEM: &str = "temphdr";
const C_ARGS: &[&str] = &[
    "--no-layout-tests",
    "--no-doc-comments",
    "--no-prepend-enum-name",
    "--disable-header-comment",
    "--",
    "-std=c17",
];
const CPP_ARGS: &[&str] = &[
    "--generate-inline-functions",
    "--no-layout-tests",
    "--no-doc-comments",
    "--no-prepend-enum-name",
    "--disable-header-comment",
    "--",
    "-xc++",
    "-std=c++17",
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
    let input = "#pragma once\n#include ".to_string() + &input;
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

fn gen_command(header: String, args: &mut Vec<&str>, is_cpp: bool) -> (Command, Option<PathBuf>) {
    let mut cmd = Command::new("bindgen");
    let (path, header) = if header.starts_with('<') && header.ends_with('>') {
        let header = gen_header(header.split_whitespace().collect(), is_cpp);
        let path = format!("{}", header.display());
        (path, Some(header))
    } else {
        let cwd = std::env::current_dir().expect("Couldn't get current working dir!");
        let path = cwd.join(&header.replace('"', ""));
        let path = format!("{}", path.display());
        (path, None)
    };
    let mut args = args.to_vec();
    args.insert(0, &path);
    cmd.args(args);
    (cmd, header)
}

pub(crate) fn common(input: TokenStream, is_cpp: bool) -> TokenStream {
    let input = input.to_string();
    let input: Vec<&str> = input.split(',').collect();
    let extra_args: Vec<String> = input[1..]
        .to_vec()
        .iter()
        .map(|s| s.replace('"', ""))
        .collect();
    let header = input[0].to_string();
    let mut args: Vec<&str> = if is_cpp {
        CPP_ARGS.to_vec()
    } else {
        C_ARGS.to_vec()
    };
    args.append(&mut extra_args.iter().map(|s| s.trim()).collect());
    let (mut cmd, header) = gen_command(header, &mut args, is_cpp);
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
