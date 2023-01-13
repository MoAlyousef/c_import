mod cairo {
    use c_import::c_import;
    c_import!("header.h", "-Isrc");
}

fn main() {
    let version = unsafe { cairo::cairo_version() };
    println!("{}", version);
}
