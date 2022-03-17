# c_import

This is a small crate providing a c_import macro (also a cpp_import macro), which can be used to import C headers into your program. You need [bindgen](https://github.com/rust-lang/rust-bindgen) to be installed in your system.

## Usage
```toml
# Cargo.toml

[depenedencies]
c_import = "0.1"
```

```rust
// src/main.rs
use c_import::c_import;

c_import!(<cairo/cairo.h>);

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=cairo");
}
```

Using non-system headers is also possible via enclosing the header path with quotation marks:
```rust
use c_import::c_import;
c_import!("src/my_header.h");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
```

## Using with C++
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

```rust
// build.rs
fn main() {
    // assuming there's a libmy_cpp_lib.a
    println!("cargo:rustc-link-lib=my_cpp_lib");
}
```

Another example:
```cpp
// src/fltk_wrapper.h
#pragma once
#include <FL/Fl.H>
```

```rust
// src/main.rs
use c_import::cpp_import;

cpp_import!("src/fltk_wrapper.hpp");

fn main() {
    let version = unsafe { Fl::api_version() };
    println!("{}", version);
}
```

```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=fltk");
}
```

## Limitations
- Mostly bindgen limitations with C++ headers.