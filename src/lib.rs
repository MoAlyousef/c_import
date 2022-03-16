extern crate proc_macro;

use proc_macro::TokenStream;
use std;
use std::process::Command;
use syn;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
struct FileName {
    filename: String,
}

impl Parse for FileName {
    fn parse(input: ParseStream) -> Result<Self> {
        let f: syn::LitStr = input.parse()?;
        Ok(Self {
            filename: f.value(),
        })
    }
}

#[proc_macro]
pub fn c_import(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as FileName);

    let cwd = std::env::current_dir().unwrap();

    let file_path = cwd.join(&input.filename);
    assert!(file_path.exists(), "{} doesn't exist!", input.filename);
    let file_path_str = format!("{}", file_path.display());
    let out = Command::new("bindgen")
        .args(&[
            &file_path_str,
            "--no-layout-tests",
            "--no-doc-comments",
            "--no-prepend-enum-name",
            "--disable-header-comment",
            "--",
            "-std=c17",
        ])
        .output()
        .unwrap();
    let out = String::from_utf8(out.stdout).unwrap();

    out.parse().unwrap()
}

// bindgen path --enable-cxx-namespaces --generate-inline-functions --no-layout-tests --no-doc-comments --no-prepend-enum-name --disable-header-comment -- -xc++ -std=c++17
#[proc_macro]
pub fn cpp_import(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as FileName);

    let cwd = std::env::current_dir().unwrap();

    let file_path = cwd.join(&input.filename);
    assert!(file_path.exists(), "{} doesn't exist!", input.filename);
    let file_path_str = format!("{}", file_path.display());
    let out = Command::new("bindgen")
        .args(&[
            &file_path_str,
            "--generate-inline-functions",
            "--enable-cxx-namespaces",
            "--no-layout-tests",
            "--no-doc-comments",
            "--no-prepend-enum-name",
            "--disable-header-comment",
            "--",
            "-xc++",
            "-std=c++17",
        ])
        .output()
        .unwrap();
    let out = String::from("use crate::root::*;\n") + &String::from_utf8(out.stdout).unwrap();
    out.parse().unwrap()
}
