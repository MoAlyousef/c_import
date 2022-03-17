/*!
# c_import

This is a small crate providing a c_import macro (also a cpp_import macro), which can be used to import C headers into your program. You need [bindgen](https://github.com/rust-lang/rust-bindgen) to be installed in your system.

## Usage
In your Cargo.toml:
```toml
# Cargo.toml

[depenedencies]
c_import = "0.1"
```

In your Rust source file:
```rust,ignore
// src/main.rs
use c_import::c_import;

c_import!(<cairo/cairo.h>);

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

In your Rust build script:
```rust,ignore
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=cairo");
}
```

Using non-system headers is also possible via enclosing the header path with quotation marks:
```rust,ignore
// src/main.rs
use c_import::c_import;
c_import!("src/my_header.h");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Usage with C++ headers (limited)

```rust,ignore
// src/main.rs
use c_import::cpp_import;

cpp_import!(<FL/Fl.H>);

fn main() {
    let version = unsafe { Fl::api_version() };
    println!("{}", version);
}
```

```rust,ignore
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=fltk");
}
```

Another example showing how to deal with C++ namespaces:

```cpp
// src/my_header.hpp
#pragma once

namespace my_namespace {
class MyStruct {
    int version_;
  public:
    MyStruct(int version);
    int version() const;
};
}
```

```rust,ignore
// src/main.rs
use c_import::cpp_import;

cpp_import!("src/my_header.hpp");

fn main() {
    let h = unsafe { my_namespace_MyStruct::new(2) };
    println!("{}", unsafe { h.version() });
}
```


## Limitations
- Mostly bindgen limitations with C++ headers.
*/

use proc_macro::TokenStream;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const FILENAME: &str = "temp_c_import";
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

fn gen_header(input: String, is_cpp: bool) -> PathBuf {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let input_hash = hasher.finish();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time!");
    let filename = format!(
        "{}_{:?}_{}.{}",
        input_hash,
        now.as_secs(),
        FILENAME,
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

fn common(input: TokenStream, is_cpp: bool) -> String {
    let mut cmd = std::process::Command::new("bindgen");
    let mut args: Vec<&str> = if is_cpp {
        CPP_ARGS.to_vec()
    } else {
        C_ARGS.to_vec()
    };
    let input = input.to_string();
    let header = if input.starts_with('<') && input.ends_with('>') {
        let header = gen_header(input.split_whitespace().collect(), is_cpp);
        let path = format!("{}", header.display());
        args.insert(0, &path);
        cmd.args(&args);
        Some(header)
    } else {
        let cwd = std::env::current_dir().expect("Couldn't get current working dir!");
        let path = cwd.join(&input.replace('"', ""));
        assert!(path.exists(), "{} doesn't exist!", input);
        let path = format!("{}", path.display());
        args.insert(0, &path);
        cmd.args(&args);
        None
    };
    let ret = String::from_utf8(cmd.output().expect("Failed to invoke bindgen!").stdout)
        .expect("Failed to parse bindgen output")
        .parse()
        .unwrap();
    del_header(header);
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
