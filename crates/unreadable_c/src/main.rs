use libc::size_t;
use std::ffi::{c_int, c_void};

type AllocatorFn = extern "C" fn(size_t) -> *mut c_void;
type InteriorFunctionFn = extern "C" fn(c_int, c_int) -> *mut *mut *mut *mut c_int;
type MiddleFunctionFn = extern "C" fn(c_int, c_int, *mut *mut *mut c_int) -> InteriorFunctionFn;
type OuterFunctionFn = extern "C" fn(*mut *mut c_int, AllocatorFn) -> *mut MiddleFunctionFn;

type _OuterFnDifferently =
    extern "C" fn(
        *mut *mut c_int,
        extern "C" fn(size_t) -> *mut c_void,
    ) -> *mut extern "C" fn(
        c_int,
        c_int,
        *mut *mut *mut c_int,
    )
        -> extern "C" fn(c_int, c_int) -> *mut *mut *mut *mut c_int;

extern "C" {
    pub fn call_unreadable(p: OuterFunctionFn);
}

extern "C" fn interior(a: c_int, b: c_int) -> *mut *mut *mut *mut c_int {
    println!("interior: a={}, b={}", a, b);
    std::ptr::null_mut()
}

extern "C" fn middle(a: c_int, b: c_int, xppp: *mut *mut *mut c_int) -> InteriorFunctionFn {
    println!("middle: a={}, b={}, xppp={}", a, b, xppp as usize);
    interior
}

#[unsafe(no_mangle)]
extern "C" fn unreadable(xpp: *mut *mut c_int, alloc: AllocatorFn) -> *mut MiddleFunctionFn {
    let x = unsafe { **xpp };
    let vp = alloc(1);
    println!("outer x: {}, vp: {:?}", x, vp);
    let func_ptr: *mut MiddleFunctionFn = Box::into_raw(Box::new(middle));
    func_ptr
}

fn main() {
    unsafe {
        call_unreadable(unreadable);
    }
}
