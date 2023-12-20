#![no_std]
#![no_main]

use core::ffi::*;
use defer_lite::defer;


c_import::c_import!(
    "<stdio.h>", 
    "<stdlib.h>",
    "<cairo.h>",
    "$pkg-config --cflags cairo",
    "--link cairo", 
    "--link c"
);

#[no_mangle]
pub extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    unsafe {
        let args = core::slice::from_raw_parts(argv, argc as _);
        for (i, arg) in args.iter().enumerate() {
            printf("arg %d is %s\n\0".as_ptr() as _, i, *arg);
        }
        let version = cairo_version();
        let msg_len = snprintf(core::ptr::null_mut(), 0, "Cairo version is: %d\n\0".as_ptr() as _, version);
        let buf: *mut c_char = malloc(msg_len as _) as _;
        defer!(free(buf as _));
        snprintf(buf, msg_len as _, "Cairo version is: %d\n\0".as_ptr() as _, version);
        printf("%s\n\0".as_ptr() as _, buf);
    }
    0
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

