c_import::cpp_import!(
    "<FL/Fl.H>",
    "--link fltk"
);

fn main() {
    println!("{}", unsafe { Fl::abi_version() });
}
