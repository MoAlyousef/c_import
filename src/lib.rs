extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;
use std::fs;
use std::process::Command;

const FILE: &str = "temp_c_import_includer.h";
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
    // "--enable-cxx-namespaces",
    "--no-layout-tests",
    "--no-doc-comments",
    "--no-prepend-enum-name",
    "--disable-header-comment",
    "--",
    "-xc++",
    "-std=c++17",
];

fn gen_header(input: String) {
    let f = env::temp_dir().join(FILE);
    let input = "#pragma once\n#include ".to_string() + &input;
    fs::write(f, &input).expect("Failed to generate temporary header!");
}

fn del_header() {
    let f = env::temp_dir().join(FILE);
    if f.exists() {
        fs::remove_file(f).expect("Failed to delete temporary header!");
    }
}

fn common(input: TokenStream, is_cpp: bool) -> String {
    let mut cmd = Command::new("bindgen");
    let mut args: Vec<&str> = if is_cpp {
        CPP_ARGS.to_vec()
    } else {
        C_ARGS.to_vec()
    };
    let input = input.to_string();
    if input.starts_with('<') && input.ends_with('>') {
        let f = env::temp_dir().join(FILE);
        gen_header(input.split_whitespace().collect());
        let path = format!("{}", f.display());
        args.insert(0, &path);
        cmd.args(&args);
    } else {
        let cwd = env::current_dir().expect("Couldn't get current working dir!");
        let path = cwd.join(&input.replace('"', ""));
        assert!(path.exists(), "{} doesn't exist!", input);
        let path = format!("{}", path.display());
        args.insert(0, &path);
        cmd.args(&args);
    }
    let ret = String::from_utf8(cmd.output().expect("Failed to invoke bindgen!").stdout)
        .unwrap()
        .parse()
        .unwrap();
    del_header();
    ret
}

#[proc_macro]
pub fn c_import(input: TokenStream) -> TokenStream {
    common(input, false).parse().unwrap()
}

// bindgen path --generate-inline-functions --no-layout-tests --no-doc-comments --no-prepend-enum-name --disable-header-comment -- -xc++ -std=c++17
#[proc_macro]
pub fn cpp_import(input: TokenStream) -> TokenStream {
    common(input, true).parse().unwrap()
}
