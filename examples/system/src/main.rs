use c_import::c_import;

c_import!("<cairo/cairo.h>", "<stdio.h>");

fn main() {
    let version = unsafe { cairo_version() };
    println!("{}", version);
}
