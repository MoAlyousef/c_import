# c_import

This is a small proc macro crate providing a c_import macro (also a cpp_import macro), which can be used to import C headers into your program. It leverages [bindgen](https://github.com/rust-lang/rust-bindgen), so bindgen needs to be installed in your system.
It also works in no_std mode.

## Usage
In your Cargo.toml:
```toml
# Cargo.toml

[depenedencies]
c_import = "0.2"
```

In your Rust source file:
```rust
// src/main.rs
use c_import::c_import;

c_import!(
    "<stdio.h>", 
    "<cairo/cairo.h>",
    "--link cairo"
);

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

If you don't use the `--link` directive, you can use a Rust build script:
```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=cairo");
}
```

Using non-system headers is also possible via enclosing the header path with quotation marks:
```rust
// src/main.rs
use c_import::c_import;
c_import!("src/my_header.h");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Extra clang arguments
You can pass extra clang arguments as extra arguments to the macro:
```rust
// src/main.rs
use c_import::c_import;
c_import!(
    "src/my_header.h", 
    "-DMY_DEFINE", 
    "-I/somepath/include"
);

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

Similarly you can invoke tools like pkg-config to retrieve cflags and pass them to bindgen:
```rust
use c_import::c_import;

c_import!(
    "<cairo.h>", 
    "<stdio.h>",
    "$pkg-config --cflags cairo"
);

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Macro parameters
- Normal arguments are considered header files.
- Arguments starting with `--link` are used to insert `#[link (name = libname)]` attributes to the generated extern functions, this allows linking the libraries without having to explicitly create a build.rs file containing `println!("cargo:rustc-link-lib=libname");`
- Arguments starting with `--` are considered bindgen arguments.
- Arguments starting with `-` are considered cflags, such as include paths or defines ("-I" & "-D" respectively).
- Arguments starting with `$` are considered shell commands which return cflags, similar to pkg-config.

## Usage with C++ headers (limited)

```rust
// src/main.rs
use c_import::cpp_import;

cpp_import!("<FL/Fl.H>");

fn main() {
    let version = unsafe { Fl::api_version() }; // static method of class Fl
    println!("{}", version);
}
```

```rust
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

```rust
// src/main.rs
use c_import::cpp_import;

cpp_import!("src/my_header.hpp");

fn main() {
    let h = unsafe { my_namespace_MyStruct::new(2) };
    println!("{}", unsafe { h.version() });
}
```


## Limitations
- Mostly bindgen limitations: 
  - with C++ headers.
  - with static inline functions.