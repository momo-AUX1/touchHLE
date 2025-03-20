use std::ffi::{CStr, c_char};
use std::os::raw::c_int;

use sdl2_sys::{SDL_INIT_EVERYTHING, SDL_INIT_GAMECONTROLLER};

fn main() {
}

#[no_mangle]
pub extern "C" fn external_main(
    host_window: *mut sdl2_sys::SDL_Window,
    host_gl_context: sdl2_sys::SDL_GLContext,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    // Force the host GL context to be current.
    unsafe {
        sdl2_sys::SDL_GL_MakeCurrent(host_window, host_gl_context);
        sdl2_sys::SDL_Init(SDL_INIT_GAMECONTROLLER);
        sdl2_sys::SDL_Init(SDL_INIT_EVERYTHING);
        let hint_key = std::ffi::CString::new("SDL_JOYSTICK_ALLOW_BACKGROUND_EVENTS").unwrap();
        let hint_value = std::ffi::CString::new("1").unwrap();
        sdl2_sys::SDL_SetHint(hint_key.as_ptr(), hint_value.as_ptr());
        sdl2::hint::set("SDL_JOYSTICK_HIDAPI", "1");
    }

    let mut args: Vec<String> = unsafe {
        std::slice::from_raw_parts(argv, argc as usize)
            .iter()
            .map(|&arg| {
                CStr::from_ptr(arg)
                    .to_string_lossy()
                    .into_owned()
            })
            .collect()
    };

    println!("external_main called with args: {:?}", args);


    if args.len() >= 3 {
        let override_arg = args[2].clone();
        if !override_arg.is_empty() {
            std::env::set_var("LOCAL_STATE_PATH", &override_arg);
            println!("Set LOCAL_STATE_PATH to: {}", override_arg);
        }

        args.remove(2);

        if args[1] == "" || args[1].is_empty() {
            args.remove(1);
        }
    }

    match touchHLE::main(args.into_iter()) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Error in external_main: {}", e);
            1
        }
    }
}
