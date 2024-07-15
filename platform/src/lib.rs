use roc_std::{RocBox, RocResult, RocStr};
use std::{
    ffi::c_void,
    mem::{self, ManuallyDrop},
};

pub mod glue;
pub mod runtime;

pub type Model = RocBox<c_void>;
pub type Message = RocBox<c_void>;

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let (program, settings) = program();

    match runtime::run(program, settings) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("ERROR: {err}");
            1
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    model: ManuallyDrop<Model>,
}

pub fn program() -> (Program, glue::Settings) {
    extern "C" {
        fn roc__mainForHost_1_exposed() -> glue::Init;
    }

    let glue::Init { model, settings } = unsafe { roc__mainForHost_1_exposed() };

    (
        Program {
            model: ManuallyDrop::new(model),
        },
        settings,
    )
}

impl Program {
    pub fn update(&mut self, message: Message) {
        extern "C" {
            fn roc__mainForHost_0_caller(
                model: *const Model,
                message: *const Message,
                closure_data: *mut u8,
                output: *mut u8,
            );
            fn roc__mainForHost_0_result_size() -> usize;
            fn roc__mainForHost_1_caller(
                arg: *const u8,
                closure_data: *const u8,
                output: *mut RocResult<Model, ()>,
            );
        }

        let closure_size = unsafe { roc__mainForHost_0_result_size() };
        let mut closure_data = vec![0; closure_size];

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_0_caller(
                &*self.model,
                &message,
                vec![].as_mut_ptr(),
                closure_data.as_mut_ptr(),
            );
            roc__mainForHost_1_caller(vec![].as_ptr(), closure_data.as_ptr(), output.as_mut_ptr());

            // Decremented by Roc
            mem::forget(message);

            let result: Result<_, _> = output.assume_init().into();

            if let Ok(model) = result {
                self.model = ManuallyDrop::new(model);
            }
        }
    }

    pub fn view(&self) -> glue::Element {
        extern "C" {
            fn roc__mainForHost_2_caller(
                model: *const Model,
                closure_data: *mut u8,
                output: *mut glue::Element,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();

        // Inc by 1 since Roc will dec by 1
        let model = self.model.clone();

        unsafe {
            roc__mainForHost_2_caller(&*model, vec![].as_mut_ptr(), output.as_mut_ptr());

            return output.assume_init();
        }
    }
}

#[no_mangle]
pub extern "C" fn roc_fx_println(s: &RocStr) {
    println!("{s}");
}

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(msg: *mut RocStr, tag_id: u32) {
    match tag_id {
        0 => {
            eprintln!("Roc standard library hit a panic: {}", &*msg);
        }
        1 => {
            eprintln!("Application hit a panic: {}", &*msg);
        }
        _ => unreachable!(),
    }
    std::process::exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dbg(loc: *mut RocStr, msg: *mut RocStr, src: *mut RocStr) {
    eprintln!("[{}] {} = {}", &*loc, &*src, &*msg);
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_getppid() -> libc::pid_t {
    libc::getppid()
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_mmap(
    addr: *mut libc::c_void,
    len: libc::size_t,
    prot: libc::c_int,
    flags: libc::c_int,
    fd: libc::c_int,
    offset: libc::off_t,
) -> *mut libc::c_void {
    libc::mmap(addr, len, prot, flags, fd, offset)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_shm_open(
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::mode_t,
) -> libc::c_int {
    libc::shm_open(name, oflag, mode as libc::c_uint)
}
